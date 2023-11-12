#[doc = "Register `APB_FZ1` reader"]
pub type R = crate::R<APB_FZ1_SPEC>;
#[doc = "Register `APB_FZ1` writer"]
pub type W = crate::W<APB_FZ1_SPEC>;
#[doc = "Field `DBG_TIMER3_STOP` reader - Debug Timer 3 stopped when Core is halted"]
pub type DBG_TIMER3_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIMER3_STOP` writer - Debug Timer 3 stopped when Core is halted"]
pub type DBG_TIMER3_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBG_RTC_STOP` reader - Debug RTC stopped when Core is halted"]
pub type DBG_RTC_STOP_R = crate::BitReader;
#[doc = "Field `DBG_RTC_STOP` writer - Debug RTC stopped when Core is halted"]
pub type DBG_RTC_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBG_WWDG_STOP` reader - Debug Window Wachdog stopped when Core is halted"]
pub type DBG_WWDG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_WWDG_STOP` writer - Debug Window Wachdog stopped when Core is halted"]
pub type DBG_WWDG_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBG_IWDG_STOP` reader - Debug Independent Wachdog stopped when Core is halted"]
pub type DBG_IWDG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_IWDG_STOP` writer - Debug Independent Wachdog stopped when Core is halted"]
pub type DBG_IWDG_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBG_LPTIM_STOP` reader - Debug LPTIM stopped when Core is halted"]
pub type DBG_LPTIM_STOP_R = crate::BitReader;
#[doc = "Field `DBG_LPTIM_STOP` writer - Debug LPTIM stopped when Core is halted"]
pub type DBG_LPTIM_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Debug Timer 3 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer3_stop(&self) -> DBG_TIMER3_STOP_R {
        DBG_TIMER3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 31 - Debug LPTIM stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_lptim_stop(&self) -> DBG_LPTIM_STOP_R {
        DBG_LPTIM_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Debug Timer 3 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_timer3_stop(&mut self) -> DBG_TIMER3_STOP_W<APB_FZ1_SPEC, 1> {
        DBG_TIMER3_STOP_W::new(self)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<APB_FZ1_SPEC, 10> {
        DBG_RTC_STOP_W::new(self)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<APB_FZ1_SPEC, 11> {
        DBG_WWDG_STOP_W::new(self)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<APB_FZ1_SPEC, 12> {
        DBG_IWDG_STOP_W::new(self)
    }
    #[doc = "Bit 31 - Debug LPTIM stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim_stop(&mut self) -> DBG_LPTIM_STOP_W<APB_FZ1_SPEC, 31> {
        DBG_LPTIM_STOP_W::new(self)
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
#[doc = "APB Freeze Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_fz1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_fz1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_FZ1_SPEC;
impl crate::RegisterSpec for APB_FZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_fz1::R`](R) reader structure"]
impl crate::Readable for APB_FZ1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_fz1::W`](W) writer structure"]
impl crate::Writable for APB_FZ1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_FZ1 to value 0"]
impl crate::Resettable for APB_FZ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
