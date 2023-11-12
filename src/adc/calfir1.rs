#[doc = "Register `CALFIR1` reader"]
pub type R = crate::R<CALFIR1_SPEC>;
#[doc = "Register `CALFIR1` writer"]
pub type W = crate::W<CALFIR1_SPEC>;
#[doc = "Field `CALC4IO` reader - Calibration C4 factor input"]
pub type CALC4IO_R = crate::FieldReader;
#[doc = "Field `CALC4IO` writer - Calibration C4 factor input"]
pub type CALC4IO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CALC5IO` reader - Calibration C5 factor input"]
pub type CALC5IO_R = crate::FieldReader;
#[doc = "Field `CALC5IO` writer - Calibration C5 factor input"]
pub type CALC5IO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CALBIO` reader - Calibration offset factor input"]
pub type CALBIO_R = crate::FieldReader;
#[doc = "Field `CALBIO` writer - Calibration offset factor input"]
pub type CALBIO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:7 - Calibration C4 factor input"]
    #[inline(always)]
    pub fn calc4io(&self) -> CALC4IO_R {
        CALC4IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Calibration C5 factor input"]
    #[inline(always)]
    pub fn calc5io(&self) -> CALC5IO_R {
        CALC5IO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - Calibration offset factor input"]
    #[inline(always)]
    pub fn calbio(&self) -> CALBIO_R {
        CALBIO_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calibration C4 factor input"]
    #[inline(always)]
    #[must_use]
    pub fn calc4io(&mut self) -> CALC4IO_W<CALFIR1_SPEC, 0> {
        CALC4IO_W::new(self)
    }
    #[doc = "Bits 8:15 - Calibration C5 factor input"]
    #[inline(always)]
    #[must_use]
    pub fn calc5io(&mut self) -> CALC5IO_W<CALFIR1_SPEC, 8> {
        CALC5IO_W::new(self)
    }
    #[doc = "Bits 16:22 - Calibration offset factor input"]
    #[inline(always)]
    #[must_use]
    pub fn calbio(&mut self) -> CALBIO_W<CALFIR1_SPEC, 16> {
        CALBIO_W::new(self)
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
#[doc = "ADC calibration factor input register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calfir1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calfir1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALFIR1_SPEC;
impl crate::RegisterSpec for CALFIR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calfir1::R`](R) reader structure"]
impl crate::Readable for CALFIR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`calfir1::W`](W) writer structure"]
impl crate::Writable for CALFIR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALFIR1 to value 0"]
impl crate::Resettable for CALFIR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
