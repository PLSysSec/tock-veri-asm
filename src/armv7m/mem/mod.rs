// The following file implements memory layout for the ARMv7m architecture.
//
// NOTE: For now, just going to map the PPB - a 1 mb region reserved by the architecture. In the
// future, maybe we can expand this.
//
// Memory types:
//
// - Normal Memory: Can be read or write and is idempotent (see p. A3-80 in the manual)
// - Device Memory: Causes side effects
// - Strongly Ordered Memory: An access to memory marked as Strongly Ordered acts as a memory barrier to all other explicit accesses from that processor, until the point at which the access is complete (that is, has changed the state of the target location or data has been returned). In addition, an access to memory marked as Strongly Ordered must complete before the end of a memory barrier
//
// See here for PPB docs: https://developer.arm.com/documentation/ddi0403/d/System-Level-Architecture/System-Address-Map/System-Control-Space--SCS-?lang=en
//
// System control and ID registers	
// 0xE000E000-0xE000E00F	Includes the Interrupt Controller Type and Auxiliary Control registers
// 0xE000ED00-0xE000ED8F	System control block
// 0xE000EDF0-0xE000EEFF	Debug registers in the SCS
// 0xE000EF00-0xE000EF8F	Includes the SW Trigger Interrupt Register
// 0xE000EF90-0xE000EFCF	implementation defined
// 0xE000EFD0-0xE000EFFF	Microcontroller-specific ID space
//
//
// SysTick	0xE000E010-0xE000E0FF	System Timer, see The system timer, SysTick
// NVIC	0xE000E100-0xE000ECFF	External interrupt controller, see Nested Vectored Interrupt Controller, NVIC
// MPU	0xE000ED90-0xE000EDEF	Memory Protection Unit, see Protected Memory System Architecture, PMSAv7

const PPB_START: u32 = 0xE000_0000;
const PPB_END: u32 = 0xE00F_FFFF;

const INTERRUPT_AUXILIARY_CONTROL_REGISTER_START: u32 = 0xE000_E000;
const  INTERRUPT_AUXILIARY_CONTROL_REGISTER_END: u32 = 0xE000_E00F;

const SYSTEM_CONTROL_BLOCK_START: u32 = 0xE000_ED00;
const SYSTEM_CONTROL_BLOCK_END: u32 = 0xE000_ED8F;

const SW_TRIGGER_INTERRUPT_REG_START: u32 = 0xE000EF00;
const SW_TRIGGER_INTERRUPT_REG_END: u32 = 0xE000EF8F;

const SYS_TICK_START: u32 = 0xE000E010;
const SYS_TICK_END: u32 = 0xE000E0FF;

const NVIC_START: u32 =	0xE000E100;
const NVIC_END: u32 = 0xE000ECFF;

const MPU_START: u32 = 0xE000ED90;
const MPU_END: u32 = 0xE000EDEF;

mod flux_defs;
mod nvic;
mod sys_control;
mod sys_tick;
mod mpu;

use flux_defs::*;
use mpu::Mpu;
use sys_control::SysControlSpace;
use sys_tick::SysTick;
use nvic::Nvic;

