#[doc = "Register `CALFIR2` reader"]
pub type R = crate::R<CALFIR2_SPEC>;
#[doc = "Register `CALFIR2` writer"]
pub type W = crate::W<CALFIR2_SPEC>;
#[doc = "Field `CALC0IO` reader - Calibration C0 factor input"]
pub type CALC0IO_R = crate::FieldReader;
#[doc = "Field `CALC0IO` writer - Calibration C0 factor input"]
pub type CALC0IO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CALC1IO` reader - Calibration C1 factor input"]
pub type CALC1IO_R = crate::FieldReader;
#[doc = "Field `CALC1IO` writer - Calibration C1 factor input"]
pub type CALC1IO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CALC2IO` reader - Calibration C2 factor input"]
pub type CALC2IO_R = crate::FieldReader;
#[doc = "Field `CALC2IO` writer - Calibration C2 factor input"]
pub type CALC2IO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CALC3IO` reader - Calibration C3 factor input"]
pub type CALC3IO_R = crate::FieldReader;
#[doc = "Field `CALC3IO` writer - Calibration C3 factor input"]
pub type CALC3IO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Calibration C0 factor input"]
    #[inline(always)]
    pub fn calc0io(&self) -> CALC0IO_R {
        CALC0IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Calibration C1 factor input"]
    #[inline(always)]
    pub fn calc1io(&self) -> CALC1IO_R {
        CALC1IO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Calibration C2 factor input"]
    #[inline(always)]
    pub fn calc2io(&self) -> CALC2IO_R {
        CALC2IO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Calibration C3 factor input"]
    #[inline(always)]
    pub fn calc3io(&self) -> CALC3IO_R {
        CALC3IO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calibration C0 factor input"]
    #[inline(always)]
    #[must_use]
    pub fn calc0io(&mut self) -> CALC0IO_W<CALFIR2_SPEC, 0> {
        CALC0IO_W::new(self)
    }
    #[doc = "Bits 8:15 - Calibration C1 factor input"]
    #[inline(always)]
    #[must_use]
    pub fn calc1io(&mut self) -> CALC1IO_W<CALFIR2_SPEC, 8> {
        CALC1IO_W::new(self)
    }
    #[doc = "Bits 16:23 - Calibration C2 factor input"]
    #[inline(always)]
    #[must_use]
    pub fn calc2io(&mut self) -> CALC2IO_W<CALFIR2_SPEC, 16> {
        CALC2IO_W::new(self)
    }
    #[doc = "Bits 24:31 - Calibration C3 factor input"]
    #[inline(always)]
    #[must_use]
    pub fn calc3io(&mut self) -> CALC3IO_W<CALFIR2_SPEC, 24> {
        CALC3IO_W::new(self)
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
#[doc = "ADC calibration factor input register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calfir2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calfir2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALFIR2_SPEC;
impl crate::RegisterSpec for CALFIR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calfir2::R`](R) reader structure"]
impl crate::Readable for CALFIR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`calfir2::W`](W) writer structure"]
impl crate::Writable for CALFIR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALFIR2 to value 0"]
impl crate::Resettable for CALFIR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
