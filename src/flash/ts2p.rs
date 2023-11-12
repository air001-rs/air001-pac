#[doc = "Register `TS2P` reader"]
pub type R = crate::R<TS2P_SPEC>;
#[doc = "Register `TS2P` writer"]
pub type W = crate::W<TS2P_SPEC>;
#[doc = "Field `TS2P` reader - FLash TS2P register"]
pub type TS2P_R = crate::FieldReader;
#[doc = "Field `TS2P` writer - FLash TS2P register"]
pub type TS2P_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - FLash TS2P register"]
    #[inline(always)]
    pub fn ts2p(&self) -> TS2P_R {
        TS2P_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FLash TS2P register"]
    #[inline(always)]
    #[must_use]
    pub fn ts2p(&mut self) -> TS2P_W<TS2P_SPEC, 0> {
        TS2P_W::new(self)
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
#[doc = "Flash TS2P register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ts2p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ts2p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TS2P_SPEC;
impl crate::RegisterSpec for TS2P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts2p::R`](R) reader structure"]
impl crate::Readable for TS2P_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ts2p::W`](W) writer structure"]
impl crate::Writable for TS2P_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TS2P to value 0xb4"]
impl crate::Resettable for TS2P_SPEC {
    const RESET_VALUE: Self::Ux = 0xb4;
}
