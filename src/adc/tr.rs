#[doc = "Register `TR` reader"]
pub type R = crate::R<TR_SPEC>;
#[doc = "Register `TR` writer"]
pub type W = crate::W<TR_SPEC>;
#[doc = "Field `LT` reader - ADC analog watchdog threshold low"]
pub type LT_R = crate::FieldReader<u16>;
#[doc = "Field `LT` writer - ADC analog watchdog threshold low"]
pub type LT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `HT` reader - ADC analog watchdog threshold high"]
pub type HT_R = crate::FieldReader<u16>;
#[doc = "Field `HT` writer - ADC analog watchdog threshold high"]
pub type HT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - ADC analog watchdog threshold low"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - ADC analog watchdog threshold high"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADC analog watchdog threshold low"]
    #[inline(always)]
    #[must_use]
    pub fn lt(&mut self) -> LT_W<TR_SPEC, 0> {
        LT_W::new(self)
    }
    #[doc = "Bits 16:27 - ADC analog watchdog threshold high"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<TR_SPEC, 16> {
        HT_W::new(self)
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
#[doc = "ADC analog watchdog 1 threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TR_SPEC;
impl crate::RegisterSpec for TR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr::R`](R) reader structure"]
impl crate::Readable for TR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tr::W`](W) writer structure"]
impl crate::Writable for TR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR to value 0x0fff_0000"]
impl crate::Resettable for TR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_0000;
}
