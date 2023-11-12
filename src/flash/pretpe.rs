#[doc = "Register `PRETPE` reader"]
pub type R = crate::R<PRETPE_SPEC>;
#[doc = "Register `PRETPE` writer"]
pub type W = crate::W<PRETPE_SPEC>;
#[doc = "Field `PRETPE` reader - FLash PRETPE register"]
pub type PRETPE_R = crate::FieldReader<u16>;
#[doc = "Field `PRETPE` writer - FLash PRETPE register"]
pub type PRETPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
impl R {
    #[doc = "Bits 0:12 - FLash PRETPE register"]
    #[inline(always)]
    pub fn pretpe(&self) -> PRETPE_R {
        PRETPE_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - FLash PRETPE register"]
    #[inline(always)]
    #[must_use]
    pub fn pretpe(&mut self) -> PRETPE_W<PRETPE_SPEC, 0> {
        PRETPE_W::new(self)
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
#[doc = "Flash PRETPE register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pretpe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pretpe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRETPE_SPEC;
impl crate::RegisterSpec for PRETPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pretpe::R`](R) reader structure"]
impl crate::Readable for PRETPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pretpe::W`](W) writer structure"]
impl crate::Writable for PRETPE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRETPE to value 0x12c0"]
impl crate::Resettable for PRETPE_SPEC {
    const RESET_VALUE: Self::Ux = 0x12c0;
}
