#[doc = "Register `TS1` reader"]
pub type R = crate::R<TS1_SPEC>;
#[doc = "Register `TS1` writer"]
pub type W = crate::W<TS1_SPEC>;
#[doc = "Field `TS1` reader - FLash TS1 register"]
pub type TS1_R = crate::FieldReader<u16>;
#[doc = "Field `TS1` writer - FLash TS1 register"]
pub type TS1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - FLash TS1 register"]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - FLash TS1 register"]
    #[inline(always)]
    #[must_use]
    pub fn ts1(&mut self) -> TS1_W<TS1_SPEC, 0> {
        TS1_W::new(self)
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
#[doc = "Flash TS1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ts1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ts1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TS1_SPEC;
impl crate::RegisterSpec for TS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts1::R`](R) reader structure"]
impl crate::Readable for TS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ts1::W`](W) writer structure"]
impl crate::Writable for TS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TS1 to value 0x01b0"]
impl crate::Resettable for TS1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01b0;
}
