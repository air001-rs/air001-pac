#[doc = "Register `SMERTPE` reader"]
pub type R = crate::R<SMERTPE_SPEC>;
#[doc = "Register `SMERTPE` writer"]
pub type W = crate::W<SMERTPE_SPEC>;
#[doc = "Field `SMERTPE` reader - FLash SMERTPE register"]
pub type SMERTPE_R = crate::FieldReader<u32>;
#[doc = "Field `SMERTPE` writer - FLash SMERTPE register"]
pub type SMERTPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 17, O, u32>;
impl R {
    #[doc = "Bits 0:16 - FLash SMERTPE register"]
    #[inline(always)]
    pub fn smertpe(&self) -> SMERTPE_R {
        SMERTPE_R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - FLash SMERTPE register"]
    #[inline(always)]
    #[must_use]
    pub fn smertpe(&mut self) -> SMERTPE_W<SMERTPE_SPEC, 0> {
        SMERTPE_W::new(self)
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
#[doc = "Flash SMERTPE register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smertpe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smertpe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMERTPE_SPEC;
impl crate::RegisterSpec for SMERTPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smertpe::R`](R) reader structure"]
impl crate::Readable for SMERTPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smertpe::W`](W) writer structure"]
impl crate::Writable for SMERTPE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMERTPE to value 0xfd20"]
impl crate::Resettable for SMERTPE_SPEC {
    const RESET_VALUE: Self::Ux = 0xfd20;
}
