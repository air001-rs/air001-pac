#[doc = "Register `WINR` reader"]
pub type R = crate::R<WINR_SPEC>;
#[doc = "Field `WIN` reader - window counter"]
pub type WIN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - window counter"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Window register (IWDG_SR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`winr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WINR_SPEC;
impl crate::RegisterSpec for WINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`winr::R`](R) reader structure"]
impl crate::Readable for WINR_SPEC {}
#[doc = "`reset()` method sets WINR to value 0"]
impl crate::Resettable for WINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
