#[doc = "Register `APBRSTR1` reader"]
pub type R = crate::R<APBRSTR1_SPEC>;
#[doc = "Register `APBRSTR1` writer"]
pub type W = crate::W<APBRSTR1_SPEC>;
#[doc = "Field `TIM3RST` reader - TIM3 timer reset"]
pub type TIM3RST_R = crate::BitReader;
#[doc = "Field `TIM3RST` writer - TIM3 timer reset"]
pub type TIM3RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub type SPI2RST_R = crate::BitReader;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub type SPI2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART2RST` reader - USART2 reset"]
pub type USART2RST_R = crate::BitReader;
#[doc = "Field `USART2RST` writer - USART2 reset"]
pub type USART2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2CRST` reader - I2C reset"]
pub type I2CRST_R = crate::BitReader;
#[doc = "Field `I2CRST` writer - I2C reset"]
pub type I2CRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBGRST` reader - Debug support reset"]
pub type DBGRST_R = crate::BitReader;
#[doc = "Field `DBGRST` writer - Debug support reset"]
pub type DBGRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub type PWRRST_R = crate::BitReader;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub type PWRRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIMRST` reader - Low Power Timer reset"]
pub type LPTIMRST_R = crate::BitReader;
#[doc = "Field `LPTIMRST` writer - Low Power Timer reset"]
pub type LPTIMRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - TIM3 timer reset"]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C reset"]
    #[inline(always)]
    pub fn i2crst(&self) -> I2CRST_R {
        I2CRST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug support reset"]
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Power Timer reset"]
    #[inline(always)]
    pub fn lptimrst(&self) -> LPTIMRST_R {
        LPTIMRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TIM3 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<APBRSTR1_SPEC, 1> {
        TIM3RST_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<APBRSTR1_SPEC, 14> {
        SPI2RST_W::new(self)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<APBRSTR1_SPEC, 17> {
        USART2RST_W::new(self)
    }
    #[doc = "Bit 21 - I2C reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2crst(&mut self) -> I2CRST_W<APBRSTR1_SPEC, 21> {
        I2CRST_W::new(self)
    }
    #[doc = "Bit 27 - Debug support reset"]
    #[inline(always)]
    #[must_use]
    pub fn dbgrst(&mut self) -> DBGRST_W<APBRSTR1_SPEC, 27> {
        DBGRST_W::new(self)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwrrst(&mut self) -> PWRRST_W<APBRSTR1_SPEC, 28> {
        PWRRST_W::new(self)
    }
    #[doc = "Bit 31 - Low Power Timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn lptimrst(&mut self) -> LPTIMRST_W<APBRSTR1_SPEC, 31> {
        LPTIMRST_W::new(self)
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
#[doc = "APB peripheral reset register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbrstr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbrstr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBRSTR1_SPEC;
impl crate::RegisterSpec for APBRSTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrstr1::R`](R) reader structure"]
impl crate::Readable for APBRSTR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbrstr1::W`](W) writer structure"]
impl crate::Writable for APBRSTR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBRSTR1 to value 0"]
impl crate::Resettable for APBRSTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
