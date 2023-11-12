#[doc = "Register `TPS3` reader"]
pub type R = crate::R<TPS3_SPEC>;
#[doc = "Register `TPS3` writer"]
pub type W = crate::W<TPS3_SPEC>;
#[doc = "Field `TPS3` reader - FLash TPS3 register"]
pub type TPS3_R = crate::FieldReader<u16>;
#[doc = "Field `TPS3` writer - FLash TPS3 register"]
pub type TPS3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - FLash TPS3 register"]
    #[inline(always)]
    pub fn tps3(&self) -> TPS3_R {
        TPS3_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - FLash TPS3 register"]
    #[inline(always)]
    #[must_use]
    pub fn tps3(&mut self) -> TPS3_W<TPS3_SPEC, 0> {
        TPS3_W::new(self)
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
#[doc = "Flash TPS3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tps3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tps3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPS3_SPEC;
impl crate::RegisterSpec for TPS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tps3::R`](R) reader structure"]
impl crate::Readable for TPS3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tps3::W`](W) writer structure"]
impl crate::Writable for TPS3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPS3 to value 0x06c0"]
impl crate::Resettable for TPS3_SPEC {
    const RESET_VALUE: Self::Ux = 0x06c0;
}
