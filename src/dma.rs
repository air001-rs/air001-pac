#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register (DMA_ISR)"]
    pub isr: ISR,
    #[doc = "0x04 - DMA interrupt flag clear register (DMA_IFCR)"]
    pub ifcr: IFCR,
    #[doc = "0x08 - DMA channel configuration register (DMA_CCR)"]
    pub ccr1: CCR1,
    #[doc = "0x0c - DMA channel 1 number of data register"]
    pub cndtr1: CNDTR1,
    #[doc = "0x10 - DMA channel 1 peripheral address register"]
    pub cpar1: CPAR1,
    #[doc = "0x14 - DMA channel 1 memory address register"]
    pub cmar1: CMAR1,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - DMA channel configuration register (DMA_CCR)"]
    pub ccr2: CCR2,
    #[doc = "0x20 - DMA channel 2 number of data register"]
    pub cndtr2: CNDTR2,
    #[doc = "0x24 - DMA channel 2 peripheral address register"]
    pub cpar2: CPAR2,
    #[doc = "0x28 - DMA channel 2 memory address register"]
    pub cmar2: CMAR2,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - DMA channel configuration register (DMA_CCR)"]
    pub ccr3: CCR3,
    #[doc = "0x34 - DMA channel 3 number of data register"]
    pub cndtr3: CNDTR3,
    #[doc = "0x38 - DMA channel 3 peripheral address register"]
    pub cpar3: CPAR3,
    #[doc = "0x3c - DMA channel 3 memory address register"]
    pub cmar3: CMAR3,
}
#[doc = "ISR (r) register accessor: DMA interrupt status register (DMA_ISR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "DMA interrupt status register (DMA_ISR)"]
pub mod isr;
#[doc = "IFCR (w) register accessor: DMA interrupt flag clear register (DMA_IFCR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`]
module"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "DMA interrupt flag clear register (DMA_IFCR)"]
pub mod ifcr;
#[doc = "CCR1 (rw) register accessor: DMA channel configuration register (DMA_CCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`]
module"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "DMA channel configuration register (DMA_CCR)"]
pub mod ccr1;
#[doc = "CNDTR1 (rw) register accessor: DMA channel 1 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr1`]
module"]
pub type CNDTR1 = crate::Reg<cndtr1::CNDTR1_SPEC>;
#[doc = "DMA channel 1 number of data register"]
pub mod cndtr1;
#[doc = "CPAR1 (rw) register accessor: DMA channel 1 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar1`]
module"]
pub type CPAR1 = crate::Reg<cpar1::CPAR1_SPEC>;
#[doc = "DMA channel 1 peripheral address register"]
pub mod cpar1;
#[doc = "CMAR1 (rw) register accessor: DMA channel 1 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar1`]
module"]
pub type CMAR1 = crate::Reg<cmar1::CMAR1_SPEC>;
#[doc = "DMA channel 1 memory address register"]
pub mod cmar1;
#[doc = "CCR2 (rw) register accessor: DMA channel configuration register (DMA_CCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`]
module"]
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
#[doc = "DMA channel configuration register (DMA_CCR)"]
pub mod ccr2;
#[doc = "CNDTR2 (rw) register accessor: DMA channel 2 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr2`]
module"]
pub type CNDTR2 = crate::Reg<cndtr2::CNDTR2_SPEC>;
#[doc = "DMA channel 2 number of data register"]
pub mod cndtr2;
#[doc = "CPAR2 (rw) register accessor: DMA channel 2 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar2`]
module"]
pub type CPAR2 = crate::Reg<cpar2::CPAR2_SPEC>;
#[doc = "DMA channel 2 peripheral address register"]
pub mod cpar2;
#[doc = "CMAR2 (rw) register accessor: DMA channel 2 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar2`]
module"]
pub type CMAR2 = crate::Reg<cmar2::CMAR2_SPEC>;
#[doc = "DMA channel 2 memory address register"]
pub mod cmar2;
#[doc = "CCR3 (rw) register accessor: DMA channel configuration register (DMA_CCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3`]
module"]
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
#[doc = "DMA channel configuration register (DMA_CCR)"]
pub mod ccr3;
#[doc = "CNDTR3 (rw) register accessor: DMA channel 3 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr3`]
module"]
pub type CNDTR3 = crate::Reg<cndtr3::CNDTR3_SPEC>;
#[doc = "DMA channel 3 number of data register"]
pub mod cndtr3;
#[doc = "CPAR3 (rw) register accessor: DMA channel 3 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar3`]
module"]
pub type CPAR3 = crate::Reg<cpar3::CPAR3_SPEC>;
#[doc = "DMA channel 3 peripheral address register"]
pub mod cpar3;
#[doc = "CMAR3 (rw) register accessor: DMA channel 3 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar3`]
module"]
pub type CMAR3 = crate::Reg<cmar3::CMAR3_SPEC>;
#[doc = "DMA channel 3 memory address register"]
pub mod cmar3;
