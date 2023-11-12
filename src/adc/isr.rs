#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<ISR_SPEC>;
#[doc = "Field `EOSMP` reader - ADC group regular end of sampling flag"]
pub type EOSMP_R = crate::BitReader;
#[doc = "Field `EOSMP` writer - ADC group regular end of sampling flag"]
pub type EOSMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC` reader - ADC group regular end of unitary conversion flag"]
pub type EOC_R = crate::BitReader;
#[doc = "Field `EOC` writer - ADC group regular end of unitary conversion flag"]
pub type EOC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOSEQ` reader - ADC group regular end of sequence conversions flag"]
pub type EOSEQ_R = crate::BitReader;
#[doc = "Field `EOSEQ` writer - ADC group regular end of sequence conversions flag"]
pub type EOSEQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVR` reader - ADC group regular overrun flag"]
pub type OVR_R = crate::BitReader;
#[doc = "Field `OVR` writer - ADC group regular overrun flag"]
pub type OVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AWD` reader - ADC analog watchdog flag"]
pub type AWD_R = crate::BitReader;
#[doc = "Field `AWD` writer - ADC analog watchdog flag"]
pub type AWD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - ADC group regular end of sampling flag"]
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion flag"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions flag"]
    #[inline(always)]
    pub fn eoseq(&self) -> EOSEQ_R {
        EOSEQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC group regular overrun flag"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC analog watchdog flag"]
    #[inline(always)]
    pub fn awd(&self) -> AWD_R {
        AWD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ADC group regular end of sampling flag"]
    #[inline(always)]
    #[must_use]
    pub fn eosmp(&mut self) -> EOSMP_W<ISR_SPEC, 1> {
        EOSMP_W::new(self)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion flag"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<ISR_SPEC, 2> {
        EOC_W::new(self)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions flag"]
    #[inline(always)]
    #[must_use]
    pub fn eoseq(&mut self) -> EOSEQ_W<ISR_SPEC, 3> {
        EOSEQ_W::new(self)
    }
    #[doc = "Bit 4 - ADC group regular overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<ISR_SPEC, 4> {
        OVR_W::new(self)
    }
    #[doc = "Bit 7 - ADC analog watchdog flag"]
    #[inline(always)]
    #[must_use]
    pub fn awd(&mut self) -> AWD_W<ISR_SPEC, 7> {
        AWD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