#[derive(Debug)]
#[flux_rs::refined_by(
    /* System Control Space Start */
    // sys control id regs
    ictr: int,    
    actlr: int,    
    stir: int,    
    pid4: int,    
    pid5: int,    
    pid6: int,    
    pid7: int,    
    pid0: int,    
    pid1: int,    
    pid2: int,    
    pid3: int,    
    cid0: int,    
    cid1: int,    
    cid2: int,    
    cid3: int,
    // sys control blocks
    cpuid: int,    
    icsr: int,    
    vtor: int,    
    aircr: int,    
    scr: int,    
    ccr: int,    
    shpr1: int,    
    shpr2: int,    
    shpr3: int,    
    shcsr: int,    
    cfsr: int,    
    hfsr: int,    
    dfsr: int,    
    mmfar: int,    
    bfar: int,    
    afsr: int,    
    cpacr: int,
    /* System Control Space End */
    /* Sys Tick Start */
    syst_csr: int,
    syst_rvr: int,
    syst_cvr: int,
    syst_calib: int,
    /* Sys Tick End */
    /* NVIC START */
    iser0: int,
    iser1: int,
    iser2: int,
    iser3: int,
    iser4: int,
    iser5: int,
    iser6: int,
    iser7: int,
    iser8: int,
    iser9: int,
    iser10: int,
    iser11: int,
    iser12: int,
    iser13: int,
    iser14: int,
    iser15: int,
    icer0: int,
    icer1: int,
    icer2: int,
    icer3: int,
    icer4: int,
    icer5: int,
    icer6: int,
    icer7: int,
    icer8: int,
    icer9: int,
    icer10: int,
    icer11: int,
    icer12: int,
    icer13: int,
    icer14: int,
    icer15: int,
    ispr0: int,
    ispr1: int,
    ispr2: int,
    ispr3: int,
    ispr4: int,
    ispr5: int,
    ispr6: int,
    ispr7: int,
    ispr8: int,
    ispr9: int,
    ispr10: int,
    ispr11: int,
    ispr12: int,
    ispr13: int,
    ispr14: int,
    ispr15: int,
    icpr0: int,
    icpr1: int,
    icpr2: int,
    icpr3: int,
    icpr4: int,
    icpr5: int,
    icpr6: int,
    icpr7: int,
    icpr8: int,
    icpr9: int,
    icpr10: int,
    icpr11: int,
    icpr12: int,
    icpr13: int,
    icpr14: int,
    icpr15: int,
    iabr0: int,
    iabr1: int,
    iabr2: int,
    iabr3: int,
    iabr4: int,
    iabr5: int,
    iabr6: int,
    iabr7: int,
    iabr8: int,
    iabr9: int,
    iabr10: int,
    iabr11: int,
    iabr12: int,
    iabr13: int,
    iabr14: int,
    iabr15: int,
    ipr0: int,
    ipr1: int,
    ipr2: int,
    ipr3: int,
    ipr4: int,
    ipr5: int,
    ipr6: int,
    ipr7: int,
    ipr8: int,
    ipr9: int,
    ipr10: int,
    ipr11: int,
    ipr12: int,
    ipr13: int,
    ipr14: int,
    ipr15: int,
    ipr16: int,
    ipr17: int,
    ipr18: int,
    ipr19: int,
    ipr20: int,
    ipr21: int,
    ipr22: int,
    ipr23: int,
    ipr24: int,
    ipr25: int,
    ipr26: int,
    ipr27: int,
    ipr28: int,
    ipr29: int,
    ipr30: int,
    ipr31: int,
    ipr32: int,
    ipr33: int,
    ipr34: int,
    ipr35: int,
    ipr36: int,
    ipr37: int,
    ipr38: int,
    ipr39: int,
    ipr40: int,
    ipr41: int,
    ipr42: int,
    ipr43: int,
    ipr44: int,
    ipr45: int,
    ipr46: int,
    ipr47: int,
    ipr48: int,
    ipr49: int,
    ipr50: int,
    ipr51: int,
    ipr52: int,
    ipr53: int,
    ipr54: int,
    ipr55: int,
    ipr56: int,
    ipr57: int,
    ipr58: int,
    ipr59: int,
    ipr60: int,
    ipr61: int,
    ipr62: int,
    ipr63: int,
    ipr64: int,
    ipr65: int,
    ipr66: int,
    ipr67: int,
    ipr68: int,
    ipr69: int,
    ipr70: int,
    ipr71: int,
    ipr72: int,
    ipr73: int,
    ipr74: int,
    ipr75: int,
    ipr76: int,
    ipr77: int,
    ipr78: int,
    ipr79: int,
    ipr80: int,
    ipr81: int,
    ipr82: int,
    ipr83: int,
    ipr84: int,
    ipr85: int,
    ipr86: int,
    ipr87: int,
    ipr88: int,
    ipr89: int,
    ipr90: int,
    ipr91: int,
    ipr92: int,
    ipr93: int,
    ipr94: int,
    ipr95: int,
    ipr96: int,
    ipr97: int,
    ipr98: int,
    ipr99: int,
    ipr100: int,
    ipr101: int,
    ipr102: int,
    ipr103: int,
    ipr104: int,
    ipr105: int,
    ipr106: int,
    ipr107: int,
    ipr108: int,
    ipr109: int,
    ipr110: int,
    ipr111: int,
    ipr112: int,
    ipr113: int,
    ipr114: int,
    ipr115: int,
    ipr116: int,
    ipr117: int,
    ipr118: int,
    ipr119: int,
    ipr120: int,
    ipr121: int,
    ipr122: int,
    ipr123: int,
    /* NVIC END */
    /* MPU START */
    mpu_type: int,
    mpu_ctrl: int,
    mpu_rnr: int,
    mpu_rbar: int,
    mpu_rasr: int,
    mpu_rbar_a1: int,
    mpu_rasr_a1: int,
    mpu_rbar_a2: int,
    mpu_rasr_a2: int,
    mpu_rbar_a3: int,
    mpu_rasr_a3: int
    /* MPU END */
)]
pub struct Ppb {
    #[field(SysControlSpace[
        ictr,    
        actlr,    
        stir,    
        pid4,    
        pid5,    
        pid6,    
        pid7,    
        pid0,    
        pid1,    
        pid2,    
        pid3,    
        cid0,    
        cid1,    
        cid2,    
        cid3,
        cpuid,    
        icsr,    
        vtor,    
        aircr,    
        scr,    
        ccr,    
        shpr1,    
        shpr2,    
        shpr3,    
        shcsr,    
        cfsr,    
        hfsr,    
        dfsr,    
        mmfar,    
        bfar,    
        afsr,    
        cpacr
    ])]
    system_control_space: SysControlSpace,

    #[field(SysTick[
        syst_csr,
        syst_rvr,
        syst_cvr,
        syst_calib
    ])]
    sys_tick: SysTick,

    #[field(Nvic[
        iser0,
        iser1,
        iser2,
        iser3,
        iser4,
        iser5,
        iser6,
        iser7,
        iser8,
        iser9,
        iser10,
        iser11,
        iser12,
        iser13,
        iser14,
        iser15,
        icer0,
        icer1,
        icer2,
        icer3,
        icer4,
        icer5,
        icer6,
        icer7,
        icer8,
        icer9,
        icer10,
        icer11,
        icer12,
        icer13,
        icer14,
        icer15,
        ispr0,
        ispr1,
        ispr2,
        ispr3,
        ispr4,
        ispr5,
        ispr6,
        ispr7,
        ispr8,
        ispr9,
        ispr10,
        ispr11,
        ispr12,
        ispr13,
        ispr14,
        ispr15,
        icpr0,
        icpr1,
        icpr2,
        icpr3,
        icpr4,
        icpr5,
        icpr6,
        icpr7,
        icpr8,
        icpr9,
        icpr10,
        icpr11,
        icpr12,
        icpr13,
        icpr14,
        icpr15,
        iabr0,
        iabr1,
        iabr2,
        iabr3,
        iabr4,
        iabr5,
        iabr6,
        iabr7,
        iabr8,
        iabr9,
        iabr10,
        iabr11,
        iabr12,
        iabr13,
        iabr14,
        iabr15,
        ipr0,
        ipr1,
        ipr2,
        ipr3,
        ipr4,
        ipr5,
        ipr6,
        ipr7,
        ipr8,
        ipr9,
        ipr10,
        ipr11,
        ipr12,
        ipr13,
        ipr14,
        ipr15,
        ipr16,
        ipr17,
        ipr18,
        ipr19,
        ipr20,
        ipr21,
        ipr22,
        ipr23,
        ipr24,
        ipr25,
        ipr26,
        ipr27,
        ipr28,
        ipr29,
        ipr30,
        ipr31,
        ipr32,
        ipr33,
        ipr34,
        ipr35,
        ipr36,
        ipr37,
        ipr38,
        ipr39,
        ipr40,
        ipr41,
        ipr42,
        ipr43,
        ipr44,
        ipr45,
        ipr46,
        ipr47,
        ipr48,
        ipr49,
        ipr50,
        ipr51,
        ipr52,
        ipr53,
        ipr54,
        ipr55,
        ipr56,
        ipr57,
        ipr58,
        ipr59,
        ipr60,
        ipr61,
        ipr62,
        ipr63,
        ipr64,
        ipr65,
        ipr66,
        ipr67,
        ipr68,
        ipr69,
        ipr70,
        ipr71,
        ipr72,
        ipr73,
        ipr74,
        ipr75,
        ipr76,
        ipr77,
        ipr78,
        ipr79,
        ipr80,
        ipr81,
        ipr82,
        ipr83,
        ipr84,
        ipr85,
        ipr86,
        ipr87,
        ipr88,
        ipr89,
        ipr90,
        ipr91,
        ipr92,
        ipr93,
        ipr94,
        ipr95,
        ipr96,
        ipr97,
        ipr98,
        ipr99,
        ipr100,
        ipr101,
        ipr102,
        ipr103,
        ipr104,
        ipr105,
        ipr106,
        ipr107,
        ipr108,
        ipr109,
        ipr110,
        ipr111,
        ipr112,
        ipr113,
        ipr114,
        ipr115,
        ipr116,
        ipr117,
        ipr118,
        ipr119,
        ipr120,
        ipr121,
        ipr122,
        ipr123,
    ])]
    nvic: Nvic,

    #[field(Mpu[
        mpu_type,
        mpu_ctrl,
        mpu_rnr,
        mpu_rbar,
        mpu_rasr,
        mpu_rbar_a1,
        mpu_rasr_a1,
        mpu_rbar_a2,
        mpu_rasr_a2,
        mpu_rbar_a3,
        mpu_rasr_a3
    ])]
    mpu: Mpu,
}

