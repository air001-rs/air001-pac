#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key register (IWDG_KR)"]
    pub kr: KR,
    #[doc = "0x04 - Prescaler register (IWDG_PR)"]
    pub pr: PR,
    #[doc = "0x08 - Reload register (IWDG_RLR)"]
    pub rlr: RLR,
    #[doc = "0x0c - Status register (IWDG_SR)"]
    pub sr: SR,
    #[doc = "0x10 - Window register (IWDG_SR)"]
    pub winr: WINR,
}
#[doc = "KR (w) register accessor: Key register (IWDG_KR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`kr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kr`]
module"]
pub type KR = crate::Reg<kr::KR_SPEC>;
#[doc = "Key register (IWDG_KR)"]
pub mod kr;
#[doc = "PR (rw) register accessor: Prescaler register (IWDG_PR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`]
module"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Prescaler register (IWDG_PR)"]
pub mod pr;
#[doc = "RLR (rw) register accessor: Reload register (IWDG_RLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlr`]
module"]
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
#[doc = "Reload register (IWDG_RLR)"]
pub mod rlr;
#[doc = "SR (r) register accessor: Status register (IWDG_SR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register (IWDG_SR)"]
pub mod sr;
#[doc = "WINR (r) register accessor: Window register (IWDG_SR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`winr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winr`]
module"]
pub type WINR = crate::Reg<winr::WINR_SPEC>;
#[doc = "Window register (IWDG_SR)"]
pub mod winr;
