#[doc = "Register `TS0` reader"]
pub type R = crate::R<TS0_SPEC>;
#[doc = "Register `TS0` writer"]
pub type W = crate::W<TS0_SPEC>;
#[doc = "Field `TS0` reader - FLash TS0 register"]
pub type TS0_R = crate::FieldReader;
#[doc = "Field `TS0` writer - FLash TS0 register"]
pub type TS0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - FLash TS0 register"]
    #[inline(always)]
    pub fn ts0(&self) -> TS0_R {
        TS0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FLash TS0 register"]
    #[inline(always)]
    #[must_use]
    pub fn ts0(&mut self) -> TS0_W<TS0_SPEC, 0> {
        TS0_W::new(self)
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
#[doc = "Flash TS0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ts0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ts0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TS0_SPEC;
impl crate::RegisterSpec for TS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts0::R`](R) reader structure"]
impl crate::Readable for TS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ts0::W`](W) writer structure"]
impl crate::Writable for TS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TS0 to value 0xb4"]
impl crate::Resettable for TS0_SPEC {
    const RESET_VALUE: Self::Ux = 0xb4;
}