impl Ppb {
    #[flux_rs::sig(fn (&Memory, u32[@addr]) -> u32 requires in_ppb(addr))]
    pub fn read(&self, address: u32) -> u32 {
        match address {
            INTERRUPT_AUXILIARY_CONTROL_REGISTER_START..=INTERRUPT_AUXILIARY_CONTROL_REGISTER_END
            | SYSTEM_CONTROL_BLOCK_START..=SYSTEM_CONTROL_BLOCK_END
            | SW_TRIGGER_INTERRUPT_REG_START..=SW_TRIGGER_INTERRUPT_REG_END
            => self.system_control_space.read(address),
            SYS_TICK_START..=SYS_TICK_END => self.sys_tick.read(address),
            NVIC_START..=NVIC_END => self.nvic.read(address),
            MPU_START..=MPU_END => self.mpu.read(address),
            // NOTE: Not supporting some of these for now
            0xE000EDF0..=0xE000EEFF => panic!("Read of debug reg (not implemented)"),
            0xE000EF90..=0xE000EFCF => panic!("Read of Implementation defined regs"),
            0xE000EFD0..=0xE000EFFF => panic!("Read of mc specific space"),
            _ => panic!("Read of invalid addr (only system control, sys tick, nvic, and mpun are defined)")
        }
    }

