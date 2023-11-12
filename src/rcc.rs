#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub cr: CR,
    #[doc = "0x04 - Internal clock sources calibration register"]
    pub icscr: ICSCR,
    #[doc = "0x08 - Clock configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x0c - PLL configuration register"]
    pub pllcfgr: PLLCFGR,
    #[doc = "0x10 - External clock source control register"]
    pub ecscr: ECSCR,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - Clock interrupt enable register"]
    pub cier: CIER,
    #[doc = "0x1c - Clock interrupt flag register"]
    pub cifr: CIFR,
    #[doc = "0x20 - Clock interrupt clear register"]
    pub cicr: CICR,
    #[doc = "0x24 - GPIO reset register"]
    pub ioprstr: IOPRSTR,
    #[doc = "0x28 - AHB peripheral reset register"]
    pub ahbrstr: AHBRSTR,
    #[doc = "0x2c - APB peripheral reset register 1"]
    pub apbrstr1: APBRSTR1,
    #[doc = "0x30 - APB peripheral reset register 2"]
    pub apbrstr2: APBRSTR2,
    #[doc = "0x34 - GPIO clock enable register"]
    pub iopenr: IOPENR,
    #[doc = "0x38 - AHB peripheral clock enable register"]
    pub ahbenr: AHBENR,
    #[doc = "0x3c - APB peripheral clock enable register 1"]
    pub apbenr1: APBENR1,
    #[doc = "0x40 - APB peripheral clock enable register 2"]
    pub apbenr2: APBENR2,
    _reserved16: [u8; 0x10],
    #[doc = "0x54 - Peripherals independent clock configuration register"]
    pub ccipr: CCIPR,
    _reserved17: [u8; 0x04],
    #[doc = "0x5c - RTC domain control register"]
    pub bdcr: BDCR,
    #[doc = "0x60 - Control/status register"]
    pub csr: CSR,
}
#[doc = "CR (rw) register accessor: Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Clock control register"]
pub mod cr;
#[doc = "ICSCR (rw) register accessor: Internal clock sources calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icscr`]
module"]
pub type ICSCR = crate::Reg<icscr::ICSCR_SPEC>;
#[doc = "Internal clock sources calibration register"]
pub mod icscr;
#[doc = "CFGR (rw) register accessor: Clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Clock configuration register"]
pub mod cfgr;
#[doc = "PLLCFGR (rw) register accessor: PLL configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcfgr`]
module"]
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGR_SPEC>;
#[doc = "PLL configuration register"]
pub mod pllcfgr;
#[doc = "ECSCR (rw) register accessor: External clock source control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecscr`]
module"]
pub type ECSCR = crate::Reg<ecscr::ECSCR_SPEC>;
#[doc = "External clock source control register"]
pub mod ecscr;
#[doc = "CIER (rw) register accessor: Clock interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cier`]
module"]
pub type CIER = crate::Reg<cier::CIER_SPEC>;
#[doc = "Clock interrupt enable register"]
pub mod cier;
#[doc = "CIFR (r) register accessor: Clock interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cifr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cifr`]
module"]
pub type CIFR = crate::Reg<cifr::CIFR_SPEC>;
#[doc = "Clock interrupt flag register"]
pub mod cifr;
#[doc = "CICR (w) register accessor: Clock interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cicr`]
module"]
pub type CICR = crate::Reg<cicr::CICR_SPEC>;
#[doc = "Clock interrupt clear register"]
pub mod cicr;
#[doc = "IOPRSTR (rw) register accessor: GPIO reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioprstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioprstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioprstr`]
module"]
pub type IOPRSTR = crate::Reg<ioprstr::IOPRSTR_SPEC>;
#[doc = "GPIO reset register"]
pub mod ioprstr;
#[doc = "AHBRSTR (rw) register accessor: AHB peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrstr`]
module"]
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTR_SPEC>;
#[doc = "AHB peripheral reset register"]
pub mod ahbrstr;
#[doc = "APBRSTR1 (rw) register accessor: APB peripheral reset register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbrstr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbrstr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbrstr1`]
module"]
pub type APBRSTR1 = crate::Reg<apbrstr1::APBRSTR1_SPEC>;
#[doc = "APB peripheral reset register 1"]
pub mod apbrstr1;
#[doc = "APBRSTR2 (rw) register accessor: APB peripheral reset register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbrstr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbrstr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbrstr2`]
module"]
pub type APBRSTR2 = crate::Reg<apbrstr2::APBRSTR2_SPEC>;
#[doc = "APB peripheral reset register 2"]
pub mod apbrstr2;
#[doc = "IOPENR (rw) register accessor: GPIO clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iopenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iopenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopenr`]
module"]
pub type IOPENR = crate::Reg<iopenr::IOPENR_SPEC>;
#[doc = "GPIO clock enable register"]
pub mod iopenr;
#[doc = "AHBENR (rw) register accessor: AHB peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbenr`]
module"]
pub type AHBENR = crate::Reg<ahbenr::AHBENR_SPEC>;
#[doc = "AHB peripheral clock enable register"]
pub mod ahbenr;
#[doc = "APBENR1 (rw) register accessor: APB peripheral clock enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbenr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbenr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbenr1`]
module"]
pub type APBENR1 = crate::Reg<apbenr1::APBENR1_SPEC>;
#[doc = "APB peripheral clock enable register 1"]
pub mod apbenr1;
#[doc = "APBENR2 (rw) register accessor: APB peripheral clock enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbenr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbenr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbenr2`]
module"]
pub type APBENR2 = crate::Reg<apbenr2::APBENR2_SPEC>;
#[doc = "APB peripheral clock enable register 2"]
pub mod apbenr2;
#[doc = "CCIPR (rw) register accessor: Peripherals independent clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr`]
module"]
pub type CCIPR = crate::Reg<ccipr::CCIPR_SPEC>;
#[doc = "Peripherals independent clock configuration register"]
pub mod ccipr;
#[doc = "BDCR (rw) register accessor: RTC domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`]
module"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "RTC domain control register"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: Control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Control/status register"]
pub mod csr;
