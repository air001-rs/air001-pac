#[doc = "Register `EGR` writer"]
pub type W = crate::W<EGR_SPEC>;
#[doc = "Field `UG` writer - Update generation"]
pub type UG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CC1G` writer - Capture/Compare 1 generation"]
pub type CC1G_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMG` writer - COM evnet generation"]
pub type COMG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BG` writer - Break event generation"]
pub type BG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<EGR_SPEC, 0> {
        UG_W::new(self)
    }
    #[doc = "Bit 1 - Capture/Compare 1 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CC1G_W<EGR_SPEC, 1> {
        CC1G_W::new(self)
    }
    #[doc = "Bit 5 - COM evnet generation"]
    #[inline(always)]
    #[must_use]
    pub fn comg(&mut self) -> COMG_W<EGR_SPEC, 5> {
        COMG_W::new(self)
    }
    #[doc = "Bit 7 - Break event generation"]
    #[inline(always)]
    #[must_use]
    pub fn bg(&mut self) -> BG_W<EGR_SPEC, 7> {
        BG_W::new(self)
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
#[doc = "event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EGR_SPEC;
impl crate::RegisterSpec for EGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`egr::W`](W) writer structure"]
impl crate::Writable for EGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
