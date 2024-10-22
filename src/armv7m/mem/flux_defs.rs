use super::mpu;
use super::nvic;
use super::sys_control;
use super::sys_tick;
use super::{
    Memory, Ppb, INTERRUPT_AUXILIARY_CONTROL_REGISTER_END,
    INTERRUPT_AUXILIARY_CONTROL_REGISTER_START, MPU_END, MPU_START, NVIC_END, NVIC_START, PPB_END,
    PPB_START, SW_TRIGGER_INTERRUPT_REG_END, SW_TRIGGER_INTERRUPT_REG_START,
    SYSTEM_CONTROL_BLOCK_END, SYSTEM_CONTROL_BLOCK_START, SYS_TICK_END, SYS_TICK_START,
};

pub mod mpu_defs {
    use super::mpu::{
        Mpu, MPU_CTRL_ADDR, MPU_RASR_A1_ADDR, MPU_RASR_A2_ADDR, MPU_RASR_A3_ADDR, MPU_RASR_ADDR,
        MPU_RBAR_A1_ADDR, MPU_RBAR_A2_ADDR, MPU_RBAR_A3_ADDR, MPU_RBAR_ADDR, MPU_RNR_ADDR,
        MPU_TYPE_ADDR,
    };

    flux_rs::defs! {
        fn is_valid_mpu_read_addr(address: int) -> bool {
            // all address are read
            address == MPU_TYPE_ADDR
                || address == MPU_CTRL_ADDR
                || address == MPU_RNR_ADDR
                || address == MPU_RBAR_ADDR
                || address == MPU_RASR_ADDR
                || address == MPU_RBAR_A1_ADDR
                || address == MPU_RASR_A1_ADDR
                || address == MPU_RBAR_A2_ADDR
                || address == MPU_RASR_A2_ADDR
                || address == MPU_RBAR_A3_ADDR
                || address == MPU_RASR_A3_ADDR
        }

        fn is_valid_mpu_write_addr(address: int) -> bool {
            // all address except MPU_TYPE are write
            address == MPU_CTRL_ADDR
                || address == MPU_RNR_ADDR
                || address == MPU_RBAR_ADDR
                || address == MPU_RASR_ADDR
                || address == MPU_RBAR_A1_ADDR
                || address == MPU_RASR_A1_ADDR
                || address == MPU_RBAR_A2_ADDR
                || address == MPU_RASR_A2_ADDR
                || address == MPU_RBAR_A3_ADDR
                || address == MPU_RASR_A3_ADDR
        }

        fn mpu_addr_into_reg(address: int, mpu: Mpu) -> int {
            if address == MPU_TYPE_ADDR {
                mpu.mpu_type
            } else if address == MPU_CTRL_ADDR {
                mpu.mpu_ctrl
            } else if address == MPU_RNR_ADDR {
                mpu.mpu_rnr
            } else if address == MPU_RBAR_ADDR {
                mpu.mpu_rbar
            } else if address == MPU_RASR_ADDR {
                mpu.mpu_rasr
            } else if address == MPU_RBAR_A1_ADDR {
                mpu.mpu_rbar_a1
            } else if address == MPU_RASR_A1_ADDR {
                mpu.mpu_rasr_a1
            } else if address == MPU_RBAR_A2_ADDR {
                mpu.mpu_rbar_a2
            } else if address == MPU_RASR_A2_ADDR {
                mpu.mpu_rasr_a2
            } else if address == MPU_RBAR_A3_ADDR {
                mpu.mpu_rbar_a3
            } else if address == MPU_RASR_A3_ADDR {
                mpu.mpu_rasr_a3
            } else {
                -1
            }
        }
    }
}

pub mod sys_tick_defs {

    use super::sys_tick::{SysTick, SYST_CALIB_ADDR, SYST_CSR_ADDR, SYST_CVR_ADDR, SYST_RVR_ADDR};

    flux_rs::defs! {

        fn is_valid_sys_tick_read_addr(address: int) -> bool {
            // all addresses are read
            address == SYST_CSR_ADDR ||
            address == SYST_RVR_ADDR ||
            address == SYST_CVR_ADDR ||
            address == SYST_CALIB_ADDR
        }

        fn is_valid_sys_tick_write_addr(address: int) -> bool {
            // all addresses but SYS_CALIB are write
            address == SYST_CSR_ADDR ||
            address == SYST_RVR_ADDR ||
            address == SYST_CVR_ADDR
        }

        fn sys_tick_addr_into_reg(address: int, sys_tick: SysTick) -> int {
            if address == SYST_CSR_ADDR {
                sys_tick.syst_csr
            } else if address == SYST_RVR_ADDR  {
                sys_tick.syst_rvr
            } else if address == SYST_CVR_ADDR {
                sys_tick.syst_cvr
            } else if address == SYST_CALIB_ADDR {
                sys_tick.syst_calib
            } else {
                -1
            }
        }
    }
}

pub mod sys_control_block_defs {
    use super::sys_control::{
        SysControlBlock, AFSR_ADDR, AIRCR_ADDR, BFAR_ADDR, CCR_ADDR, CFSR_ADDR, CPACR_ADDR,
        CPUID_ADDR, DFSR_ADDR, HFSR_ADDR, ICSR_ADDR, MMFAR_ADDR, SCR_ADDR, SHCSR_ADDR, SHPR1_ADDR,
        SHPR2_ADDR, SHPR3_ADDR, VTOR_ADDR,
    };

    flux_rs::defs! {
        fn is_valid_sys_control_block_read_addr(address: int) -> bool {
            // all addresses are read
            address == CPUID_ADDR ||
                address == ICSR_ADDR ||
                address == VTOR_ADDR ||
                address == AIRCR_ADDR ||
                address == SCR_ADDR ||
                address == CCR_ADDR ||
                address == SHPR1_ADDR ||
                address == SHPR2_ADDR ||
                address == SHPR3_ADDR ||
                address == SHCSR_ADDR ||
                address == CFSR_ADDR ||
                address == HFSR_ADDR ||
                address == DFSR_ADDR ||
                address == MMFAR_ADDR ||
                address == BFAR_ADDR ||
                address == AFSR_ADDR ||
                address == CPACR_ADDR
        }

        fn is_valid_sys_control_block_write_addr(address: int) -> bool {
            // all addresses but CPUID are write
            address == ICSR_ADDR ||
                address == VTOR_ADDR ||
                address == AIRCR_ADDR ||
                address == SCR_ADDR ||
                address == CCR_ADDR ||
                address == SHPR1_ADDR ||
                address == SHPR2_ADDR ||
                address == SHPR3_ADDR ||
                address == SHCSR_ADDR ||
                address == CFSR_ADDR ||
                address == HFSR_ADDR ||
                address == DFSR_ADDR ||
                address == MMFAR_ADDR ||
                address == BFAR_ADDR ||
                address == AFSR_ADDR ||
                address == CPACR_ADDR
        }

        fn sys_control_block_addr_into_reg(address: int, sys_control: SysControlBlock) -> int {
            if address == CPUID_ADDR {
                sys_control.cpuid
            } else if address == ICSR_ADDR {
                sys_control.icsr
            } else if address == VTOR_ADDR {
                sys_control.vtor
            } else if address == AIRCR_ADDR {
                sys_control.aircr
            } else if address == SCR_ADDR {
                sys_control.scr
            } else if address == CCR_ADDR {
                sys_control.ccr
            } else if address == SHPR1_ADDR {
                sys_control.shpr1
            } else if address == SHPR2_ADDR {
                sys_control.shpr2
            } else if address == SHPR3_ADDR {
                sys_control.shpr3
            } else if address == SHCSR_ADDR {
                sys_control.shcsr
            } else if address == CFSR_ADDR {
                sys_control.cfsr
            } else if address == HFSR_ADDR {
                sys_control.hfsr
            } else if address == DFSR_ADDR {
                sys_control.dfsr
            } else if address == MMFAR_ADDR {
                sys_control.mmfar
            } else if address == BFAR_ADDR {
                sys_control.bfar
            } else if address == AFSR_ADDR {
                sys_control.afsr
            } else if address == CPACR_ADDR {
                sys_control.cpacr
            } else {
                -1
            }
        }
    }
}

pub mod sys_control_id_reg_defs {
    use super::sys_control::{
        SysControlIDReg, ACTLR_ADDR, CID0_ADDR, CID1_ADDR, CID2_ADDR, CID3_ADDR, ICTR_ADDR,
        PID0_ADDR, PID1_ADDR, PID2_ADDR, PID3_ADDR, PID4_ADDR, PID5_ADDR, PID6_ADDR, PID7_ADDR,
        STIR_ADDR,
    };

    flux_rs::defs! {
            fn is_valid_sys_control_id_reg_read_addr(address: int) -> bool {
                // all but STIR are read
                address == ICTR_ADDR ||
                    address == ACTLR_ADDR ||
                    address == PID4_ADDR ||
                    address == PID5_ADDR ||
                    address == PID6_ADDR ||
                    address == PID7_ADDR ||
                    address == PID0_ADDR ||
                    address == PID1_ADDR ||
                    address == PID2_ADDR ||
                    address == PID3_ADDR ||
                    address == CID0_ADDR ||
                    address == CID1_ADDR ||
                    address == CID2_ADDR ||
                    address == CID3_ADDR
            }

            fn is_valid_sys_control_id_reg_write_addr(address: int) -> bool {
                // only actlr && stir are write
                address == ACTLR_ADDR || address == STIR_ADDR
            }

            fn sys_control_id_reg_addr_into_reg(address: int, sys_control_id: SysControlIDReg) -> int {
                if address == ICTR_ADDR {
                    sys_control_id.ictr
                } else if address == ACTLR_ADDR {
                    sys_control_id.actlr
                } else if address == STIR_ADDR {
                    sys_control_id.stir
                } else if address == PID4_ADDR {
                    sys_control_id.pid4
                } else if address == PID5_ADDR {
                    sys_control_id.pid5
                } else if address == PID6_ADDR {
                    sys_control_id.pid6
                } else if address == PID7_ADDR {
                    sys_control_id.pid7
                } else if address == PID0_ADDR {
                    sys_control_id.pid0
                } else if address == PID1_ADDR {
                    sys_control_id.pid1
                } else if address == PID2_ADDR {
                    sys_control_id.pid2
                } else if address == PID3_ADDR {
                    sys_control_id.pid3
                } else if address == CID0_ADDR {
                    sys_control_id.cid0
                } else if address == CID1_ADDR {
                    sys_control_id.cid1
                } else if address == CID2_ADDR {
                    sys_control_id.cid2
                } else if address == CID3_ADDR {
                    sys_control_id.cid3
                } else {
                    -1
                }
            }
    }
}

