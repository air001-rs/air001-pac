#[doc = "Register `SR2` reader"]
pub type R = crate::R<SR2_SPEC>;
#[doc = "Field `MSL` reader - Master/slave"]
pub type MSL_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Bus busy"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `TRA` reader - Transmitter/receiver"]
pub type TRA_R = crate::BitReader;
#[doc = "Field `GENCALL` reader - General call address (Slave mode)"]
pub type GENCALL_R = crate::BitReader;
#[doc = "Field `DUALF` reader - Dual flag (Slave mode)"]
pub type DUALF_R = crate::BitReader;
#[doc = "Field `PEC` reader - acket error checking register"]
pub type PEC_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Master/slave"]
    #[inline(always)]
    pub fn msl(&self) -> MSL_R {
        MSL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bus busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter/receiver"]
    #[inline(always)]
    pub fn tra(&self) -> TRA_R {
        TRA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - General call address (Slave mode)"]
    #[inline(always)]
    pub fn gencall(&self) -> GENCALL_R {
        GENCALL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Dual flag (Slave mode)"]
    #[inline(always)]
    pub fn dualf(&self) -> DUALF_R {
        DUALF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - acket error checking register"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for SR2_SPEC {}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}