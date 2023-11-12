#[doc = "Register `TS3` reader"]
pub type R = crate::R<TS3_SPEC>;
#[doc = "Register `TS3` writer"]
pub type W = crate::W<TS3_SPEC>;
#[doc = "Field `TS3` reader - FLash TS3 register"]
pub type TS3_R = crate::FieldReader;
#[doc = "Field `TS3` writer - FLash TS3 register"]
pub type TS3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - FLash TS3 register"]
    #[inline(always)]
    pub fn ts3(&self) -> TS3_R {
        TS3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FLash TS3 register"]
    #[inline(always)]
    #[must_use]
    pub fn ts3(&mut self) -> TS3_W<TS3_SPEC, 0> {
        TS3_W::new(self)
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
#[doc = "Flash TS3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ts3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ts3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TS3_SPEC;
impl crate::RegisterSpec for TS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts3::R`](R) reader structure"]
impl crate::Readable for TS3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ts3::W`](W) writer structure"]
impl crate::Writable for TS3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TS3 to value 0xb4"]
impl crate::Resettable for TS3_SPEC {
    const RESET_VALUE: Self::Ux = 0xb4;
}