pub mod sys_control_space_defs {
    use super::sys_control::{
        SysControlBlock, SysControlIDReg, SysControlSpace, ACTLR_ADDR, AFSR_ADDR, AIRCR_ADDR,
        BFAR_ADDR, CCR_ADDR, CFSR_ADDR, CID0_ADDR, CID1_ADDR, CID2_ADDR, CID3_ADDR, CPACR_ADDR,
        CPUID_ADDR, DFSR_ADDR, HFSR_ADDR, ICSR_ADDR, ICTR_ADDR, MMFAR_ADDR, PID0_ADDR, PID1_ADDR,
        PID2_ADDR, PID3_ADDR, PID4_ADDR, PID5_ADDR, PID6_ADDR, PID7_ADDR, SCR_ADDR, SHCSR_ADDR,
        SHPR1_ADDR, SHPR2_ADDR, SHPR3_ADDR, STIR_ADDR, VTOR_ADDR,
    };
    use super::sys_control_block_defs::*;
    use super::sys_control_block_defs::*;

    flux_rs::defs! {
            fn is_valid_sys_control_space_read_addr(address: int) -> bool {
                is_valid_sys_control_block_read_addr(address) || is_valid_sys_control_id_reg_read_addr(address)
            }

            fn is_valid_sys_control_space_write_addr(address: int) -> bool {
                is_valid_sys_control_block_write_addr(address) || is_valid_sys_control_id_reg_write_addr(address)
            }

            fn sys_control_space_addr_into_reg(address: int, sys_control: SysControlSpace) -> int {
                // SYS ID Regs
                if address == ICTR_ADDR {
                    sys_control.ictr
                } else if address == ACTLR_ADDR {
                    sys_control.actlr
                } else if address == STIR_ADDR {
                    sys_control.stir
                } else if address == PID4_ADDR {
                    sys_control.pid4
                } else if address == PID5_ADDR {
                    sys_control.pid5
                } else if address == PID6_ADDR {
                    sys_control.pid6
                } else if address == PID7_ADDR {
                    sys_control.pid7
                } else if address == PID0_ADDR {
                    sys_control.pid0
                } else if address == PID1_ADDR {
                    sys_control.pid1
                } else if address == PID2_ADDR {
                    sys_control.pid2
                } else if address == PID3_ADDR {
                    sys_control.pid3
                } else if address == CID0_ADDR {
                    sys_control.cid0
                } else if address == CID1_ADDR {
                    sys_control.cid1
                } else if address == CID2_ADDR {
                    sys_control.cid2
                } else if address == CID3_ADDR {
                    sys_control.cid3
                } else
                // SYS CONTROL BLOCK
                if address == CPUID_ADDR {
                    sys_control.cpuid
                } else if address == ICSR_ADDR {
                    sys_control.icsr
                } else if address == VTOR_ADDR {
                    sys_control.vtor
                } else if address == AIRCR_ADDR {
                    sys_control.aircr
                } else if address == SCR_ADDR {
                    sys_control.scr
                } else if address == CCR_ADDR {
                    sys_control.ccr
                } else if address == SHPR1_ADDR {
                    sys_control.shpr1
                } else if address == SHPR2_ADDR {
                    sys_control.shpr2
                } else if address == SHPR3_ADDR {
                    sys_control.shpr3
                } else if address == SHCSR_ADDR {
                    sys_control.shcsr
                } else if address == CFSR_ADDR {
                    sys_control.cfsr
                } else if address == HFSR_ADDR {
                    sys_control.hfsr
                } else if address == DFSR_ADDR {
                    sys_control.dfsr
                } else if address == MMFAR_ADDR {
                    sys_control.mmfar
                } else if address == BFAR_ADDR {
                    sys_control.bfar
                } else if address == AFSR_ADDR {
                    sys_control.afsr
                } else if address == CPACR_ADDR {
                    sys_control.cpacr
                } else {
                    -1
                }
            }
    }
}

pub mod nvic_defs {
    use super::nvic::{
        Nvic, IABR0_ADDR, IABR10_ADDR, IABR11_ADDR, IABR12_ADDR, IABR13_ADDR, IABR14_ADDR,
        IABR15_ADDR, IABR1_ADDR, IABR2_ADDR, IABR3_ADDR, IABR4_ADDR, IABR5_ADDR, IABR6_ADDR,
        IABR7_ADDR, IABR8_ADDR, IABR9_ADDR, ICER0_ADDR, ICER10_ADDR, ICER11_ADDR, ICER12_ADDR,
        ICER13_ADDR, ICER14_ADDR, ICER15_ADDR, ICER1_ADDR, ICER2_ADDR, ICER3_ADDR, ICER4_ADDR,
        ICER5_ADDR, ICER6_ADDR, ICER7_ADDR, ICER8_ADDR, ICER9_ADDR, ICPR0_ADDR, ICPR10_ADDR,
        ICPR11_ADDR, ICPR12_ADDR, ICPR13_ADDR, ICPR14_ADDR, ICPR15_ADDR, ICPR1_ADDR, ICPR2_ADDR,
        ICPR3_ADDR, ICPR4_ADDR, ICPR5_ADDR, ICPR6_ADDR, ICPR7_ADDR, ICPR8_ADDR, ICPR9_ADDR,
        IPR0_ADDR, IPR100_ADDR, IPR101_ADDR, IPR102_ADDR, IPR103_ADDR, IPR104_ADDR, IPR105_ADDR,
        IPR106_ADDR, IPR107_ADDR, IPR108_ADDR, IPR109_ADDR, IPR10_ADDR, IPR110_ADDR, IPR111_ADDR,
        IPR112_ADDR, IPR113_ADDR, IPR114_ADDR, IPR115_ADDR, IPR116_ADDR, IPR117_ADDR, IPR118_ADDR,
        IPR119_ADDR, IPR11_ADDR, IPR120_ADDR, IPR121_ADDR, IPR122_ADDR, IPR123_ADDR, IPR12_ADDR,
        IPR13_ADDR, IPR14_ADDR, IPR15_ADDR, IPR16_ADDR, IPR17_ADDR, IPR18_ADDR, IPR19_ADDR,
        IPR1_ADDR, IPR20_ADDR, IPR21_ADDR, IPR22_ADDR, IPR23_ADDR, IPR24_ADDR, IPR25_ADDR,
        IPR26_ADDR, IPR27_ADDR, IPR28_ADDR, IPR29_ADDR, IPR2_ADDR, IPR30_ADDR, IPR31_ADDR,
        IPR32_ADDR, IPR33_ADDR, IPR34_ADDR, IPR35_ADDR, IPR36_ADDR, IPR37_ADDR, IPR38_ADDR,
        IPR39_ADDR, IPR3_ADDR, IPR40_ADDR, IPR41_ADDR, IPR42_ADDR, IPR43_ADDR, IPR44_ADDR,
        IPR45_ADDR, IPR46_ADDR, IPR47_ADDR, IPR48_ADDR, IPR49_ADDR, IPR4_ADDR, IPR50_ADDR,
        IPR51_ADDR, IPR52_ADDR, IPR53_ADDR, IPR54_ADDR, IPR55_ADDR, IPR56_ADDR, IPR57_ADDR,
        IPR58_ADDR, IPR59_ADDR, IPR5_ADDR, IPR60_ADDR, IPR61_ADDR, IPR62_ADDR, IPR63_ADDR,
        IPR64_ADDR, IPR65_ADDR, IPR66_ADDR, IPR67_ADDR, IPR68_ADDR, IPR69_ADDR, IPR6_ADDR,
        IPR70_ADDR, IPR71_ADDR, IPR72_ADDR, IPR73_ADDR, IPR74_ADDR, IPR75_ADDR, IPR76_ADDR,
        IPR77_ADDR, IPR78_ADDR, IPR79_ADDR, IPR7_ADDR, IPR80_ADDR, IPR81_ADDR, IPR82_ADDR,
        IPR83_ADDR, IPR84_ADDR, IPR85_ADDR, IPR86_ADDR, IPR87_ADDR, IPR88_ADDR, IPR89_ADDR,
        IPR8_ADDR, IPR90_ADDR, IPR91_ADDR, IPR92_ADDR, IPR93_ADDR, IPR94_ADDR, IPR95_ADDR,
        IPR96_ADDR, IPR97_ADDR, IPR98_ADDR, IPR99_ADDR, IPR9_ADDR, ISER0_ADDR, ISER10_ADDR,
        ISER11_ADDR, ISER12_ADDR, ISER13_ADDR, ISER14_ADDR, ISER15_ADDR, ISER1_ADDR, ISER2_ADDR,
        ISER3_ADDR, ISER4_ADDR, ISER5_ADDR, ISER6_ADDR, ISER7_ADDR, ISER8_ADDR, ISER9_ADDR,
        ISPR0_ADDR, ISPR10_ADDR, ISPR11_ADDR, ISPR12_ADDR, ISPR13_ADDR, ISPR14_ADDR, ISPR15_ADDR,
        ISPR1_ADDR, ISPR2_ADDR, ISPR3_ADDR, ISPR4_ADDR, ISPR5_ADDR, ISPR6_ADDR, ISPR7_ADDR,
        ISPR8_ADDR, ISPR9_ADDR,
    };

