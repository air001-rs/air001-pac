#[doc = "Register `ECSCR` reader"]
pub type R = crate::R<ECSCR_SPEC>;
#[doc = "Register `ECSCR` writer"]
pub type W = crate::W<ECSCR_SPEC>;
#[doc = "Field `HSE_FREQ` reader - HSE clock freqency selection"]
pub type HSE_FREQ_R = crate::FieldReader;
#[doc = "Field `HSE_FREQ` writer - HSE clock freqency selection"]
pub type HSE_FREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LSE_DRIVER` reader - LSE clock driver selection"]
pub type LSE_DRIVER_R = crate::FieldReader;
#[doc = "Field `LSE_DRIVER` writer - LSE clock driver selection"]
pub type LSE_DRIVER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 2:3 - HSE clock freqency selection"]
    #[inline(always)]
    pub fn hse_freq(&self) -> HSE_FREQ_R {
        HSE_FREQ_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 16:17 - LSE clock driver selection"]
    #[inline(always)]
    pub fn lse_driver(&self) -> LSE_DRIVER_R {
        LSE_DRIVER_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - HSE clock freqency selection"]
    #[inline(always)]
    #[must_use]
    pub fn hse_freq(&mut self) -> HSE_FREQ_W<ECSCR_SPEC, 2> {
        HSE_FREQ_W::new(self)
    }
    #[doc = "Bits 16:17 - LSE clock driver selection"]
    #[inline(always)]
    #[must_use]
    pub fn lse_driver(&mut self) -> LSE_DRIVER_W<ECSCR_SPEC, 16> {
        LSE_DRIVER_W::new(self)
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
#[doc = "External clock source control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECSCR_SPEC;
impl crate::RegisterSpec for ECSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecscr::R`](R) reader structure"]
impl crate::Readable for ECSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecscr::W`](W) writer structure"]
impl crate::Writable for ECSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECSCR to value 0"]
impl crate::Resettable for ECSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
