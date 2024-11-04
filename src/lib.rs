#![allow(unused)]

pub mod armv7m;
mod flux_support;

mod arm_isr {
    use crate::{armv7m::{cpu::Armv7m, lang::{GPR, IsbOpt, SpecialRegister}}, flux_support::b32::B32};

    flux_rs::defs! {
    
        fn isr_bit_loc(old_cpu: Armv7m) -> B32 {
            bv32((to_int(get_special_reg(ipsr(), old_cpu)) - 16) % 32)
        }
        
        fn isr_r0(old_cpu: Armv7m) -> B32 {
            left_shift(
                bv32(1), 
                isr_bit_loc(old_cpu)
            )
        }

        fn isr_r2(old_cpu: Armv7m) -> B32 {
            bv32((to_int(get_special_reg(ipsr(), old_cpu)) - 16) / 32)
        }

        fn isr_offset(old_cpu: Armv7m) -> int {
            to_int(isr_r2(old_cpu)) * 4
        }
    }

    // Here is disassembly of the armv7m program. Note that the .w specifies "wide"
    // for the 32 bit version of the instruction
    //
    //
    //   0:   f04f 0000       mov.w   r0, #0
    //   4:   f380 8814       msr     CONTROL, r0
    //   8:   f3bf 8f6f       isb     sy
    //   c:   f06f 0e06       mvn.w   lr, #6
    //   10:   f3ef 8005       mrs     r0, IPSR
    //   14:   f000 00ff       and.w   r0, r0, #255    @ 0xff
    //   18:   f1a0 0010       sub.w   r0, r0, #16
    //   1c:   0942            lsrs    r2, r0, #5
    //   1e:   2301            movs    r3, #1
    //   20:   f000 001f       and.w   r0, r0, #31
    //   24:   fa03 f000       lsl.w   r0, r3, r0
    //   28:   4b03            ldr     r3, [pc, #12]   @ (38 <generic_isr_arm_v7m+0x38>)
    //   2a:   f843 0022       str.w   r0, [r3, r2, lsl #2]
    //   2e:   4b03            ldr     r3, [pc, #12]   @ (3c <generic_isr_arm_v7m+0x3c>)
    //   30:   f843 0022       str.w   r0, [r3, r2, lsl #2]
    //   34:   4770            bx      lr
    //   38:   e000e180        .word   0xe000e180
    //   3c:   e000e200        .word   0xe000e200
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu]) 
        // VTOCK TODO:
        //
        // Note we need to say that the IPSR is more than 16
        // This is guaranteed by exception numbers but we should
        // probably formalize that somehow
        requires to_int(get_special_reg(ipsr(), old_cpu)) >= 16 
        ensures self: Armv7m { new_cpu:
            get_gpr(r0(), new_cpu) == isr_r0(old_cpu)
            &&
            get_gpr(r2(), new_cpu) == isr_r2(old_cpu)
            && 
            nth_bit_is_set(
                get_mem_addr(
                    0xe000_e180 + isr_offset(old_cpu),
                    new_cpu.mem
                ),
                isr_bit_loc(old_cpu)
            )
            &&
            nth_bit_is_set(
                get_mem_addr(
                    0xe000_e200 + isr_offset(old_cpu),
                    new_cpu.mem
                ),
                isr_bit_loc(old_cpu)
            )
        }
    )]
    pub fn generic_isr_armv7m(armv7m: &mut Armv7m) {
        // r0 = 0
        armv7m.movw_imm(GPR::R0, B32::from(0));
        // control = r0 = 0
        armv7m.msr(SpecialRegister::Control, GPR::R0);
        // isb
        armv7m.isb(Some(IsbOpt::Sys));
        // NOTE: using pseudo instr here
        // lr = 0xFFFFFFF9
        armv7m.pseudo_ldr_special(SpecialRegister::Lr, B32::from(0xFFFFFFF9));
        // r0 = ipsr
        armv7m.mrs(GPR::R0, SpecialRegister::IPSR);
        // Note: this seems to be a useless instruction?
        armv7m.and_imm(GPR::R0, B32::from(0xff));
        // r0 = ipsr - 16
        armv7m.subw_imm(GPR::R0, GPR::R0, B32::from(16));
        // r2 = r0 >> 5 ---> (ipsr - 16 / 32)
        armv7m.lsrs_imm(GPR::R2, GPR::R0, B32::from(5));
        // r3 = 1
        armv7m.movs_imm(GPR::R3, B32::from(1));
        // r0 = r0 & 31
        armv7m.and_imm(GPR::R0, B32::from(31));
        // r0 = r3 << r0
        //      -     -
        //      1     (ipsr - 16 & 31)
        armv7m.lslw_reg(
            GPR::R0,
            GPR::R3,
            GPR::R0,
        );
        // Note: Ignoring the dissasembled version of this because dealing with program counter is
        // annoying
        //
        // Gonna encode this as a pseudo instruction for now
        armv7m.pseudo_ldr(GPR::R3, B32::from(0xe000_e180));
        // r0 = 1 << (ipsr - 16 & 31)
        // r3 = 0xe000_e180
        // r2 = (ipsr - 16 >> 5) 
        armv7m.strw_lsl_reg(
            GPR::R0,
            GPR::R3,
            GPR::R2,
            B32::from(2),
        );
        // Note: Ignoring the dissasembled version of this because dealing with program counter is
        // annoying
        //
        // Gonna encode this as a pseudo instruction for now
        armv7m.pseudo_ldr(GPR::R3, B32::from(0xe000_e200));
        // r0 = 1 << (ipsr - 16 & 31)
        // r3 = 0xe000_e200
        // r2 = (ipsr - 16 >> 5) << 2
        //
        // mem[0xe000_e200 + ((ipsr - 16 >> 5) << 2)] = (1 << ipsr - 16 & 31) i.e. "bit for the ipsr # is set"
        armv7m.strw_lsl_reg(
            GPR::R0,
            GPR::R3,
            GPR::R2,
            B32::from(2),
        );
        armv7m.bx(SpecialRegister::Lr);
    }
}