    flux_rs::defs! {
            // all addresses are read / write
            fn is_valid_nvic_addr(address: int) -> bool {
                address == ISER0_ADDR ||
                address == ISER1_ADDR ||
                address == ISER2_ADDR ||
                address == ISER3_ADDR ||
                address == ISER4_ADDR ||
                address == ISER5_ADDR ||
                address == ISER6_ADDR ||
                address == ISER7_ADDR ||
                address == ISER8_ADDR ||
                address == ISER9_ADDR ||
                address == ISER10_ADDR ||
                address == ISER11_ADDR ||
                address == ISER12_ADDR ||
                address == ISER13_ADDR ||
                address == ISER14_ADDR ||
                address == ISER15_ADDR ||

                // NVIC_ICER0 - NVIC_ICER15
                address == ICER0_ADDR ||
                address == ICER1_ADDR ||
                address == ICER2_ADDR ||
                address == ICER3_ADDR ||
                address == ICER4_ADDR ||
                address == ICER5_ADDR ||
                address == ICER6_ADDR ||
                address == ICER7_ADDR ||
                address == ICER8_ADDR ||
                address == ICER9_ADDR ||
                address == ICER10_ADDR ||
                address == ICER11_ADDR ||
                address == ICER12_ADDR ||
                address == ICER13_ADDR ||
                address == ICER14_ADDR ||
                address == ICER15_ADDR ||

                // NVIC_ISPR0 - NVIC_ISPR15
                address == ISPR0_ADDR ||
                address == ISPR1_ADDR ||
                address == ISPR2_ADDR ||
                address == ISPR3_ADDR ||
                address == ISPR4_ADDR ||
                address == ISPR5_ADDR ||
                address == ISPR6_ADDR ||
                address == ISPR7_ADDR ||
                address == ISPR8_ADDR ||
                address == ISPR9_ADDR ||
                address == ISPR10_ADDR ||
                address == ISPR11_ADDR ||
                address == ISPR12_ADDR ||
                address == ISPR13_ADDR ||
                address == ISPR14_ADDR ||
                address == ISPR15_ADDR ||

                // NVIC_ICPR0 - NVIC_ICPR15
                address == ICPR0_ADDR ||
                address == ICPR1_ADDR ||
                address == ICPR2_ADDR ||
                address == ICPR3_ADDR ||
                address == ICPR4_ADDR ||
                address == ICPR5_ADDR ||
                address == ICPR6_ADDR ||
                address == ICPR7_ADDR ||
                address == ICPR8_ADDR ||
                address == ICPR9_ADDR ||
                address == ICPR10_ADDR ||
                address == ICPR11_ADDR ||
                address == ICPR12_ADDR ||
                address == ICPR13_ADDR ||
                address == ICPR14_ADDR ||
                address == ICPR15_ADDR ||

                // NVIC_IABR0 - NVIC_IABR15
                address == IABR0_ADDR ||
                address == IABR1_ADDR ||
                address == IABR2_ADDR ||
                address == IABR3_ADDR ||
                address == IABR4_ADDR ||
                address == IABR5_ADDR ||
                address == IABR6_ADDR ||
                address == IABR7_ADDR ||
                address == IABR8_ADDR ||
                address == IABR9_ADDR ||
                address == IABR10_ADDR ||
                address == IABR11_ADDR ||
                address == IABR12_ADDR ||
                address == IABR13_ADDR ||
                address == IABR14_ADDR ||
                address == IABR15_ADDR ||

                // NVIC IPR0 - NVIC_IPR123
                address == IPR0_ADDR ||
                address == IPR1_ADDR ||
                address == IPR2_ADDR ||
                address == IPR3_ADDR ||
                address == IPR4_ADDR ||
                address == IPR5_ADDR ||
                address == IPR6_ADDR ||
                address == IPR7_ADDR ||
                address == IPR8_ADDR ||
                address == IPR9_ADDR ||
                address == IPR10_ADDR ||
                address == IPR11_ADDR ||
                address == IPR12_ADDR ||
                address == IPR13_ADDR ||
                address == IPR14_ADDR ||
                address == IPR15_ADDR ||
                address == IPR16_ADDR ||
                address == IPR17_ADDR ||
                address == IPR18_ADDR ||
                address == IPR19_ADDR ||
                address == IPR20_ADDR ||
                address == IPR21_ADDR ||
                address == IPR22_ADDR ||
                address == IPR23_ADDR ||
                address == IPR24_ADDR ||
                address == IPR25_ADDR ||
                address == IPR26_ADDR ||
                address == IPR27_ADDR ||
                address == IPR28_ADDR ||
                address == IPR29_ADDR ||
                address == IPR30_ADDR ||
                address == IPR31_ADDR ||
                address == IPR32_ADDR ||
                address == IPR33_ADDR ||
                address == IPR34_ADDR ||
                address == IPR35_ADDR ||
                address == IPR36_ADDR ||
                address == IPR37_ADDR ||
                address == IPR38_ADDR ||
                address == IPR39_ADDR ||
                address == IPR40_ADDR ||
                address == IPR41_ADDR ||
                address == IPR42_ADDR ||
                address == IPR43_ADDR ||
                address == IPR44_ADDR ||
                address == IPR45_ADDR ||
                address == IPR46_ADDR ||
                address == IPR47_ADDR ||
                address == IPR48_ADDR ||
                address == IPR49_ADDR ||
                address == IPR50_ADDR ||
                address == IPR51_ADDR ||
                address == IPR52_ADDR ||
                address == IPR53_ADDR ||
                address == IPR54_ADDR ||
                address == IPR55_ADDR ||
                address == IPR56_ADDR ||
                address == IPR57_ADDR ||
                address == IPR58_ADDR ||
                address == IPR59_ADDR ||
                address == IPR60_ADDR ||
                address == IPR61_ADDR ||
                address == IPR62_ADDR ||
                address == IPR63_ADDR ||
                address == IPR64_ADDR ||
                address == IPR65_ADDR ||
                address == IPR66_ADDR ||
                address == IPR67_ADDR ||
                address == IPR68_ADDR ||
                address == IPR69_ADDR ||
                address == IPR70_ADDR ||
                address == IPR71_ADDR ||
                address == IPR72_ADDR ||
                address == IPR73_ADDR ||
                address == IPR74_ADDR ||
                address == IPR75_ADDR ||
                address == IPR76_ADDR ||
                address == IPR77_ADDR ||
                address == IPR78_ADDR ||
                address == IPR79_ADDR ||
                address == IPR80_ADDR ||
                address == IPR81_ADDR ||
                address == IPR82_ADDR ||
                address == IPR83_ADDR ||
                address == IPR84_ADDR ||
                address == IPR85_ADDR ||
                address == IPR86_ADDR ||
                address == IPR87_ADDR ||
                address == IPR88_ADDR ||
                address == IPR89_ADDR ||
                address == IPR90_ADDR ||
                address == IPR91_ADDR ||
                address == IPR92_ADDR ||
                address == IPR93_ADDR ||
                address == IPR94_ADDR ||
                address == IPR95_ADDR ||
                address == IPR96_ADDR ||
                address == IPR97_ADDR ||
                address == IPR98_ADDR ||
                address == IPR99_ADDR ||
                address == IPR100_ADDR ||
                address == IPR101_ADDR ||
                address == IPR102_ADDR ||
                address == IPR103_ADDR ||
                address == IPR104_ADDR ||
                address == IPR105_ADDR ||
                address == IPR106_ADDR ||
                address == IPR107_ADDR ||
                address == IPR108_ADDR ||
                address == IPR109_ADDR ||
                address == IPR110_ADDR ||
                address == IPR111_ADDR ||
                address == IPR112_ADDR ||
                address == IPR113_ADDR ||
                address == IPR114_ADDR ||
                address == IPR115_ADDR ||
                address == IPR116_ADDR ||
                address == IPR117_ADDR ||
                address == IPR118_ADDR ||
                address == IPR119_ADDR ||
                address == IPR120_ADDR ||
                address == IPR121_ADDR ||
                address == IPR122_ADDR ||
                address == IPR123_ADDR
            }

            fn is_valid_nvic_read_addr(address: int) -> bool {
                // all read
                is_valid_nvic_addr(address)
            }

            fn is_valid_nvic_write_addr(address: int) -> bool {
                // all write
                is_valid_nvic_addr(address)
            }

            fn nvic_addr_into_reg(address: int, nvic: Nvic) -> int {
                if address == ISER0_ADDR {
                    nvic.iser0
                } else if address == ISER1_ADDR {
                    nvic.iser1
                } else if address == ISER2_ADDR {
                    nvic.iser2
                } else if address == ISER3_ADDR {
                    nvic.iser3
                } else if address == ISER4_ADDR {
                    nvic.iser4
                } else if address == ISER5_ADDR {
                    nvic.iser5
                } else if address == ISER6_ADDR {
                    nvic.iser6
                } else if address == ISER7_ADDR {
                    nvic.iser7
                } else if address == ISER8_ADDR {
                    nvic.iser8
                } else if address == ISER9_ADDR {
                    nvic.iser9
                } else if address == ISER10_ADDR {
                    nvic.iser10
                } else if address == ISER11_ADDR {
                    nvic.iser11
                } else if address == ISER12_ADDR {
                    nvic.iser12
                } else if address == ISER13_ADDR {
                    nvic.iser13
                } else if address == ISER14_ADDR {
                    nvic.iser14
                } else if address == ISER15_ADDR {
                    nvic.iser15
                } else
                // NVIC_ICER0 - NVIC_ICER15
                if address == ICER0_ADDR {
                    nvic.icer0
                } else if address == ICER1_ADDR {
                    nvic.icer1
                } else if address == ICER2_ADDR {
                    nvic.icer2
                } else if address == ICER3_ADDR {
                    nvic.icer3
                } else if address == ICER4_ADDR {
                    nvic.icer4
                } else if address == ICER5_ADDR {
                    nvic.icer5
                } else if address == ICER6_ADDR {
                    nvic.icer6
                } else if address == ICER7_ADDR {
                    nvic.icer7
                } else if address == ICER8_ADDR {
                    nvic.icer8
                } else if address == ICER9_ADDR {
                    nvic.icer9
                } else if address == ICER10_ADDR {
                    nvic.icer10
                } else if address == ICER11_ADDR {
                    nvic.icer11
                } else if address == ICER12_ADDR {
                    nvic.icer12
                } else if address == ICER13_ADDR {
                    nvic.icer13
                } else if address == ICER14_ADDR {
                    nvic.icer14
                } else if address == ICER15_ADDR {
                    nvic.icer15
                } else
                // NVIC_ISPR0 - NVIC_ISPR15
                if address == ISPR0_ADDR {
                    nvic.ispr0
                } else if address == ISPR1_ADDR {
                    nvic.ispr1
                } else if address == ISPR2_ADDR {
                    nvic.ispr2
                } else if address == ISPR3_ADDR {
                    nvic.ispr3
                } else if address == ISPR4_ADDR {
                    nvic.ispr4
                } else if address == ISPR5_ADDR {
                    nvic.ispr5
                } else if address == ISPR6_ADDR {
                    nvic.ispr6
                } else if address == ISPR7_ADDR {
                    nvic.ispr7
                } else if address == ISPR8_ADDR {
                    nvic.ispr8
                } else if address == ISPR9_ADDR {
                    nvic.ispr9
                } else if address == ISPR10_ADDR {
                    nvic.ispr10
                } else if address == ISPR11_ADDR {
                    nvic.ispr11
                } else if address == ISPR12_ADDR {
                    nvic.ispr12
                } else if address == ISPR13_ADDR {
                    nvic.ispr13
                } else if address == ISPR14_ADDR {
                    nvic.ispr14
                } else if address == ISPR15_ADDR {
                    nvic.ispr15
                } else
                // NVIC_ICPR0 - NVIC_ICPR15
                if address == ICPR0_ADDR {
                    nvic.icpr0
                } else if address == ICPR1_ADDR {
                    nvic.icpr1
                } else if address == ICPR2_ADDR {
                    nvic.icpr2
                } else if address == ICPR3_ADDR {
                    nvic.icpr3
                } else if address == ICPR4_ADDR {
                    nvic.icpr4
                } else if address == ICPR5_ADDR {
                    nvic.icpr5
                } else if address == ICPR6_ADDR {
                    nvic.icpr6
                } else if address == ICPR7_ADDR {
                    nvic.icpr7
                } else if address == ICPR8_ADDR {
                    nvic.icpr8
                } else if address == ICPR9_ADDR {
                    nvic.icpr9
                } else if address == ICPR10_ADDR {
                    nvic.icpr10
                } else if address == ICPR11_ADDR {
                    nvic.icpr11
                } else if address == ICPR12_ADDR {
                    nvic.icpr12
                } else if address == ICPR13_ADDR {
                    nvic.icpr13
                } else if address == ICPR14_ADDR {
                    nvic.icpr14
                } else if address == ICPR15_ADDR {
                    nvic.icpr15
                } else
                // NVIC_IABR0 - NVIC_IABR15
                if address == IABR0_ADDR {
                    nvic.iabr0
                } else if address == IABR1_ADDR {
                    nvic.iabr1
                } else if address == IABR2_ADDR {
                    nvic.iabr2
                } else if address == IABR3_ADDR {
                    nvic.iabr3
                } else if address == IABR4_ADDR {
                    nvic.iabr4
                } else if address == IABR5_ADDR {
                    nvic.iabr5
                } else if address == IABR6_ADDR {
                    nvic.iabr6
                } else if address == IABR7_ADDR {
                    nvic.iabr7
                } else if address == IABR8_ADDR {
                    nvic.iabr8
                } else if address == IABR9_ADDR {
                    nvic.iabr9
                } else if address == IABR10_ADDR {
                    nvic.iabr10
                } else if address == IABR11_ADDR {
                    nvic.iabr11
                } else if address == IABR12_ADDR {
                    nvic.iabr12
                } else if address == IABR13_ADDR {
                    nvic.iabr13
                } else if address == IABR14_ADDR {
                    nvic.iabr14
                } else if address == IABR15_ADDR {
                    nvic.iabr15
                } else
                // NVIC IPR0 - NVIC_IPR123
                if address == IPR0_ADDR {
                    nvic.ipr0
                } else if address == IPR1_ADDR {
                    nvic.ipr1
                } else if address == IPR2_ADDR {
                    nvic.ipr2
                } else if address == IPR3_ADDR {
                    nvic.ipr3
                } else if address == IPR4_ADDR {
                    nvic.ipr4
                } else if address == IPR5_ADDR {
                    nvic.ipr5
                } else if address == IPR6_ADDR {
                    nvic.ipr6
                } else if address == IPR7_ADDR {
                    nvic.ipr7
                } else if address == IPR8_ADDR {
                    nvic.ipr8
                } else if address == IPR9_ADDR {
                    nvic.ipr9
                } else if address == IPR10_ADDR {
                    nvic.ipr10
                } else if address == IPR11_ADDR {
                    nvic.ipr11
                } else if address == IPR12_ADDR {
                    nvic.ipr12
                } else if address == IPR13_ADDR {
                    nvic.ipr13
                } else if address == IPR14_ADDR {
                    nvic.ipr14
                } else if address == IPR15_ADDR {
                    nvic.ipr15
                } else if address == IPR16_ADDR {
                    nvic.ipr16
                } else if address == IPR17_ADDR {
                    nvic.ipr17
                } else if address == IPR18_ADDR {
                    nvic.ipr18
                } else if address == IPR19_ADDR {
                    nvic.ipr19
                } else if address == IPR20_ADDR {
                    nvic.ipr20
                } else if address == IPR21_ADDR {
                    nvic.ipr21
                } else if address == IPR22_ADDR {
                    nvic.ipr22
                } else if address == IPR23_ADDR {
                    nvic.ipr23
                } else if address == IPR24_ADDR {
                    nvic.ipr24
                } else if address == IPR25_ADDR {
                    nvic.ipr25
                } else if address == IPR26_ADDR {
                    nvic.ipr26
                } else if address == IPR27_ADDR {
                    nvic.ipr27
                } else if address == IPR28_ADDR {
                    nvic.ipr28
                } else if address == IPR29_ADDR {
                    nvic.ipr29
                } else if address == IPR30_ADDR {
                    nvic.ipr30
                } else if address == IPR31_ADDR {
                    nvic.ipr31
                } else if address == IPR32_ADDR {
                    nvic.ipr32
                } else if address == IPR33_ADDR {
                    nvic.ipr33
                } else if address == IPR34_ADDR {
                    nvic.ipr34
                } else if address == IPR35_ADDR {
                    nvic.ipr35
                } else if address == IPR36_ADDR {
                    nvic.ipr36
                } else if address == IPR37_ADDR {
                    nvic.ipr37
                } else if address == IPR38_ADDR {
                    nvic.ipr38
                } else if address == IPR39_ADDR {
                    nvic.ipr39
                } else if address == IPR40_ADDR {
                    nvic.ipr40
                } else if address == IPR41_ADDR {
                    nvic.ipr41
                } else if address == IPR42_ADDR {
                    nvic.ipr42
                } else if address == IPR43_ADDR {
                    nvic.ipr43
                } else if address == IPR44_ADDR {
                    nvic.ipr44
                } else if address == IPR45_ADDR {
                    nvic.ipr45
                } else if address == IPR46_ADDR {
                    nvic.ipr46
                } else if address == IPR47_ADDR {
                    nvic.ipr47
                } else if address == IPR48_ADDR {
                    nvic.ipr48
                } else if address == IPR49_ADDR {
                    nvic.ipr49
                } else if address == IPR50_ADDR {
                    nvic.ipr50
                } else if address == IPR51_ADDR {
                    nvic.ipr51
                } else if address == IPR52_ADDR {
                    nvic.ipr52
                } else if address == IPR53_ADDR {
                    nvic.ipr53
                } else if address == IPR54_ADDR {
                    nvic.ipr54
                } else if address == IPR55_ADDR {
                    nvic.ipr55
                } else if address == IPR56_ADDR {
                    nvic.ipr56
                } else if address == IPR57_ADDR {
                    nvic.ipr57
                } else if address == IPR58_ADDR {
                    nvic.ipr58
                } else if address == IPR59_ADDR {
                    nvic.ipr59
                } else if address == IPR60_ADDR {
                    nvic.ipr60
                } else if address == IPR61_ADDR {
                    nvic.ipr61
                } else if address == IPR62_ADDR {
                    nvic.ipr62
                } else if address == IPR63_ADDR {
                    nvic.ipr63
                } else if address == IPR64_ADDR {
                    nvic.ipr64
                } else if address == IPR65_ADDR {
                    nvic.ipr65
                } else if address == IPR66_ADDR {
                    nvic.ipr66
                } else if address == IPR67_ADDR {
                    nvic.ipr67
                } else if address == IPR68_ADDR {
                    nvic.ipr68
                } else if address == IPR69_ADDR {
                    nvic.ipr69
                } else if address == IPR70_ADDR {
                    nvic.ipr70
                } else if address == IPR71_ADDR {
                    nvic.ipr71
                } else if address == IPR72_ADDR {
                    nvic.ipr72
                } else if address == IPR73_ADDR {
                    nvic.ipr73
                } else if address == IPR74_ADDR {
                    nvic.ipr74
                } else if address == IPR75_ADDR {
                    nvic.ipr75
                } else if address == IPR76_ADDR {
                    nvic.ipr76
                } else if address == IPR77_ADDR {
                    nvic.ipr77
                } else if address == IPR78_ADDR {
                    nvic.ipr78
                } else if address == IPR79_ADDR {
                    nvic.ipr79
                } else if address == IPR80_ADDR {
                    nvic.ipr80
                } else if address == IPR81_ADDR {
                    nvic.ipr81
                } else if address == IPR82_ADDR {
                    nvic.ipr82
                } else if address == IPR83_ADDR {
                    nvic.ipr83
                } else if address == IPR84_ADDR {
                    nvic.ipr84
                } else if address == IPR85_ADDR {
                    nvic.ipr85
                } else if address == IPR86_ADDR {
                    nvic.ipr86
                } else if address == IPR87_ADDR {
                    nvic.ipr87
                } else if address == IPR88_ADDR {
                    nvic.ipr88
                } else if address == IPR89_ADDR {
                    nvic.ipr89
                } else if address == IPR90_ADDR {
                    nvic.ipr90
                } else if address == IPR91_ADDR {
                    nvic.ipr91
                } else if address == IPR92_ADDR {
                    nvic.ipr92
                } else if address == IPR93_ADDR {
                    nvic.ipr93
                } else if address == IPR94_ADDR {
                    nvic.ipr94
                } else if address == IPR95_ADDR {
                    nvic.ipr95
                } else if address == IPR96_ADDR {
                    nvic.ipr96
                } else if address == IPR97_ADDR {
                    nvic.ipr97
                } else if address == IPR98_ADDR {
                    nvic.ipr98
                } else if address == IPR99_ADDR {
                    nvic.ipr99
                } else if address == IPR100_ADDR {
                    nvic.ipr100
                } else if address == IPR101_ADDR {
                    nvic.ipr101
                } else if address == IPR102_ADDR {
                    nvic.ipr102
                } else if address == IPR103_ADDR {
                    nvic.ipr103
                } else if address == IPR104_ADDR {
                    nvic.ipr104
                } else if address == IPR105_ADDR {
                    nvic.ipr105
                } else if address == IPR106_ADDR {
                    nvic.ipr106
                } else if address == IPR107_ADDR {
                    nvic.ipr107
                } else if address == IPR108_ADDR {
                    nvic.ipr108
                } else if address == IPR109_ADDR {
                    nvic.ipr109
                } else if address == IPR110_ADDR {
                    nvic.ipr110
                } else if address == IPR111_ADDR {
                    nvic.ipr111
                } else if address == IPR112_ADDR {
                    nvic.ipr112
                } else if address == IPR113_ADDR {
                    nvic.ipr113
                } else if address == IPR114_ADDR {
                    nvic.ipr114
                } else if address == IPR115_ADDR {
                    nvic.ipr115
                } else if address == IPR116_ADDR {
                    nvic.ipr116
                } else if address == IPR117_ADDR {
                    nvic.ipr117
                } else if address == IPR118_ADDR {
                    nvic.ipr118
                } else if address == IPR119_ADDR {
                    nvic.ipr119
                } else if address == IPR120_ADDR {
                    nvic.ipr120
                } else if address == IPR121_ADDR {
                    nvic.ipr121
                } else if address == IPR122_ADDR {
                    nvic.ipr122
                } else if address == IPR123_ADDR {
                    nvic.ipr123
                } else {
                    -1
                }
            }

    }
}

