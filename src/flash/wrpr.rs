#[doc = "Register `WRPR` reader"]
pub type R = crate::R<WRPR_SPEC>;
#[doc = "Register `WRPR` writer"]
pub type W = crate::W<WRPR_SPEC>;
#[doc = "Field `WRP` reader - WRP address"]
pub type WRP_R = crate::FieldReader<u16>;
#[doc = "Field `WRP` writer - WRP address"]
pub type WRP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - WRP address"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WRP address"]
    #[inline(always)]
    #[must_use]
    pub fn wrp(&mut self) -> WRP_W<WRPR_SPEC, 0> {
        WRP_W::new(self)
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
#[doc = "Flash WRP address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRPR_SPEC;
impl crate::RegisterSpec for WRPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpr::R`](R) reader structure"]
impl crate::Readable for WRPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wrpr::W`](W) writer structure"]
impl crate::Writable for WRPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRPR to value 0xffff"]
impl crate::Resettable for WRPR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
