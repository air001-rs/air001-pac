#[doc = "Register `STCR` reader"]
pub type R = crate::R<STCR_SPEC>;
#[doc = "Register `STCR` writer"]
pub type W = crate::W<STCR_SPEC>;
#[doc = "Field `SLEEP_EN` reader - FLash sleep enable"]
pub type SLEEP_EN_R = crate::BitReader;
#[doc = "Field `SLEEP_EN` writer - FLash sleep enable"]
pub type SLEEP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLEEP_TIME` reader - FLash sleep time configuration(counter based on HSI_10M)"]
pub type SLEEP_TIME_R = crate::FieldReader;
#[doc = "Field `SLEEP_TIME` writer - FLash sleep time configuration(counter based on HSI_10M)"]
pub type SLEEP_TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 0 - FLash sleep enable"]
    #[inline(always)]
    pub fn sleep_en(&self) -> SLEEP_EN_R {
        SLEEP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - FLash sleep time configuration(counter based on HSI_10M)"]
    #[inline(always)]
    pub fn sleep_time(&self) -> SLEEP_TIME_R {
        SLEEP_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FLash sleep enable"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_en(&mut self) -> SLEEP_EN_W<STCR_SPEC, 0> {
        SLEEP_EN_W::new(self)
    }
    #[doc = "Bits 8:15 - FLash sleep time configuration(counter based on HSI_10M)"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_time(&mut self) -> SLEEP_TIME_W<STCR_SPEC, 8> {
        SLEEP_TIME_W::new(self)
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
#[doc = "Flash sleep time config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STCR_SPEC;
impl crate::RegisterSpec for STCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcr::R`](R) reader structure"]
impl crate::Readable for STCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stcr::W`](W) writer structure"]
impl crate::Writable for STCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCR to value 0x6400"]
impl crate::Resettable for STCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x6400;
}
