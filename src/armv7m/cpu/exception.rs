use crate::{armv7m::lang::{SpecialRegister, GPR}, flux_support::bv32::BV32};

use super::{Armv7m, CPUMode, Control};

impl Armv7m {
    #[flux_rs::trusted]
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu]) 
            // requires sp_can_handle_exception_entry(cpu)
            ensures self: Armv7m { new_cpu: new_cpu == Armv7m {  
                    sp: sp_post_exception_entry(cpu), 
                    mem: mem_post_exception_entry(int(get_sp(cpu.sp, cpu.mode, cpu.control)), cpu),
                        //int(get_sp(sp_post_exception_entry(cpu), cpu.mode, cpu.control)), cpu),
                    ..cpu 
                }
            }
    )]
    fn push_stack(&mut self) {
        // Assuming 4 byte alignment for now
        // but maybe this is something to revisit
        let frame_size = BV32::from(0x20);
        let frame_ptr = self.get_value_from_special_reg(&SpecialRegister::sp());
        let frame_ptr = (frame_ptr - frame_size); // & !BV32::from(3);
        self.update_special_reg_with_b32(SpecialRegister::sp(), frame_ptr);
        let frame_ptr = frame_ptr.into();
        // MemA[frameptr,4] = R[0];
        // MemA[frameptr+0x4,4] = R[1];
        // MemA[frameptr+0x8,4] = R[2];
        // MemA[frameptr+0xC,4] = R[3];
        // MemA[frameptr+0x10,4] = R[12];
        // MemA[frameptr+0x14,4] = LR;
        // MemA[frameptr+0x18,4] = ReturnAddress(ExceptionType);
        // MemA[frameptr+0x1C,4] = (XPSR<31:10>:frameptralign:XPSR<8:0>);
        let r0 = self.get_value_from_general_reg(&GPR::r0());
        self.mem.write(frame_ptr, r0);
        let r1 = self.get_value_from_general_reg(&GPR::r1());
        self.mem.write(frame_ptr + 0x4, r1);
        let r2 = self.get_value_from_general_reg(&GPR::r2());
        self.mem.write(frame_ptr + 0x8, r2);
        let r3 = self.get_value_from_general_reg(&GPR::r3());
        self.mem.write(frame_ptr + 0xC, r3);
        let r12 = self.get_value_from_general_reg(&GPR::r12());
        self.mem.write(frame_ptr + 0x10, r12);
        let lr = self.get_value_from_special_reg(&SpecialRegister::lr());
        self.mem.write(frame_ptr + 0x14, lr);
        // putting a dummy value for ret addr
        self.mem.write(frame_ptr + 0x18, BV32::from(0));
        // TODO: Real implementation skips bit 9
        let psr = self.get_value_from_special_reg(&SpecialRegister::psr());
        self.mem.write(frame_ptr + 0x1C, psr);
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], u8[@exception_num])
            ensures self: Armv7m { new_cpu: new_cpu == Armv7m { 
                    mode: handler_mode(),
                    control: control_post_exception_entry(old_cpu),
                    psr: psr_post_exception_entry(old_cpu, exception_num),
                    lr: lr_post_exception_entry(old_cpu, old_cpu.control),
                    ..old_cpu
                }
            }
    )]
    fn exception_taken(&mut self, exception_number: u8) {
        // TODO: need to forget r0 - r3, r12 somehow

        // set exception num in psr
        self.psr = (self.psr & !BV32::from(0xff)) | BV32::from(exception_number as u32);

        // set link register
        self.lr = if self.mode_is_handler() {
            // From another exception
            BV32::from(0xFFFF_FFF1)
        } else if self.control.spsel {
            // from process stack
            BV32::from(0xFFFF_FFFD)
        } else {
            // from main stack
            BV32::from(0xFFFF_FFF9)
        };

        // stack = main
        self.mode = CPUMode::Handler;
        self.control.spsel = false;

        // TODO: There are other ops here but I don't think they
        // matter
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu], u8[@exception_num]) 
            // requires sp_can_handle_exception_entry(cpu)
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_exception_entry(cpu, exception_num) }
    )]
    fn exception_entry(&mut self, exception_number: u8) {
        self.push_stack();
        self.exception_taken(exception_number);
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu], BV32[@return_exec])
            // requires 
            //     is_valid_ram_addr(get_sp_from_isr_ret(cpu.sp, return_exec))
            //     &&
            //     is_valid_ram_addr(get_sp_from_isr_ret(cpu.sp, return_exec) + 0x20)
            ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                    mode: thread_mode(),
                    control: Control { spsel: return_exec != bv32(0xFFFF_FFF9), ..cpu.control },
                    general_regs: gprs_post_exception_exit(get_sp_from_isr_ret(cpu.sp, return_exec), cpu),
                    lr: get_mem_addr(get_sp_from_isr_ret(cpu.sp, return_exec) + 0x14, cpu.mem),
                    psr: get_mem_addr(get_sp_from_isr_ret(cpu.sp, return_exec) + 0x1C, cpu.mem),
                    sp: sp_post_exception_exit(cpu.sp, return_exec),
                    ..cpu
                }
                && is_valid_ram_addr(int(get_sp(new_cpu.sp, new_cpu.mode, new_cpu.control)))
            }
    )]
    fn exception_exit(&mut self, return_exec: BV32) {
        let frame_ptr = if return_exec == BV32::from(0xFFFF_FFF9) {
            self.control.spsel = false;
            self.mode = CPUMode::Thread;
            self.sp.sp_main.into()
        } else {
            self.control.spsel = true;
            self.mode = CPUMode::Thread;
            self.sp.sp_process.into()
        };
        // R[0] = MemA[frameptr,4];
        // R[1] = MemA[frameptr+0x4,4];
        // R[2] = MemA[frameptr+0x8,4];
        // R[3] = MemA[frameptr+0xC,4];
        // R[12] = MemA[frameptr+0x10,4];
        // LR = MemA[frameptr+0x14,4];
        // BranchTo(MemA[frameptr+0x18,4]); // UNPREDICTABLE if the new PC not halfword aligned
        // psr = MemA[frameptr+0x1C,4];
        let r0 = self.mem.read(frame_ptr);
        self.update_general_reg_with_b32(GPR::r0(), r0);
        let r1 = self.mem.read(frame_ptr + 0x4);
        self.update_general_reg_with_b32(GPR::r1(), r1);
        let r2 =  self.mem.read(frame_ptr + 0x8);
        self.update_general_reg_with_b32(GPR::r2(), r2);
        let r3 = self.mem.read(frame_ptr + 0xC);
        self.update_general_reg_with_b32(GPR::r3(), r3);
        let r12 = self.mem.read(frame_ptr + 0x10);
        self.update_general_reg_with_b32(GPR::r12(), r12);
        let lr = self.mem.read(frame_ptr + 0x14);
        self.update_special_reg_with_b32(SpecialRegister::lr(), lr);
        let psr = self.mem.read(frame_ptr + 0x1C);
        self.update_special_reg_with_b32(SpecialRegister::psr(), psr);
        if return_exec == BV32::from(0xFFFF_FFF9) {
            self.sp.sp_main = self.sp.sp_main + BV32::from(0x20);
        } else {
            self.sp.sp_process = self.sp.sp_process + BV32::from(0x20);
        };
    }

    #[flux_rs::sig(
        fn (&Armv7m[@cpu], u8[@exception_num]) -> BV32[get_bx_from_exception_num(exception_num, cpu.lr)]
    )]
    fn run_isr(&self, exception_number: u8) -> BV32 {
        if exception_number == 11 && self.lr == BV32::from(0xFFFF_FFF9) {
            BV32::from(0xFFFF_FFFD)
        } else {
            BV32::from(0xFFFF_FFF9)
        }
    }

    // #[flux_rs::trusted]
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu], u8[@exception_num]) 
            // requires sp_can_handle_exception_entry(cpu)
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_exception_exit(cpu, exception_num) }
    )]
    pub fn preempt(
        &mut self,
        exception_number: u8,
    ) {
        // stack
        self.exception_entry(exception_number);
        // call isr
        let ret_value = self.run_isr(exception_number);
        // unstack
        self.exception_exit(ret_value);
    } 


    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu]) 
           requires mode_is_thread_privileged(old_cpu.mode, old_cpu.control) && sp_can_handle_exception_entry(old_cpu)
           ensures self: Armv7m { new_cpu: 
            // sp_main(new_cpu.sp) == sp_main(old_cpu.sp) && get_gpr(r0(), new_cpu) == bv32(10) 
            // bv_add(sp_process(old_cpu.sp), bv32(0x20)) == sp_process(new_cpu.sp)
            // &&
            // bv_add(sp_main(new_cpu.sp), bv32(0x20)) == sp_main(old_cpu.sp)
            get_mem_addr(
                int(sp_main(old_cpu.sp)), new_cpu.mem
            ) == get_gpr(r0(), old_cpu)
            }
    )]
    fn full_circle(&mut self) {
        // executes some kernel logic
        self.movs_imm(GPR::r0(), BV32::from(10));
        self.exception_entry(11);
        // self.preempt(11);
        // process that havocs all state except the main sp and the fact it's in thread mode unprivileged
        // process(armv7m);
        // fake sys call
        // armv7m.preempt(11);
        // end up back here
        // no more instructions for now
    }
}
