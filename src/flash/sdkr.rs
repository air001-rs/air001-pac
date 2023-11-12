#[doc = "Register `SDKR` reader"]
pub type R = crate::R<SDKR_SPEC>;
#[doc = "Register `SDKR` writer"]
pub type W = crate::W<SDKR_SPEC>;
#[doc = "Field `SDK_STRT` reader - SDK area start address"]
pub type SDK_STRT_R = crate::FieldReader;
#[doc = "Field `SDK_STRT` writer - SDK area start address"]
pub type SDK_STRT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SDK_END` reader - SDK area end address"]
pub type SDK_END_R = crate::FieldReader;
#[doc = "Field `SDK_END` writer - SDK area end address"]
pub type SDK_END_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - SDK area start address"]
    #[inline(always)]
    pub fn sdk_strt(&self) -> SDK_STRT_R {
        SDK_STRT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - SDK area end address"]
    #[inline(always)]
    pub fn sdk_end(&self) -> SDK_END_R {
        SDK_END_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - SDK area start address"]
    #[inline(always)]
    #[must_use]
    pub fn sdk_strt(&mut self) -> SDK_STRT_W<SDKR_SPEC, 0> {
        SDK_STRT_W::new(self)
    }
    #[doc = "Bits 8:12 - SDK area end address"]
    #[inline(always)]
    #[must_use]
    pub fn sdk_end(&mut self) -> SDK_END_W<SDKR_SPEC, 8> {
        SDK_END_W::new(self)
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
#[doc = "Flash SDK address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdkr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdkr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDKR_SPEC;
impl crate::RegisterSpec for SDKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdkr::R`](R) reader structure"]
impl crate::Readable for SDKR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdkr::W`](W) writer structure"]
impl crate::Writable for SDKR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDKR to value 0xffe0_001f"]
impl crate::Resettable for SDKR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffe0_001f;
}
