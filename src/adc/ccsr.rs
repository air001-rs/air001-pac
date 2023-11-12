#[doc = "Register `CCSR` reader"]
pub type R = crate::R<CCSR_SPEC>;
#[doc = "Register `CCSR` writer"]
pub type W = crate::W<CCSR_SPEC>;
#[doc = "Field `CALSEL` reader - Calibration contents selection"]
pub type CALSEL_R = crate::BitReader;
#[doc = "Field `CALSEL` writer - Calibration contents selection"]
pub type CALSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CALSMP` reader - Calibration sample time selection"]
pub type CALSMP_R = crate::FieldReader;
#[doc = "Field `CALSMP` writer - Calibration sample time selection"]
pub type CALSMP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CALSET` reader - Calibration factor selection"]
pub type CALSET_R = crate::BitReader;
#[doc = "Field `CALSET` writer - Calibration factor selection"]
pub type CALSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CALFAIL` reader - Calibration fail flag"]
pub type CALFAIL_R = crate::BitReader;
#[doc = "Field `CALFAIL` writer - Calibration fail flag"]
pub type CALFAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CALON` reader - Calibration flag"]
pub type CALON_R = crate::BitReader;
impl R {
    #[doc = "Bit 11 - Calibration contents selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Calibration sample time selection"]
    #[inline(always)]
    pub fn calsmp(&self) -> CALSMP_R {
        CALSMP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Calibration factor selection"]
    #[inline(always)]
    pub fn calset(&self) -> CALSET_R {
        CALSET_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 30 - Calibration fail flag"]
    #[inline(always)]
    pub fn calfail(&self) -> CALFAIL_R {
        CALFAIL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Calibration flag"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Calibration contents selection"]
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CALSEL_W<CCSR_SPEC, 11> {
        CALSEL_W::new(self)
    }
    #[doc = "Bits 12:13 - Calibration sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn calsmp(&mut self) -> CALSMP_W<CCSR_SPEC, 12> {
        CALSMP_W::new(self)
    }
    #[doc = "Bit 15 - Calibration factor selection"]
    #[inline(always)]
    #[must_use]
    pub fn calset(&mut self) -> CALSET_W<CCSR_SPEC, 15> {
        CALSET_W::new(self)
    }
    #[doc = "Bit 30 - Calibration fail flag"]
    #[inline(always)]
    #[must_use]
    pub fn calfail(&mut self) -> CALFAIL_W<CCSR_SPEC, 30> {
        CALFAIL_W::new(self)
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
#[doc = "ADC calibration configuration and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCSR_SPEC;
impl crate::RegisterSpec for CCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccsr::R`](R) reader structure"]
impl crate::Readable for CCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccsr::W`](W) writer structure"]
impl crate::Writable for CCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCSR to value 0"]
impl crate::Resettable for CCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
