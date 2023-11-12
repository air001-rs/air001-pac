#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `BIAS_CR` reader - MR Bias current"]
pub type BIAS_CR_R = crate::FieldReader;
#[doc = "Field `BIAS_CR` writer - MR Bias current"]
pub type BIAS_CR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `BIAS_CR_SEL` reader - MR Bias current selection"]
pub type BIAS_CR_SEL_R = crate::BitReader;
#[doc = "Field `BIAS_CR_SEL` writer - MR Bias current selection"]
pub type BIAS_CR_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBP` reader - Disable backup domain write protection"]
pub type DBP_R = crate::BitReader;
#[doc = "Field `DBP` writer - Disable backup domain write protection"]
pub type DBP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VOS` reader - Voltage scaling range selection"]
pub type VOS_R = crate::BitReader;
#[doc = "Field `VOS` writer - Voltage scaling range selection"]
pub type VOS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MRRDY_TIME` reader - Time selection wakeup from LP to VR"]
pub type MRRDY_TIME_R = crate::FieldReader;
#[doc = "Field `MRRDY_TIME` writer - Time selection wakeup from LP to VR"]
pub type MRRDY_TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FLS_SLPTIME` reader - Flash wait time after wakeup from the stop mode"]
pub type FLS_SLPTIME_R = crate::FieldReader;
#[doc = "Field `FLS_SLPTIME` writer - Flash wait time after wakeup from the stop mode"]
pub type FLS_SLPTIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LPR` reader - Low-power run"]
pub type LPR_R = crate::BitReader;
#[doc = "Field `LPR` writer - Low-power run"]
pub type LPR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAM_RETV` reader - SRAM retention voltage control"]
pub type SRAM_RETV_R = crate::FieldReader;
#[doc = "Field `SRAM_RETV` writer - SRAM retention voltage control"]
pub type SRAM_RETV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `HSION_CTRL` reader - HSI open time control"]
pub type HSION_CTRL_R = crate::BitReader;
#[doc = "Field `HSION_CTRL` writer - HSI open time control"]
pub type HSION_CTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - MR Bias current"]
    #[inline(always)]
    pub fn bias_cr(&self) -> BIAS_CR_R {
        BIAS_CR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - MR Bias current selection"]
    #[inline(always)]
    pub fn bias_cr_sel(&self) -> BIAS_CR_SEL_R {
        BIAS_CR_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Time selection wakeup from LP to VR"]
    #[inline(always)]
    pub fn mrrdy_time(&self) -> MRRDY_TIME_R {
        MRRDY_TIME_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Flash wait time after wakeup from the stop mode"]
    #[inline(always)]
    pub fn fls_slptime(&self) -> FLS_SLPTIME_R {
        FLS_SLPTIME_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    pub fn lpr(&self) -> LPR_R {
        LPR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SRAM retention voltage control"]
    #[inline(always)]
    pub fn sram_retv(&self) -> SRAM_RETV_R {
        SRAM_RETV_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - HSI open time control"]
    #[inline(always)]
    pub fn hsion_ctrl(&self) -> HSION_CTRL_R {
        HSION_CTRL_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - MR Bias current"]
    #[inline(always)]
    #[must_use]
    pub fn bias_cr(&mut self) -> BIAS_CR_W<CR1_SPEC, 0> {
        BIAS_CR_W::new(self)
    }
    #[doc = "Bit 4 - MR Bias current selection"]
    #[inline(always)]
    #[must_use]
    pub fn bias_cr_sel(&mut self) -> BIAS_CR_SEL_W<CR1_SPEC, 4> {
        BIAS_CR_SEL_W::new(self)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<CR1_SPEC, 8> {
        DBP_W::new(self)
    }
    #[doc = "Bit 9 - Voltage scaling range selection"]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<CR1_SPEC, 9> {
        VOS_W::new(self)
    }
    #[doc = "Bits 10:11 - Time selection wakeup from LP to VR"]
    #[inline(always)]
    #[must_use]
    pub fn mrrdy_time(&mut self) -> MRRDY_TIME_W<CR1_SPEC, 10> {
        MRRDY_TIME_W::new(self)
    }
    #[doc = "Bits 12:13 - Flash wait time after wakeup from the stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn fls_slptime(&mut self) -> FLS_SLPTIME_W<CR1_SPEC, 12> {
        FLS_SLPTIME_W::new(self)
    }
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    #[must_use]
    pub fn lpr(&mut self) -> LPR_W<CR1_SPEC, 14> {
        LPR_W::new(self)
    }
    #[doc = "Bits 16:18 - SRAM retention voltage control"]
    #[inline(always)]
    #[must_use]
    pub fn sram_retv(&mut self) -> SRAM_RETV_W<CR1_SPEC, 16> {
        SRAM_RETV_W::new(self)
    }
    #[doc = "Bit 19 - HSI open time control"]
    #[inline(always)]
    #[must_use]
    pub fn hsion_ctrl(&mut self) -> HSION_CTRL_W<CR1_SPEC, 19> {
        HSION_CTRL_W::new(self)
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
#[doc = "Power control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0x0003_0000"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0000;
}