mod arm_test {
    use crate::{armv7m::{
        cpu::Armv7m,
        lang::{GPR, SpecialRegister},
        mem::Memory,
    }, flux_support::b32::B32};

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu:
        general_purpose_register_updated(
            r0(), 
            old_cpu,
            new_cpu, 
            bv32(to_int(get_special_reg(ipsr(), old_cpu)) % 32)
        )
    })]
    fn simple_mod(armv7m: &mut Armv7m) {
        // r0 = ipsr
        armv7m.mrs(GPR::R0, SpecialRegister::IPSR);
        // r0 = r0 & 31
        armv7m.and_imm(GPR::R0, B32::from(31));
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: get_gpr(r0(), new_cpu) ==  right_shift(bv32(1 % 32), bv32(5)) })]
    fn simple_shift(armv7m: &mut Armv7m) {
        // r0 = 1
        armv7m.movs_imm(GPR::R0, B32::from(1));
        // r0 = r0 & 31
        armv7m.and_imm(GPR::R0, B32::from(31));
        // r0 = r0 >> 5
        armv7m.lsrs_imm(GPR::R0, GPR::R0, B32::from(5));
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) 
        ensures self: Armv7m { new_cpu: 
            get_gpr(r3(), new_cpu) == bv32(0xE000_E010)
        }
    )]
    fn simple_store(armv7m: &mut Armv7m) {
        armv7m.movw_imm(GPR::R3, B32::from(0xE000_E010));
        armv7m.str_direct(B32::from(1), GPR::R3)
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) 
        ensures self: Armv7m { new_cpu: 
            get_gpr(r3(), new_cpu) == bv32(0xE000_E010)
            &&
            get_special_reg(control(), new_cpu) == bv32(0xE000_E010)
        }
    )]
    fn store_diff_maps(armv7m: &mut Armv7m) {
        armv7m.movw_imm(GPR::R3, B32::from(0xE000_E010));
        armv7m.msr(SpecialRegister::Control, GPR::R3);
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: 
        mem_value_updated(0xE000_E180, old_cpu.mem, new_cpu.mem, bv32(1)) 
    })]
    fn simple_store_nvic(armv7m: &mut Armv7m) {
        armv7m.pseudo_ldr(GPR::R3, B32::from(0xE000_E180));
        armv7m.str_direct(B32::from(1), GPR::R3)
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: 
        mem_value_updated(0xE000_E184, old_cpu.mem, new_cpu.mem, bv32(1)) 
    })]
    fn lsl_store_nvic(armv7m: &mut Armv7m) {
        armv7m.pseudo_ldr(GPR::R3, B32::from(0xE000_E180));
        armv7m.movw_imm(GPR::R0, B32::from(1));
        armv7m.movw_imm(GPR::R1, B32::from(1));
        armv7m.strw_lsl_reg(
            GPR::R0,
            GPR::R3,
            GPR::R1,
            B32::from(2),
        );
    }

    #[flux_rs::should_fail]
    // Sanity check that we the postcondition here specifies the wrong
    // register (should be 0xE000_E184)
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: 
        mem_value_updated(0xE000_E180, old_cpu.mem, new_cpu.mem, bv32(1)) 
    })]
    fn lsl_store_nvic_wrong(armv7m: &mut Armv7m) {
        armv7m.pseudo_ldr(GPR::R3, B32::from(0xE000_E180));
        armv7m.movw_imm(GPR::R0, B32::from(1));
        armv7m.movw_imm(GPR::R1, B32::from(1));
        armv7m.strw_lsl_reg(
            GPR::R0,
            GPR::R3,
            GPR::R1,
            B32::from(2),
        );
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: 
        general_purpose_register_updated(r0(), old_cpu, new_cpu, bv32(0))
    })]
    fn movw_r0(armv7m: &mut Armv7m) {
        armv7m.movw_imm(GPR::R0, B32::from(0));
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: 
        general_purpose_register_updated(r1(), old_cpu, new_cpu, bv32(1))
    })]
    fn movw_r1(armv7m: &mut Armv7m) {
        armv7m.movw_imm(GPR::R1, B32::from(1));
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: 
        get_gpr(r0(), new_cpu) == bv32(0)
        &&
        get_gpr(r1(), new_cpu) == bv32(1)
    })]
    fn two_movs_by_call(armv7m: &mut Armv7m) {
        movw_r0(armv7m);
        movw_r1(armv7m);
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: 
        get_gpr(r0(), new_cpu) == bv32(0)
        &&
        get_gpr(r1(), new_cpu) == bv32(1)
    })]
    fn two_movs(armv7m: &mut Armv7m) {
        armv7m.movw_imm(GPR::R0, B32::from(0));
        armv7m.movw_imm(GPR::R1, B32::from(1));
    }
}
