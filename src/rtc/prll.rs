#[doc = "Register `PRLL` writer"]
pub type W = crate::W<PRLL_SPEC>;
#[doc = "Field `PRLL` writer - RTC Prescaler Divider Register Low"]
pub type PRLL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl W {
    #[doc = "Bits 0:15 - RTC Prescaler Divider Register Low"]
    #[inline(always)]
    #[must_use]
    pub fn prll(&mut self) -> PRLL_W<PRLL_SPEC, 0> {
        PRLL_W::new(self)
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
#[doc = "RTC Prescaler Load Register Low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prll::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRLL_SPEC;
impl crate::RegisterSpec for PRLL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prll::W`](W) writer structure"]
impl crate::Writable for PRLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRLL to value 0x8000"]
impl crate::Resettable for PRLL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
