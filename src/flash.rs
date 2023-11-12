#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: ACR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Flash key register"]
    pub keyr: KEYR,
    #[doc = "0x0c - Option byte key register"]
    pub optkeyr: OPTKEYR,
    #[doc = "0x10 - Status register"]
    pub sr: SR,
    #[doc = "0x14 - Flash control register"]
    pub cr: CR,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - Flash option register"]
    pub optr: OPTR,
    #[doc = "0x24 - Flash SDK address register"]
    pub sdkr: SDKR,
    _reserved7: [u8; 0x04],
    #[doc = "0x2c - Flash WRP address register"]
    pub wrpr: WRPR,
    _reserved8: [u8; 0x60],
    #[doc = "0x90 - Flash sleep time config register"]
    pub stcr: STCR,
    _reserved9: [u8; 0x6c],
    #[doc = "0x100 - Flash TS0 register"]
    pub ts0: TS0,
    #[doc = "0x104 - Flash TS1 register"]
    pub ts1: TS1,
    #[doc = "0x108 - Flash TS2P register"]
    pub ts2p: TS2P,
    #[doc = "0x10c - Flash TPS3 register"]
    pub tps3: TPS3,
    #[doc = "0x110 - Flash TS3 register"]
    pub ts3: TS3,
    #[doc = "0x114 - Flash PERTPE register"]
    pub pertpe: PERTPE,
    #[doc = "0x118 - Flash SMERTPE register"]
    pub smertpe: SMERTPE,
    #[doc = "0x11c - Flash PRGTPE register"]
    pub prgtpe: PRGTPE,
    #[doc = "0x120 - Flash PRETPE register"]
    pub pretpe: PRETPE,
}
#[doc = "ACR (rw) register accessor: Access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Access control register"]
pub mod acr;
#[doc = "KEYR (w) register accessor: Flash key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr`]
module"]
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
#[doc = "Flash key register"]
pub mod keyr;
#[doc = "OPTKEYR (w) register accessor: Option byte key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr`]
module"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
#[doc = "Option byte key register"]
pub mod optkeyr;
#[doc = "SR (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "CR (rw) register accessor: Flash control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Flash control register"]
pub mod cr;
#[doc = "OPTR (rw) register accessor: Flash option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optr`]
module"]
pub type OPTR = crate::Reg<optr::OPTR_SPEC>;
#[doc = "Flash option register"]
pub mod optr;
#[doc = "SDKR (rw) register accessor: Flash SDK address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdkr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdkr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdkr`]
module"]
pub type SDKR = crate::Reg<sdkr::SDKR_SPEC>;
#[doc = "Flash SDK address register"]
pub mod sdkr;
#[doc = "WRPR (rw) register accessor: Flash WRP address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpr`]
module"]
pub type WRPR = crate::Reg<wrpr::WRPR_SPEC>;
#[doc = "Flash WRP address register"]
pub mod wrpr;
#[doc = "STCR (rw) register accessor: Flash sleep time config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcr`]
module"]
pub type STCR = crate::Reg<stcr::STCR_SPEC>;
#[doc = "Flash sleep time config register"]
pub mod stcr;
#[doc = "TS0 (rw) register accessor: Flash TS0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ts0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ts0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts0`]
module"]
pub type TS0 = crate::Reg<ts0::TS0_SPEC>;
#[doc = "Flash TS0 register"]
pub mod ts0;
#[doc = "TS1 (rw) register accessor: Flash TS1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ts1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ts1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts1`]
module"]
pub type TS1 = crate::Reg<ts1::TS1_SPEC>;
#[doc = "Flash TS1 register"]
pub mod ts1;
#[doc = "TS2P (rw) register accessor: Flash TS2P register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ts2p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ts2p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts2p`]
module"]
pub type TS2P = crate::Reg<ts2p::TS2P_SPEC>;
#[doc = "Flash TS2P register"]
pub mod ts2p;
#[doc = "TPS3 (rw) register accessor: Flash TPS3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tps3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tps3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tps3`]
module"]
pub type TPS3 = crate::Reg<tps3::TPS3_SPEC>;
#[doc = "Flash TPS3 register"]
pub mod tps3;
#[doc = "TS3 (rw) register accessor: Flash TS3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ts3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ts3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts3`]
module"]
pub type TS3 = crate::Reg<ts3::TS3_SPEC>;
#[doc = "Flash TS3 register"]
pub mod ts3;
#[doc = "PERTPE (rw) register accessor: Flash PERTPE register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pertpe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pertpe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pertpe`]
module"]
pub type PERTPE = crate::Reg<pertpe::PERTPE_SPEC>;
#[doc = "Flash PERTPE register"]
pub mod pertpe;
#[doc = "SMERTPE (rw) register accessor: Flash SMERTPE register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smertpe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smertpe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smertpe`]
module"]
pub type SMERTPE = crate::Reg<smertpe::SMERTPE_SPEC>;
#[doc = "Flash SMERTPE register"]
pub mod smertpe;
#[doc = "PRGTPE (rw) register accessor: Flash PRGTPE register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prgtpe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prgtpe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prgtpe`]
module"]
pub type PRGTPE = crate::Reg<prgtpe::PRGTPE_SPEC>;
#[doc = "Flash PRGTPE register"]
pub mod prgtpe;
#[doc = "PRETPE (rw) register accessor: Flash PRETPE register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pretpe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pretpe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pretpe`]
module"]
pub type PRETPE = crate::Reg<pretpe::PRETPE_SPEC>;
#[doc = "Flash PRETPE register"]
pub mod pretpe;
