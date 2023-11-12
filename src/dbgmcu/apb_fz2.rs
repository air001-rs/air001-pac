#[doc = "Register `APB_FZ2` reader"]
pub type R = crate::R<APB_FZ2_SPEC>;
#[doc = "Register `APB_FZ2` writer"]
pub type W = crate::W<APB_FZ2_SPEC>;
#[doc = "Field `DBG_TIMER1_STOP` reader - Debug Timer 1 stopped when Core is halted"]
pub type DBG_TIMER1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIMER1_STOP` writer - Debug Timer 1 stopped when Core is halted"]
pub type DBG_TIMER1_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBG_TIMER14_STOP` reader - Debug Timer 14 stopped when Core is halted"]
pub type DBG_TIMER14_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIMER14_STOP` writer - Debug Timer 14 stopped when Core is halted"]
pub type DBG_TIMER14_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBG_TIMER16_STOP` reader - Debug Timer 16 stopped when Core is halted"]
pub type DBG_TIMER16_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIMER16_STOP` writer - Debug Timer 16 stopped when Core is halted"]
pub type DBG_TIMER16_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBG_TIMER17_STOP` reader - Debug Timer 17 stopped when Core is halted"]
pub type DBG_TIMER17_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIMER17_STOP` writer - Debug Timer 17 stopped when Core is halted"]
pub type DBG_TIMER17_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 11 - Debug Timer 1 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer1_stop(&self) -> DBG_TIMER1_STOP_R {
        DBG_TIMER1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Debug Timer 14 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer14_stop(&self) -> DBG_TIMER14_STOP_R {
        DBG_TIMER14_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Debug Timer 16 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer16_stop(&self) -> DBG_TIMER16_STOP_R {
        DBG_TIMER16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Debug Timer 17 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer17_stop(&self) -> DBG_TIMER17_STOP_R {
        DBG_TIMER17_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Debug Timer 1 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_timer1_stop(&mut self) -> DBG_TIMER1_STOP_W<APB_FZ2_SPEC, 11> {
        DBG_TIMER1_STOP_W::new(self)
    }
    #[doc = "Bit 15 - Debug Timer 14 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_timer14_stop(&mut self) -> DBG_TIMER14_STOP_W<APB_FZ2_SPEC, 15> {
        DBG_TIMER14_STOP_W::new(self)
    }
    #[doc = "Bit 17 - Debug Timer 16 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_timer16_stop(&mut self) -> DBG_TIMER16_STOP_W<APB_FZ2_SPEC, 17> {
        DBG_TIMER16_STOP_W::new(self)
    }
    #[doc = "Bit 18 - Debug Timer 17 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_timer17_stop(&mut self) -> DBG_TIMER17_STOP_W<APB_FZ2_SPEC, 18> {
        DBG_TIMER17_STOP_W::new(self)
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
#[doc = "APB Freeze Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_fz2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_fz2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_FZ2_SPEC;
impl crate::RegisterSpec for APB_FZ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_fz2::R`](R) reader structure"]
impl crate::Readable for APB_FZ2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_fz2::W`](W) writer structure"]
impl crate::Writable for APB_FZ2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_FZ2 to value 0"]
impl crate::Resettable for APB_FZ2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
