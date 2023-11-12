#[doc = "Register `PRGTPE` reader"]
pub type R = crate::R<PRGTPE_SPEC>;
#[doc = "Register `PRGTPE` writer"]
pub type W = crate::W<PRGTPE_SPEC>;
#[doc = "Field `PRGTPE` reader - FLash PRGTPE register"]
pub type PRGTPE_R = crate::FieldReader<u16>;
#[doc = "Field `PRGTPE` writer - FLash PRGTPE register"]
pub type PRGTPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - FLash PRGTPE register"]
    #[inline(always)]
    pub fn prgtpe(&self) -> PRGTPE_R {
        PRGTPE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - FLash PRGTPE register"]
    #[inline(always)]
    #[must_use]
    pub fn prgtpe(&mut self) -> PRGTPE_W<PRGTPE_SPEC, 0> {
        PRGTPE_W::new(self)
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
#[doc = "Flash PRGTPE register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prgtpe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prgtpe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRGTPE_SPEC;
impl crate::RegisterSpec for PRGTPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prgtpe::R`](R) reader structure"]
impl crate::Readable for PRGTPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prgtpe::W`](W) writer structure"]
impl crate::Writable for PRGTPE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRGTPE to value 0x8ca0"]
impl crate::Resettable for PRGTPE_SPEC {
    const RESET_VALUE: Self::Ux = 0x8ca0;
}
