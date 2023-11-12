#[doc = "Register `PERTPE` reader"]
pub type R = crate::R<PERTPE_SPEC>;
#[doc = "Register `PERTPE` writer"]
pub type W = crate::W<PERTPE_SPEC>;
#[doc = "Field `PERTPE` reader - FLash PERTPE register"]
pub type PERTPE_R = crate::FieldReader<u32>;
#[doc = "Field `PERTPE` writer - FLash PERTPE register"]
pub type PERTPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 17, O, u32>;
impl R {
    #[doc = "Bits 0:16 - FLash PERTPE register"]
    #[inline(always)]
    pub fn pertpe(&self) -> PERTPE_R {
        PERTPE_R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - FLash PERTPE register"]
    #[inline(always)]
    #[must_use]
    pub fn pertpe(&mut self) -> PERTPE_W<PERTPE_SPEC, 0> {
        PERTPE_W::new(self)
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
#[doc = "Flash PERTPE register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pertpe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pertpe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERTPE_SPEC;
impl crate::RegisterSpec for PERTPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pertpe::R`](R) reader structure"]
impl crate::Readable for PERTPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pertpe::W`](W) writer structure"]
impl crate::Writable for PERTPE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERTPE to value 0xea60"]
impl crate::Resettable for PERTPE_SPEC {
    const RESET_VALUE: Self::Ux = 0xea60;
}