    #[flux_rs::sig(fn (&mut Memory, u32[@addr], u32[@value]) requires in_ppb(addr))]
    pub fn write(&mut self, address: u32, value: u32) {
        match address {
            INTERRUPT_AUXILIARY_CONTROL_REGISTER_START..=INTERRUPT_AUXILIARY_CONTROL_REGISTER_END
            | SYSTEM_CONTROL_BLOCK_START..=SYSTEM_CONTROL_BLOCK_END
            | SW_TRIGGER_INTERRUPT_REG_START..=SW_TRIGGER_INTERRUPT_REG_END => self.system_control_space.write(address, value),
            SYS_TICK_START..=SYS_TICK_END =>  self.sys_tick.write(address, value),
            NVIC_START..=NVIC_END => self.nvic.write(address, value),
            MPU_START..=MPU_END => self.mpu.write(address, value),
            // NOTE: Not supporting some of these for now
            0xE000EF90..=0xE000EFCF => panic!("Write to Implementation defined regs"),
            0xE000EFD0..=0xE000EFFF => panic!("Write to mc specific space"),
            0xE000EDF0..=0xE000EEFF =>  panic!("Write to debug regs (not implemented)"),
            _ => panic!("Write to invalid addr (only system control, sys tick, nvic, and mpun are defined)")
        }
    }
}

