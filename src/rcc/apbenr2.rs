#[doc = "Register `APBENR2` reader"]
pub type R = crate::R<APBENR2_SPEC>;
#[doc = "Register `APBENR2` writer"]
pub type W = crate::W<APBENR2_SPEC>;
#[doc = "Field `SYSCFGEN` reader - SYSCFG, COMP and VREFBUF clock enable"]
pub type SYSCFGEN_R = crate::BitReader;
#[doc = "Field `SYSCFGEN` writer - SYSCFG, COMP and VREFBUF clock enable"]
pub type SYSCFGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM1EN` reader - TIM1 timer clock enable"]
pub type TIM1EN_R = crate::BitReader;
#[doc = "Field `TIM1EN` writer - TIM1 timer clock enable"]
pub type TIM1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub type SPI1EN_R = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub type SPI1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub type USART1EN_R = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub type USART1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM14EN` reader - TIM14 timer clock enable"]
pub type TIM14EN_R = crate::BitReader;
#[doc = "Field `TIM14EN` writer - TIM14 timer clock enable"]
pub type TIM14EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM16EN` reader - TIM16 timer clock enable"]
pub type TIM16EN_R = crate::BitReader;
#[doc = "Field `TIM16EN` writer - TIM16 timer clock enable"]
pub type TIM16EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM17EN` reader - TIM16 timer clock enable"]
pub type TIM17EN_R = crate::BitReader;
#[doc = "Field `TIM17EN` writer - TIM16 timer clock enable"]
pub type TIM17EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADCEN` reader - ADC clock enable"]
pub type ADCEN_R = crate::BitReader;
#[doc = "Field `ADCEN` writer - ADC clock enable"]
pub type ADCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP1EN` reader - COMP1 clock enable"]
pub type COMP1EN_R = crate::BitReader;
#[doc = "Field `COMP1EN` writer - COMP1 clock enable"]
pub type COMP1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP2EN` reader - COMP2 clock enable"]
pub type COMP2EN_R = crate::BitReader;
#[doc = "Field `COMP2EN` writer - COMP2 clock enable"]
pub type COMP2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LEDEN` reader - LED clock enable"]
pub type LEDEN_R = crate::BitReader;
#[doc = "Field `LEDEN` writer - LED clock enable"]
pub type LEDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIM14 timer clock enable"]
    #[inline(always)]
    pub fn tim14en(&self) -> TIM14EN_R {
        TIM14EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - COMP1 clock enable"]
    #[inline(always)]
    pub fn comp1en(&self) -> COMP1EN_R {
        COMP1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - COMP2 clock enable"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LED clock enable"]
    #[inline(always)]
    pub fn leden(&self) -> LEDEN_R {
        LEDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<APBENR2_SPEC, 0> {
        SYSCFGEN_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<APBENR2_SPEC, 11> {
        TIM1EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<APBENR2_SPEC, 12> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<APBENR2_SPEC, 14> {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 15 - TIM14 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim14en(&mut self) -> TIM14EN_W<APBENR2_SPEC, 15> {
        TIM14EN_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<APBENR2_SPEC, 17> {
        TIM16EN_W::new(self)
    }
    #[doc = "Bit 18 - TIM16 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<APBENR2_SPEC, 18> {
        TIM17EN_W::new(self)
    }
    #[doc = "Bit 20 - ADC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<APBENR2_SPEC, 20> {
        ADCEN_W::new(self)
    }
    #[doc = "Bit 21 - COMP1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1en(&mut self) -> COMP1EN_W<APBENR2_SPEC, 21> {
        COMP1EN_W::new(self)
    }
    #[doc = "Bit 22 - COMP2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp2en(&mut self) -> COMP2EN_W<APBENR2_SPEC, 22> {
        COMP2EN_W::new(self)
    }
    #[doc = "Bit 23 - LED clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn leden(&mut self) -> LEDEN_W<APBENR2_SPEC, 23> {
        LEDEN_W::new(self)
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
#[doc = "APB peripheral clock enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbenr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbenr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBENR2_SPEC;
impl crate::RegisterSpec for APBENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbenr2::R`](R) reader structure"]
impl crate::Readable for APBENR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbenr2::W`](W) writer structure"]
impl crate::Writable for APBENR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBENR2 to value 0"]
impl crate::Resettable for APBENR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
