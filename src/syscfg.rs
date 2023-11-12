#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYSCFG configuration register 1"]
    pub cfgr1: CFGR1,
    _reserved1: [u8; 0x14],
    #[doc = "0x18 - SYSCFG configuration register 2"]
    pub cfgr2: CFGR2,
    #[doc = "0x1c - SYSCFG configuration register 3"]
    pub cfgr3: CFGR3,
}
#[doc = "CFGR1 (rw) register accessor: SYSCFG configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`]
module"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "SYSCFG configuration register 1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: SYSCFG configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "SYSCFG configuration register 2"]
pub mod cfgr2;
#[doc = "CFGR3 (rw) register accessor: SYSCFG configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr3`]
module"]
pub type CFGR3 = crate::Reg<cfgr3::CFGR3_SPEC>;
#[doc = "SYSCFG configuration register 3"]
pub mod cfgr3;