#[derive(Debug)]
#[flux_rs::refined_by(
    /* System Control Space Start */
    // sys control id regs
    ictr: int,    
    actlr: int,    
    stir: int,    
    pid4: int,    
    pid5: int,    
    pid6: int,    
    pid7: int,    
    pid0: int,    
    pid1: int,    
    pid2: int,    
    pid3: int,    
    cid0: int,    
    cid1: int,    
    cid2: int,    
    cid3: int,
    // sys control blocks
    cpuid: int,    
    icsr: int,    
    vtor: int,    
    aircr: int,    
    scr: int,    
    ccr: int,    
    shpr1: int,    
    shpr2: int,    
    shpr3: int,    
    shcsr: int,    
    cfsr: int,    
    hfsr: int,    
    dfsr: int,    
    mmfar: int,    
    bfar: int,    
    afsr: int,    
    cpacr: int,
    /* System Control Space End */
    /* Sys Tick Start */
    syst_csr: int,
    syst_rvr: int,
    syst_cvr: int,
    syst_calib: int,
    /* Sys Tick End */
    /* NVIC START */
    iser0: int,
    iser1: int,
    iser2: int,
    iser3: int,
    iser4: int,
    iser5: int,
    iser6: int,
    iser7: int,
    iser8: int,
    iser9: int,
    iser10: int,
    iser11: int,
    iser12: int,
    iser13: int,
    iser14: int,
    iser15: int,
    icer0: int,
    icer1: int,
    icer2: int,
    icer3: int,
    icer4: int,
    icer5: int,
    icer6: int,
    icer7: int,
    icer8: int,
    icer9: int,
    icer10: int,
    icer11: int,
    icer12: int,
    icer13: int,
    icer14: int,
    icer15: int,
    ispr0: int,
    ispr1: int,
    ispr2: int,
    ispr3: int,
    ispr4: int,
    ispr5: int,
    ispr6: int,
    ispr7: int,
    ispr8: int,
    ispr9: int,
    ispr10: int,
    ispr11: int,
    ispr12: int,
    ispr13: int,
    ispr14: int,
    ispr15: int,
    icpr0: int,
    icpr1: int,
    icpr2: int,
    icpr3: int,
    icpr4: int,
    icpr5: int,
    icpr6: int,
    icpr7: int,
    icpr8: int,
    icpr9: int,
    icpr10: int,
    icpr11: int,
    icpr12: int,
    icpr13: int,
    icpr14: int,
    icpr15: int,
    iabr0: int,
    iabr1: int,
    iabr2: int,
    iabr3: int,
    iabr4: int,
    iabr5: int,
    iabr6: int,
    iabr7: int,
    iabr8: int,
    iabr9: int,
    iabr10: int,
    iabr11: int,
    iabr12: int,
    iabr13: int,
    iabr14: int,
    iabr15: int,
    ipr0: int,
    ipr1: int,
    ipr2: int,
    ipr3: int,
    ipr4: int,
    ipr5: int,
    ipr6: int,
    ipr7: int,
    ipr8: int,
    ipr9: int,
    ipr10: int,
    ipr11: int,
    ipr12: int,
    ipr13: int,
    ipr14: int,
    ipr15: int,
    ipr16: int,
    ipr17: int,
    ipr18: int,
    ipr19: int,
    ipr20: int,
    ipr21: int,
    ipr22: int,
    ipr23: int,
    ipr24: int,
    ipr25: int,
    ipr26: int,
    ipr27: int,
    ipr28: int,
    ipr29: int,
    ipr30: int,
    ipr31: int,
    ipr32: int,
    ipr33: int,
    ipr34: int,
    ipr35: int,
    ipr36: int,
    ipr37: int,
    ipr38: int,
    ipr39: int,
    ipr40: int,
    ipr41: int,
    ipr42: int,
    ipr43: int,
    ipr44: int,
    ipr45: int,
    ipr46: int,
    ipr47: int,
    ipr48: int,
    ipr49: int,
    ipr50: int,
    ipr51: int,
    ipr52: int,
    ipr53: int,
    ipr54: int,
    ipr55: int,
    ipr56: int,
    ipr57: int,
    ipr58: int,
    ipr59: int,
    ipr60: int,
    ipr61: int,
    ipr62: int,
    ipr63: int,
    ipr64: int,
    ipr65: int,
    ipr66: int,
    ipr67: int,
    ipr68: int,
    ipr69: int,
    ipr70: int,
    ipr71: int,
    ipr72: int,
    ipr73: int,
    ipr74: int,
    ipr75: int,
    ipr76: int,
    ipr77: int,
    ipr78: int,
    ipr79: int,
    ipr80: int,
    ipr81: int,
    ipr82: int,
    ipr83: int,
    ipr84: int,
    ipr85: int,
    ipr86: int,
    ipr87: int,
    ipr88: int,
    ipr89: int,
    ipr90: int,
    ipr91: int,
    ipr92: int,
    ipr93: int,
    ipr94: int,
    ipr95: int,
    ipr96: int,
    ipr97: int,
    ipr98: int,
    ipr99: int,
    ipr100: int,
    ipr101: int,
    ipr102: int,
    ipr103: int,
    ipr104: int,
    ipr105: int,
    ipr106: int,
    ipr107: int,
    ipr108: int,
    ipr109: int,
    ipr110: int,
    ipr111: int,
    ipr112: int,
    ipr113: int,
    ipr114: int,
    ipr115: int,
    ipr116: int,
    ipr117: int,
    ipr118: int,
    ipr119: int,
    ipr120: int,
    ipr121: int,
    ipr122: int,
    ipr123: int,
    /* NVIC END */
    /* MPU START */
    mpu_type: int,
    mpu_ctrl: int,
    mpu_rnr: int,
    mpu_rbar: int,
    mpu_rasr: int,
    mpu_rbar_a1: int,
    mpu_rasr_a1: int,
    mpu_rbar_a2: int,
    mpu_rasr_a2: int,
    mpu_rbar_a3: int,
    mpu_rasr_a3: int
    /* MPU END */
)]
pub struct Memory {
    #[field(Ppb[
        ictr,    
        actlr,    
        stir,    
        pid4,    
        pid5,    
        pid6,    
        pid7,    
        pid0,    
        pid1,    
        pid2,    
        pid3,    
        cid0,    
        cid1,    
        cid2,    
        cid3,
        cpuid,    
        icsr,    
        vtor,    
        aircr,    
        scr,    
        ccr,    
        shpr1,    
        shpr2,    
        shpr3,    
        shcsr,    
        cfsr,    
        hfsr,    
        dfsr,    
        mmfar,    
        bfar,    
        afsr,    
        cpacr,
        syst_csr,
        syst_rvr,
        syst_cvr,
        syst_calib,
        iser0,
        iser1,
        iser2,
        iser3,
        iser4,
        iser5,
        iser6,
        iser7,
        iser8,
        iser9,
        iser10,
        iser11,
        iser12,
        iser13,
        iser14,
        iser15,
        icer0,
        icer1,
        icer2,
        icer3,
        icer4,
        icer5,
        icer6,
        icer7,
        icer8,
        icer9,
        icer10,
        icer11,
        icer12,
        icer13,
        icer14,
        icer15,
        ispr0,
        ispr1,
        ispr2,
        ispr3,
        ispr4,
        ispr5,
        ispr6,
        ispr7,
        ispr8,
        ispr9,
        ispr10,
        ispr11,
        ispr12,
        ispr13,
        ispr14,
        ispr15,
        icpr0,
        icpr1,
        icpr2,
        icpr3,
        icpr4,
        icpr5,
        icpr6,
        icpr7,
        icpr8,
        icpr9,
        icpr10,
        icpr11,
        icpr12,
        icpr13,
        icpr14,
        icpr15,
        iabr0,
        iabr1,
        iabr2,
        iabr3,
        iabr4,
        iabr5,
        iabr6,
        iabr7,
        iabr8,
        iabr9,
        iabr10,
        iabr11,
        iabr12,
        iabr13,
        iabr14,
        iabr15,
        ipr0,
        ipr1,
        ipr2,
        ipr3,
        ipr4,
        ipr5,
        ipr6,
        ipr7,
        ipr8,
        ipr9,
        ipr10,
        ipr11,
        ipr12,
        ipr13,
        ipr14,
        ipr15,
        ipr16,
        ipr17,
        ipr18,
        ipr19,
        ipr20,
        ipr21,
        ipr22,
        ipr23,
        ipr24,
        ipr25,
        ipr26,
        ipr27,
        ipr28,
        ipr29,
        ipr30,
        ipr31,
        ipr32,
        ipr33,
        ipr34,
        ipr35,
        ipr36,
        ipr37,
        ipr38,
        ipr39,
        ipr40,
        ipr41,
        ipr42,
        ipr43,
        ipr44,
        ipr45,
        ipr46,
        ipr47,
        ipr48,
        ipr49,
        ipr50,
        ipr51,
        ipr52,
        ipr53,
        ipr54,
        ipr55,
        ipr56,
        ipr57,
        ipr58,
        ipr59,
        ipr60,
        ipr61,
        ipr62,
        ipr63,
        ipr64,
        ipr65,
        ipr66,
        ipr67,
        ipr68,
        ipr69,
        ipr70,
        ipr71,
        ipr72,
        ipr73,
        ipr74,
        ipr75,
        ipr76,
        ipr77,
        ipr78,
        ipr79,
        ipr80,
        ipr81,
        ipr82,
        ipr83,
        ipr84,
        ipr85,
        ipr86,
        ipr87,
        ipr88,
        ipr89,
        ipr90,
        ipr91,
        ipr92,
        ipr93,
        ipr94,
        ipr95,
        ipr96,
        ipr97,
        ipr98,
        ipr99,
        ipr100,
        ipr101,
        ipr102,
        ipr103,
        ipr104,
        ipr105,
        ipr106,
        ipr107,
        ipr108,
        ipr109,
        ipr110,
        ipr111,
        ipr112,
        ipr113,
        ipr114,
        ipr115,
        ipr116,
        ipr117,
        ipr118,
        ipr119,
        ipr120,
        ipr121,
        ipr122,
        ipr123,
        mpu_type,
        mpu_ctrl,
        mpu_rnr,
        mpu_rbar,
        mpu_rasr,
        mpu_rbar_a1,
        mpu_rasr_a1,
        mpu_rbar_a2,
        mpu_rasr_a2,
        mpu_rbar_a3,
        mpu_rasr_a3
    ])]
    ppb: Ppb,
}

impl Memory {

    #[flux_rs::sig(fn (&Memory, u32[@addr]) -> u32 requires in_ppb(addr) )]
    pub fn read(&self, address: u32) -> u32 {
        match address {
            0xE000_0000..=0xE00F_FFFF => self.ppb.read(address),
            _ => panic!("Read of unknown memory address (only ppb is defined)")
        }
    }

    #[flux_rs::sig(fn (&mut Memory, u32[@addr]) requires in_ppb(addr))]
    pub fn write(&mut self, address: u32, value: u32) {
        match address {
            0xE000_0000..=0xE00F_FFFF => self.ppb.write(address, value),
            _ => panic!("Write to unknown memory address (only ppb is defined)")
        }
    }
}