use mpu_defs::*;
use nvic_defs::*;
use sys_control_space_defs::*;
use sys_tick_defs::*;

flux_rs::defs! {
    fn is_valid_read_addr(address: int) -> bool {
        is_valid_sys_control_space_read_addr(address)
        ||
        is_valid_nvic_read_addr(address)
        ||
        is_valid_mpu_read_addr(address)
        ||
        is_valid_sys_tick_read_addr(address)
    }

    fn is_valid_write_addr(address: int) -> bool {
        is_valid_sys_control_space_write_addr(address)
        ||
        is_valid_nvic_write_addr(address)
        ||
        is_valid_mpu_write_addr(address)
        ||
        is_valid_sys_tick_write_addr(address)
    }

    fn mem_addr_into_reg(address: int, mem: Memory) -> int {
        // SYS TICK
        if address == SYST_CSR_ADDR {
            mem.syst_csr
        } else if address == SYST_RVR_ADDR {
            mem.syst_rvr
        } else if address == SYST_CVR_ADDR {
            mem.syst_cvr
        } else if address == SYST_CALIB_ADDR {
            mem.syst_calib
        } else
        // MPU
        if address == MPU_TYPE_ADDR {
            mem.mpu_type
        } else if address == MPU_CTRL_ADDR {
            mem.mpu_ctrl
        } else if address == MPU_RNR_ADDR {
            mem.mpu_rnr
        } else if address == MPU_RBAR_ADDR {
            mem.mpu_rbar
        } else if address == MPU_RASR_ADDR {
            mem.mpu_rasr
        } else if address == MPU_RBAR_A1_ADDR {
            mem.mpu_rbar_a1
        } else if address == MPU_RASR_A1_ADDR {
            mem.mpu_rasr_a1
        } else if address == MPU_RBAR_A2_ADDR {
            mem.mpu_rbar_a2
        } else if address == MPU_RASR_A2_ADDR {
            mem.mpu_rasr_a2
        } else if address == MPU_RBAR_A3_ADDR {
            mem.mpu_rbar_a3
        } else if address == MPU_RASR_A3_ADDR {
            mem.mpu_rasr_a3
        } else
        // SYS ID Regs
        if address == ICTR_ADDR {
            mem.ictr
        } else if address == ACTLR_ADDR {
            mem.actlr
        } else if address == STIR_ADDR {
            mem.stir
        } else if address == PID4_ADDR {
            mem.pid4
        } else if address == PID5_ADDR {
            mem.pid5
        } else if address == PID6_ADDR {
            mem.pid6
        } else if address == PID7_ADDR {
            mem.pid7
        } else if address == PID0_ADDR {
            mem.pid0
        } else if address == PID1_ADDR {
            mem.pid1
        } else if address == PID2_ADDR {
            mem.pid2
        } else if address == PID3_ADDR {
            mem.pid3
        } else if address == CID0_ADDR {
            mem.cid0
        } else if address == CID1_ADDR {
            mem.cid1
        } else if address == CID2_ADDR {
            mem.cid2
        } else if address == CID3_ADDR {
            mem.cid3
        } else
        // SYS CONTROL BLOCK
        if address == CPUID_ADDR {
            mem.cpuid
        } else if address == ICSR_ADDR {
            mem.icsr
        } else if address == VTOR_ADDR {
            mem.vtor
        } else if address == AIRCR_ADDR {
            mem.aircr
        } else if address == SCR_ADDR {
            mem.scr
        } else if address == CCR_ADDR {
            mem.ccr
        } else if address == SHPR1_ADDR {
            mem.shpr1
        } else if address == SHPR2_ADDR {
            mem.shpr2
        } else if address == SHPR3_ADDR {
            mem.shpr3
        } else if address == SHCSR_ADDR {
            mem.shcsr
        } else if address == CFSR_ADDR {
            mem.cfsr
        } else if address == HFSR_ADDR {
            mem.hfsr
        } else if address == DFSR_ADDR {
            mem.dfsr
        } else if address == MMFAR_ADDR {
            mem.mmfar
        } else if address == BFAR_ADDR {
            mem.bfar
        } else if address == AFSR_ADDR {
            mem.afsr
        } else if address == CPACR_ADDR {
            mem.cpacr
        } else
        // NVIC
        if address == ISER0_ADDR {
            mem.iser0
        } else if address == ISER1_ADDR {
            mem.iser1
        } else if address == ISER2_ADDR {
            mem.iser2
        } else if address == ISER3_ADDR {
            mem.iser3
        } else if address == ISER4_ADDR {
            mem.iser4
        } else if address == ISER5_ADDR {
            mem.iser5
        } else if address == ISER6_ADDR {
            mem.iser6
        } else if address == ISER7_ADDR {
            mem.iser7
        } else if address == ISER8_ADDR {
            mem.iser8
        } else if address == ISER9_ADDR {
            mem.iser9
        } else if address == ISER10_ADDR {
            mem.iser10
        } else if address == ISER11_ADDR {
            mem.iser11
        } else if address == ISER12_ADDR {
            mem.iser12
        } else if address == ISER13_ADDR {
            mem.iser13
        } else if address == ISER14_ADDR {
            mem.iser14
        } else if address == ISER15_ADDR {
            mem.iser15
        } else
        // NVIC_ICER0 - NVIC_ICER15
        if address == ICER0_ADDR {
            mem.icer0
        } else if address == ICER1_ADDR {
            mem.icer1
        } else if address == ICER2_ADDR {
            mem.icer2
        } else if address == ICER3_ADDR {
            mem.icer3
        } else if address == ICER4_ADDR {
            mem.icer4
        } else if address == ICER5_ADDR {
            mem.icer5
        } else if address == ICER6_ADDR {
            mem.icer6
        } else if address == ICER7_ADDR {
            mem.icer7
        } else if address == ICER8_ADDR {
            mem.icer8
        } else if address == ICER9_ADDR {
            mem.icer9
        } else if address == ICER10_ADDR {
            mem.icer10
        } else if address == ICER11_ADDR {
            mem.icer11
        } else if address == ICER12_ADDR {
            mem.icer12
        } else if address == ICER13_ADDR {
            mem.icer13
        } else if address == ICER14_ADDR {
            mem.icer14
        } else if address == ICER15_ADDR {
            mem.icer15
        } else
        // NVIC_ISPR0 - NVIC_ISPR15
        if address == ISPR0_ADDR {
            mem.ispr0
        } else if address == ISPR1_ADDR {
            mem.ispr1
        } else if address == ISPR2_ADDR {
            mem.ispr2
        } else if address == ISPR3_ADDR {
            mem.ispr3
        } else if address == ISPR4_ADDR {
            mem.ispr4
        } else if address == ISPR5_ADDR {
            mem.ispr5
        } else if address == ISPR6_ADDR {
            mem.ispr6
        } else if address == ISPR7_ADDR {
            mem.ispr7
        } else if address == ISPR8_ADDR {
            mem.ispr8
        } else if address == ISPR9_ADDR {
            mem.ispr9
        } else if address == ISPR10_ADDR {
            mem.ispr10
        } else if address == ISPR11_ADDR {
            mem.ispr11
        } else if address == ISPR12_ADDR {
            mem.ispr12
        } else if address == ISPR13_ADDR {
            mem.ispr13
        } else if address == ISPR14_ADDR {
            mem.ispr14
        } else if address == ISPR15_ADDR {
            mem.ispr15
        } else
        // NVIC_ICPR0 - NVIC_ICPR15
        if address == ICPR0_ADDR {
            mem.icpr0
        } else if address == ICPR1_ADDR {
            mem.icpr1
        } else if address == ICPR2_ADDR {
            mem.icpr2
        } else if address == ICPR3_ADDR {
            mem.icpr3
        } else if address == ICPR4_ADDR {
            mem.icpr4
        } else if address == ICPR5_ADDR {
            mem.icpr5
        } else if address == ICPR6_ADDR {
            mem.icpr6
        } else if address == ICPR7_ADDR {
            mem.icpr7
        } else if address == ICPR8_ADDR {
            mem.icpr8
        } else if address == ICPR9_ADDR {
            mem.icpr9
        } else if address == ICPR10_ADDR {
            mem.icpr10
        } else if address == ICPR11_ADDR {
            mem.icpr11
        } else if address == ICPR12_ADDR {
            mem.icpr12
        } else if address == ICPR13_ADDR {
            mem.icpr13
        } else if address == ICPR14_ADDR {
            mem.icpr14
        } else if address == ICPR15_ADDR {
            mem.icpr15
        } else
        // NVIC_IABR0 - NVIC_IABR15
        if address == IABR0_ADDR {
            mem.iabr0
        } else if address == IABR1_ADDR {
            mem.iabr1
        } else if address == IABR2_ADDR {
            mem.iabr2
        } else if address == IABR3_ADDR {
            mem.iabr3
        } else if address == IABR4_ADDR {
            mem.iabr4
        } else if address == IABR5_ADDR {
            mem.iabr5
        } else if address == IABR6_ADDR {
            mem.iabr6
        } else if address == IABR7_ADDR {
            mem.iabr7
        } else if address == IABR8_ADDR {
            mem.iabr8
        } else if address == IABR9_ADDR {
            mem.iabr9
        } else if address == IABR10_ADDR {
            mem.iabr10
        } else if address == IABR11_ADDR {
            mem.iabr11
        } else if address == IABR12_ADDR {
            mem.iabr12
        } else if address == IABR13_ADDR {
            mem.iabr13
        } else if address == IABR14_ADDR {
            mem.iabr14
        } else if address == IABR15_ADDR {
            mem.iabr15
        } else
        // NVIC IPR0 - NVIC_IPR123
        if address == IPR0_ADDR {
            mem.ipr0
        } else if address == IPR1_ADDR {
            mem.ipr1
        } else if address == IPR2_ADDR {
            mem.ipr2
        } else if address == IPR3_ADDR {
            mem.ipr3
        } else if address == IPR4_ADDR {
            mem.ipr4
        } else if address == IPR5_ADDR {
            mem.ipr5
        } else if address == IPR6_ADDR {
            mem.ipr6
        } else if address == IPR7_ADDR {
            mem.ipr7
        } else if address == IPR8_ADDR {
            mem.ipr8
        } else if address == IPR9_ADDR {
            mem.ipr9
        } else if address == IPR10_ADDR {
            mem.ipr10
        } else if address == IPR11_ADDR {
            mem.ipr11
        } else if address == IPR12_ADDR {
            mem.ipr12
        } else if address == IPR13_ADDR {
            mem.ipr13
        } else if address == IPR14_ADDR {
            mem.ipr14
        } else if address == IPR15_ADDR {
            mem.ipr15
        } else if address == IPR16_ADDR {
            mem.ipr16
        } else if address == IPR17_ADDR {
            mem.ipr17
        } else if address == IPR18_ADDR {
            mem.ipr18
        } else if address == IPR19_ADDR {
            mem.ipr19
        } else if address == IPR20_ADDR {
            mem.ipr20
        } else if address == IPR21_ADDR {
            mem.ipr21
        } else if address == IPR22_ADDR {
            mem.ipr22
        } else if address == IPR23_ADDR {
            mem.ipr23
        } else if address == IPR24_ADDR {
            mem.ipr24
        } else if address == IPR25_ADDR {
            mem.ipr25
        } else if address == IPR26_ADDR {
            mem.ipr26
        } else if address == IPR27_ADDR {
            mem.ipr27
        } else if address == IPR28_ADDR {
            mem.ipr28
        } else if address == IPR29_ADDR {
            mem.ipr29
        } else if address == IPR30_ADDR {
            mem.ipr30
        } else if address == IPR31_ADDR {
            mem.ipr31
        } else if address == IPR32_ADDR {
            mem.ipr32
        } else if address == IPR33_ADDR {
            mem.ipr33
        } else if address == IPR34_ADDR {
            mem.ipr34
        } else if address == IPR35_ADDR {
            mem.ipr35
        } else if address == IPR36_ADDR {
            mem.ipr36
        } else if address == IPR37_ADDR {
            mem.ipr37
        } else if address == IPR38_ADDR {
            mem.ipr38
        } else if address == IPR39_ADDR {
            mem.ipr39
        } else if address == IPR40_ADDR {
            mem.ipr40
        } else if address == IPR41_ADDR {
            mem.ipr41
        } else if address == IPR42_ADDR {
            mem.ipr42
        } else if address == IPR43_ADDR {
            mem.ipr43
        } else if address == IPR44_ADDR {
            mem.ipr44
        } else if address == IPR45_ADDR {
            mem.ipr45
        } else if address == IPR46_ADDR {
            mem.ipr46
        } else if address == IPR47_ADDR {
            mem.ipr47
        } else if address == IPR48_ADDR {
            mem.ipr48
        } else if address == IPR49_ADDR {
            mem.ipr49
        } else if address == IPR50_ADDR {
            mem.ipr50
        } else if address == IPR51_ADDR {
            mem.ipr51
        } else if address == IPR52_ADDR {
            mem.ipr52
        } else if address == IPR53_ADDR {
            mem.ipr53
        } else if address == IPR54_ADDR {
            mem.ipr54
        } else if address == IPR55_ADDR {
            mem.ipr55
        } else if address == IPR56_ADDR {
            mem.ipr56
        } else if address == IPR57_ADDR {
            mem.ipr57
        } else if address == IPR58_ADDR {
            mem.ipr58
        } else if address == IPR59_ADDR {
            mem.ipr59
        } else if address == IPR60_ADDR {
            mem.ipr60
        } else if address == IPR61_ADDR {
            mem.ipr61
        } else if address == IPR62_ADDR {
            mem.ipr62
        } else if address == IPR63_ADDR {
            mem.ipr63
        } else if address == IPR64_ADDR {
            mem.ipr64
        } else if address == IPR65_ADDR {
            mem.ipr65
        } else if address == IPR66_ADDR {
            mem.ipr66
        } else if address == IPR67_ADDR {
            mem.ipr67
        } else if address == IPR68_ADDR {
            mem.ipr68
        } else if address == IPR69_ADDR {
            mem.ipr69
        } else if address == IPR70_ADDR {
            mem.ipr70
        } else if address == IPR71_ADDR {
            mem.ipr71
        } else if address == IPR72_ADDR {
            mem.ipr72
        } else if address == IPR73_ADDR {
            mem.ipr73
        } else if address == IPR74_ADDR {
            mem.ipr74
        } else if address == IPR75_ADDR {
            mem.ipr75
        } else if address == IPR76_ADDR {
            mem.ipr76
        } else if address == IPR77_ADDR {
            mem.ipr77
        } else if address == IPR78_ADDR {
            mem.ipr78
        } else if address == IPR79_ADDR {
            mem.ipr79
        } else if address == IPR80_ADDR {
            mem.ipr80
        } else if address == IPR81_ADDR {
            mem.ipr81
        } else if address == IPR82_ADDR {
            mem.ipr82
        } else if address == IPR83_ADDR {
            mem.ipr83
        } else if address == IPR84_ADDR {
            mem.ipr84
        } else if address == IPR85_ADDR {
            mem.ipr85
        } else if address == IPR86_ADDR {
            mem.ipr86
        } else if address == IPR87_ADDR {
            mem.ipr87
        } else if address == IPR88_ADDR {
            mem.ipr88
        } else if address == IPR89_ADDR {
            mem.ipr89
        } else if address == IPR90_ADDR {
            mem.ipr90
        } else if address == IPR91_ADDR {
            mem.ipr91
        } else if address == IPR92_ADDR {
            mem.ipr92
        } else if address == IPR93_ADDR {
            mem.ipr93
        } else if address == IPR94_ADDR {
            mem.ipr94
        } else if address == IPR95_ADDR {
            mem.ipr95
        } else if address == IPR96_ADDR {
            mem.ipr96
        } else if address == IPR97_ADDR {
            mem.ipr97
        } else if address == IPR98_ADDR {
            mem.ipr98
        } else if address == IPR99_ADDR {
            mem.ipr99
        } else if address == IPR100_ADDR {
            mem.ipr100
        } else if address == IPR101_ADDR {
            mem.ipr101
        } else if address == IPR102_ADDR {
            mem.ipr102
        } else if address == IPR103_ADDR {
            mem.ipr103
        } else if address == IPR104_ADDR {
            mem.ipr104
        } else if address == IPR105_ADDR {
            mem.ipr105
        } else if address == IPR106_ADDR {
            mem.ipr106
        } else if address == IPR107_ADDR {
            mem.ipr107
        } else if address == IPR108_ADDR {
            mem.ipr108
        } else if address == IPR109_ADDR {
            mem.ipr109
        } else if address == IPR110_ADDR {
            mem.ipr110
        } else if address == IPR111_ADDR {
            mem.ipr111
        } else if address == IPR112_ADDR {
            mem.ipr112
        } else if address == IPR113_ADDR {
            mem.ipr113
        } else if address == IPR114_ADDR {
            mem.ipr114
        } else if address == IPR115_ADDR {
            mem.ipr115
        } else if address == IPR116_ADDR {
            mem.ipr116
        } else if address == IPR117_ADDR {
            mem.ipr117
        } else if address == IPR118_ADDR {
            mem.ipr118
        } else if address == IPR119_ADDR {
            mem.ipr119
        } else if address == IPR120_ADDR {
            mem.ipr120
        } else if address == IPR121_ADDR {
            mem.ipr121
        } else if address == IPR122_ADDR {
            mem.ipr122
        } else if address == IPR123_ADDR {
            mem.ipr123
        } else {
            -1
        }
    }

    fn ppb_addr_into_reg(address: int, ppb: Ppb) -> int {
        // SYS TICK
        if address == SYST_CSR_ADDR {
            ppb.syst_csr
        } else if address == SYST_RVR_ADDR {
            ppb.syst_rvr
        } else if address == SYST_CVR_ADDR {
            ppb.syst_cvr
        } else if address == SYST_CALIB_ADDR {
            ppb.syst_calib
        } else
        // MPU
        if address == MPU_TYPE_ADDR {
            ppb.mpu_type
        } else if address == MPU_CTRL_ADDR {
            ppb.mpu_ctrl
        } else if address == MPU_RNR_ADDR {
            ppb.mpu_rnr
        } else if address == MPU_RBAR_ADDR {
            ppb.mpu_rbar
        } else if address == MPU_RASR_ADDR {
            ppb.mpu_rasr
        } else if address == MPU_RBAR_A1_ADDR {
            ppb.mpu_rbar_a1
        } else if address == MPU_RASR_A1_ADDR {
            ppb.mpu_rasr_a1
        } else if address == MPU_RBAR_A2_ADDR {
            ppb.mpu_rbar_a2
        } else if address == MPU_RASR_A2_ADDR {
            ppb.mpu_rasr_a2
        } else if address == MPU_RBAR_A3_ADDR {
            ppb.mpu_rbar_a3
        } else if address == MPU_RASR_A3_ADDR {
            ppb.mpu_rasr_a3
        } else
        // SYS ID Regs
        if address == ICTR_ADDR {
            ppb.ictr
        } else if address == ACTLR_ADDR {
            ppb.actlr
        } else if address == STIR_ADDR {
            ppb.stir
        } else if address == PID4_ADDR {
            ppb.pid4
        } else if address == PID5_ADDR {
            ppb.pid5
        } else if address == PID6_ADDR {
            ppb.pid6
        } else if address == PID7_ADDR {
            ppb.pid7
        } else if address == PID0_ADDR {
            ppb.pid0
        } else if address == PID1_ADDR {
            ppb.pid1
        } else if address == PID2_ADDR {
            ppb.pid2
        } else if address == PID3_ADDR {
            ppb.pid3
        } else if address == CID0_ADDR {
            ppb.cid0
        } else if address == CID1_ADDR {
            ppb.cid1
        } else if address == CID2_ADDR {
            ppb.cid2
        } else if address == CID3_ADDR {
            ppb.cid3
        } else
        // SYS CONTROL BLOCK
        if address == CPUID_ADDR {
            ppb.cpuid
        } else if address == ICSR_ADDR {
            ppb.icsr
        } else if address == VTOR_ADDR {
            ppb.vtor
        } else if address == AIRCR_ADDR {
            ppb.aircr
        } else if address == SCR_ADDR {
            ppb.scr
        } else if address == CCR_ADDR {
            ppb.ccr
        } else if address == SHPR1_ADDR {
            ppb.shpr1
        } else if address == SHPR2_ADDR {
            ppb.shpr2
        } else if address == SHPR3_ADDR {
            ppb.shpr3
        } else if address == SHCSR_ADDR {
            ppb.shcsr
        } else if address == CFSR_ADDR {
            ppb.cfsr
        } else if address == HFSR_ADDR {
            ppb.hfsr
        } else if address == DFSR_ADDR {
            ppb.dfsr
        } else if address == MMFAR_ADDR {
            ppb.mmfar
        } else if address == BFAR_ADDR {
            ppb.bfar
        } else if address == AFSR_ADDR {
            ppb.afsr
        } else if address == CPACR_ADDR {
            ppb.cpacr
        } else
        // NVIC
        if address == ISER0_ADDR {
            ppb.iser0
        } else if address == ISER1_ADDR {
            ppb.iser1
        } else if address == ISER2_ADDR {
            ppb.iser2
        } else if address == ISER3_ADDR {
            ppb.iser3
        } else if address == ISER4_ADDR {
            ppb.iser4
        } else if address == ISER5_ADDR {
            ppb.iser5
        } else if address == ISER6_ADDR {
            ppb.iser6
        } else if address == ISER7_ADDR {
            ppb.iser7
        } else if address == ISER8_ADDR {
            ppb.iser8
        } else if address == ISER9_ADDR {
            ppb.iser9
        } else if address == ISER10_ADDR {
            ppb.iser10
        } else if address == ISER11_ADDR {
            ppb.iser11
        } else if address == ISER12_ADDR {
            ppb.iser12
        } else if address == ISER13_ADDR {
            ppb.iser13
        } else if address == ISER14_ADDR {
            ppb.iser14
        } else if address == ISER15_ADDR {
            ppb.iser15
        } else
        // NVIC_ICER0 - NVIC_ICER15
        if address == ICER0_ADDR {
            ppb.icer0
        } else if address == ICER1_ADDR {
            ppb.icer1
        } else if address == ICER2_ADDR {
            ppb.icer2
        } else if address == ICER3_ADDR {
            ppb.icer3
        } else if address == ICER4_ADDR {
            ppb.icer4
        } else if address == ICER5_ADDR {
            ppb.icer5
        } else if address == ICER6_ADDR {
            ppb.icer6
        } else if address == ICER7_ADDR {
            ppb.icer7
        } else if address == ICER8_ADDR {
            ppb.icer8
        } else if address == ICER9_ADDR {
            ppb.icer9
        } else if address == ICER10_ADDR {
            ppb.icer10
        } else if address == ICER11_ADDR {
            ppb.icer11
        } else if address == ICER12_ADDR {
            ppb.icer12
        } else if address == ICER13_ADDR {
            ppb.icer13
        } else if address == ICER14_ADDR {
            ppb.icer14
        } else if address == ICER15_ADDR {
            ppb.icer15
        } else
        // NVIC_ISPR0 - NVIC_ISPR15
        if address == ISPR0_ADDR {
            ppb.ispr0
        } else if address == ISPR1_ADDR {
            ppb.ispr1
        } else if address == ISPR2_ADDR {
            ppb.ispr2
        } else if address == ISPR3_ADDR {
            ppb.ispr3
        } else if address == ISPR4_ADDR {
            ppb.ispr4
        } else if address == ISPR5_ADDR {
            ppb.ispr5
        } else if address == ISPR6_ADDR {
            ppb.ispr6
        } else if address == ISPR7_ADDR {
            ppb.ispr7
        } else if address == ISPR8_ADDR {
            ppb.ispr8
        } else if address == ISPR9_ADDR {
            ppb.ispr9
        } else if address == ISPR10_ADDR {
            ppb.ispr10
        } else if address == ISPR11_ADDR {
            ppb.ispr11
        } else if address == ISPR12_ADDR {
            ppb.ispr12
        } else if address == ISPR13_ADDR {
            ppb.ispr13
        } else if address == ISPR14_ADDR {
            ppb.ispr14
        } else if address == ISPR15_ADDR {
            ppb.ispr15
        } else
        // NVIC_ICPR0 - NVIC_ICPR15
        if address == ICPR0_ADDR {
            ppb.icpr0
        } else if address == ICPR1_ADDR {
            ppb.icpr1
        } else if address == ICPR2_ADDR {
            ppb.icpr2
        } else if address == ICPR3_ADDR {
            ppb.icpr3
        } else if address == ICPR4_ADDR {
            ppb.icpr4
        } else if address == ICPR5_ADDR {
            ppb.icpr5
        } else if address == ICPR6_ADDR {
            ppb.icpr6
        } else if address == ICPR7_ADDR {
            ppb.icpr7
        } else if address == ICPR8_ADDR {
            ppb.icpr8
        } else if address == ICPR9_ADDR {
            ppb.icpr9
        } else if address == ICPR10_ADDR {
            ppb.icpr10
        } else if address == ICPR11_ADDR {
            ppb.icpr11
        } else if address == ICPR12_ADDR {
            ppb.icpr12
        } else if address == ICPR13_ADDR {
            ppb.icpr13
        } else if address == ICPR14_ADDR {
            ppb.icpr14
        } else if address == ICPR15_ADDR {
            ppb.icpr15
        } else
        // NVIC_IABR0 - NVIC_IABR15
        if address == IABR0_ADDR {
            ppb.iabr0
        } else if address == IABR1_ADDR {
            ppb.iabr1
        } else if address == IABR2_ADDR {
            ppb.iabr2
        } else if address == IABR3_ADDR {
            ppb.iabr3
        } else if address == IABR4_ADDR {
            ppb.iabr4
        } else if address == IABR5_ADDR {
            ppb.iabr5
        } else if address == IABR6_ADDR {
            ppb.iabr6
        } else if address == IABR7_ADDR {
            ppb.iabr7
        } else if address == IABR8_ADDR {
            ppb.iabr8
        } else if address == IABR9_ADDR {
            ppb.iabr9
        } else if address == IABR10_ADDR {
            ppb.iabr10
        } else if address == IABR11_ADDR {
            ppb.iabr11
        } else if address == IABR12_ADDR {
            ppb.iabr12
        } else if address == IABR13_ADDR {
            ppb.iabr13
        } else if address == IABR14_ADDR {
            ppb.iabr14
        } else if address == IABR15_ADDR {
            ppb.iabr15
        } else
        // NVIC IPR0 - NVIC_IPR123
        if address == IPR0_ADDR {
            ppb.ipr0
        } else if address == IPR1_ADDR {
            ppb.ipr1
        } else if address == IPR2_ADDR {
            ppb.ipr2
        } else if address == IPR3_ADDR {
            ppb.ipr3
        } else if address == IPR4_ADDR {
            ppb.ipr4
        } else if address == IPR5_ADDR {
            ppb.ipr5
        } else if address == IPR6_ADDR {
            ppb.ipr6
        } else if address == IPR7_ADDR {
            ppb.ipr7
        } else if address == IPR8_ADDR {
            ppb.ipr8
        } else if address == IPR9_ADDR {
            ppb.ipr9
        } else if address == IPR10_ADDR {
            ppb.ipr10
        } else if address == IPR11_ADDR {
            ppb.ipr11
        } else if address == IPR12_ADDR {
            ppb.ipr12
        } else if address == IPR13_ADDR {
            ppb.ipr13
        } else if address == IPR14_ADDR {
            ppb.ipr14
        } else if address == IPR15_ADDR {
            ppb.ipr15
        } else if address == IPR16_ADDR {
            ppb.ipr16
        } else if address == IPR17_ADDR {
            ppb.ipr17
        } else if address == IPR18_ADDR {
            ppb.ipr18
        } else if address == IPR19_ADDR {
            ppb.ipr19
        } else if address == IPR20_ADDR {
            ppb.ipr20
        } else if address == IPR21_ADDR {
            ppb.ipr21
        } else if address == IPR22_ADDR {
            ppb.ipr22
        } else if address == IPR23_ADDR {
            ppb.ipr23
        } else if address == IPR24_ADDR {
            ppb.ipr24
        } else if address == IPR25_ADDR {
            ppb.ipr25
        } else if address == IPR26_ADDR {
            ppb.ipr26
        } else if address == IPR27_ADDR {
            ppb.ipr27
        } else if address == IPR28_ADDR {
            ppb.ipr28
        } else if address == IPR29_ADDR {
            ppb.ipr29
        } else if address == IPR30_ADDR {
            ppb.ipr30
        } else if address == IPR31_ADDR {
            ppb.ipr31
        } else if address == IPR32_ADDR {
            ppb.ipr32
        } else if address == IPR33_ADDR {
            ppb.ipr33
        } else if address == IPR34_ADDR {
            ppb.ipr34
        } else if address == IPR35_ADDR {
            ppb.ipr35
        } else if address == IPR36_ADDR {
            ppb.ipr36
        } else if address == IPR37_ADDR {
            ppb.ipr37
        } else if address == IPR38_ADDR {
            ppb.ipr38
        } else if address == IPR39_ADDR {
            ppb.ipr39
        } else if address == IPR40_ADDR {
            ppb.ipr40
        } else if address == IPR41_ADDR {
            ppb.ipr41
        } else if address == IPR42_ADDR {
            ppb.ipr42
        } else if address == IPR43_ADDR {
            ppb.ipr43
        } else if address == IPR44_ADDR {
            ppb.ipr44
        } else if address == IPR45_ADDR {
            ppb.ipr45
        } else if address == IPR46_ADDR {
            ppb.ipr46
        } else if address == IPR47_ADDR {
            ppb.ipr47
        } else if address == IPR48_ADDR {
            ppb.ipr48
        } else if address == IPR49_ADDR {
            ppb.ipr49
        } else if address == IPR50_ADDR {
            ppb.ipr50
        } else if address == IPR51_ADDR {
            ppb.ipr51
        } else if address == IPR52_ADDR {
            ppb.ipr52
        } else if address == IPR53_ADDR {
            ppb.ipr53
        } else if address == IPR54_ADDR {
            ppb.ipr54
        } else if address == IPR55_ADDR {
            ppb.ipr55
        } else if address == IPR56_ADDR {
            ppb.ipr56
        } else if address == IPR57_ADDR {
            ppb.ipr57
        } else if address == IPR58_ADDR {
            ppb.ipr58
        } else if address == IPR59_ADDR {
            ppb.ipr59
        } else if address == IPR60_ADDR {
            ppb.ipr60
        } else if address == IPR61_ADDR {
            ppb.ipr61
        } else if address == IPR62_ADDR {
            ppb.ipr62
        } else if address == IPR63_ADDR {
            ppb.ipr63
        } else if address == IPR64_ADDR {
            ppb.ipr64
        } else if address == IPR65_ADDR {
            ppb.ipr65
        } else if address == IPR66_ADDR {
            ppb.ipr66
        } else if address == IPR67_ADDR {
            ppb.ipr67
        } else if address == IPR68_ADDR {
            ppb.ipr68
        } else if address == IPR69_ADDR {
            ppb.ipr69
        } else if address == IPR70_ADDR {
            ppb.ipr70
        } else if address == IPR71_ADDR {
            ppb.ipr71
        } else if address == IPR72_ADDR {
            ppb.ipr72
        } else if address == IPR73_ADDR {
            ppb.ipr73
        } else if address == IPR74_ADDR {
            ppb.ipr74
        } else if address == IPR75_ADDR {
            ppb.ipr75
        } else if address == IPR76_ADDR {
            ppb.ipr76
        } else if address == IPR77_ADDR {
            ppb.ipr77
        } else if address == IPR78_ADDR {
            ppb.ipr78
        } else if address == IPR79_ADDR {
            ppb.ipr79
        } else if address == IPR80_ADDR {
            ppb.ipr80
        } else if address == IPR81_ADDR {
            ppb.ipr81
        } else if address == IPR82_ADDR {
            ppb.ipr82
        } else if address == IPR83_ADDR {
            ppb.ipr83
        } else if address == IPR84_ADDR {
            ppb.ipr84
        } else if address == IPR85_ADDR {
            ppb.ipr85
        } else if address == IPR86_ADDR {
            ppb.ipr86
        } else if address == IPR87_ADDR {
            ppb.ipr87
        } else if address == IPR88_ADDR {
            ppb.ipr88
        } else if address == IPR89_ADDR {
            ppb.ipr89
        } else if address == IPR90_ADDR {
            ppb.ipr90
        } else if address == IPR91_ADDR {
            ppb.ipr91
        } else if address == IPR92_ADDR {
            ppb.ipr92
        } else if address == IPR93_ADDR {
            ppb.ipr93
        } else if address == IPR94_ADDR {
            ppb.ipr94
        } else if address == IPR95_ADDR {
            ppb.ipr95
        } else if address == IPR96_ADDR {
            ppb.ipr96
        } else if address == IPR97_ADDR {
            ppb.ipr97
        } else if address == IPR98_ADDR {
            ppb.ipr98
        } else if address == IPR99_ADDR {
            ppb.ipr99
        } else if address == IPR100_ADDR {
            ppb.ipr100
        } else if address == IPR101_ADDR {
            ppb.ipr101
        } else if address == IPR102_ADDR {
            ppb.ipr102
        } else if address == IPR103_ADDR {
            ppb.ipr103
        } else if address == IPR104_ADDR {
            ppb.ipr104
        } else if address == IPR105_ADDR {
            ppb.ipr105
        } else if address == IPR106_ADDR {
            ppb.ipr106
        } else if address == IPR107_ADDR {
            ppb.ipr107
        } else if address == IPR108_ADDR {
            ppb.ipr108
        } else if address == IPR109_ADDR {
            ppb.ipr109
        } else if address == IPR110_ADDR {
            ppb.ipr110
        } else if address == IPR111_ADDR {
            ppb.ipr111
        } else if address == IPR112_ADDR {
            ppb.ipr112
        } else if address == IPR113_ADDR {
            ppb.ipr113
        } else if address == IPR114_ADDR {
            ppb.ipr114
        } else if address == IPR115_ADDR {
            ppb.ipr115
        } else if address == IPR116_ADDR {
            ppb.ipr116
        } else if address == IPR117_ADDR {
            ppb.ipr117
        } else if address == IPR118_ADDR {
            ppb.ipr118
        } else if address == IPR119_ADDR {
            ppb.ipr119
        } else if address == IPR120_ADDR {
            ppb.ipr120
        } else if address == IPR121_ADDR {
            ppb.ipr121
        } else if address == IPR122_ADDR {
            ppb.ipr122
        } else if address == IPR123_ADDR {
            ppb.ipr123
        } else {
            -1
        }
    }

}
