#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - COMP control and status register"]
    pub csr: CSR,
    #[doc = "0x04 - Comparator Filter register"]
    pub fr: FR,
}
#[doc = "CSR (rw) register accessor: COMP control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "COMP control and status register"]
pub mod csr;
#[doc = "FR (rw) register accessor: Comparator Filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fr`]
module"]
pub type FR = crate::Reg<fr::FR_SPEC>;
#[doc = "Comparator Filter register"]
pub mod fr;
