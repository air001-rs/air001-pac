#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub cr: CR,
    #[doc = "0x04 - Prescaler register"]
    pub pr: PR,
    #[doc = "0x08 - Time register"]
    pub tr: TR,
    #[doc = "0x0c - Data0 register"]
    pub dr0: DR0,
    #[doc = "0x10 - Data1 register"]
    pub dr1: DR1,
    #[doc = "0x14 - Data2 register"]
    pub dr2: DR2,
    #[doc = "0x18 - Data3 register"]
    pub dr3: DR3,
    #[doc = "0x1c - Interrupt register 1"]
    pub ir: IR,
}
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
pub mod cr;
#[doc = "PR (rw) register accessor: Prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`]
module"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Prescaler register"]
pub mod pr;
#[doc = "TR (rw) register accessor: Time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr`]
module"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "Time register"]
pub mod tr;
#[doc = "DR0 (rw) register accessor: Data0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr0`]
module"]
pub type DR0 = crate::Reg<dr0::DR0_SPEC>;
#[doc = "Data0 register"]
pub mod dr0;
#[doc = "DR1 (rw) register accessor: Data1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr1`]
module"]
pub type DR1 = crate::Reg<dr1::DR1_SPEC>;
#[doc = "Data1 register"]
pub mod dr1;
#[doc = "DR2 (rw) register accessor: Data2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr2`]
module"]
pub type DR2 = crate::Reg<dr2::DR2_SPEC>;
#[doc = "Data2 register"]
pub mod dr2;
#[doc = "DR3 (rw) register accessor: Data3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr3`]
module"]
pub type DR3 = crate::Reg<dr3::DR3_SPEC>;
#[doc = "Data3 register"]
pub mod dr3;
#[doc = "IR (rw) register accessor: Interrupt register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`]
module"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Interrupt register 1"]
pub mod ir;
