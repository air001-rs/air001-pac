#[doc = "Register `CHSELR` reader"]
pub type R = crate::R<CHSELR_SPEC>;
#[doc = "Register `CHSELR` writer"]
pub type W = crate::W<CHSELR_SPEC>;
#[doc = "Field `CHSEL0` reader - Channel-0 selection"]
pub type CHSEL0_R = crate::BitReader;
#[doc = "Field `CHSEL0` writer - Channel-0 selection"]
pub type CHSEL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHSEL1` reader - Channel-1 selection"]
pub type CHSEL1_R = crate::BitReader;
#[doc = "Field `CHSEL1` writer - Channel-1 selection"]
pub type CHSEL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHSEL2` reader - Channel-2 selection"]
pub type CHSEL2_R = crate::BitReader;
#[doc = "Field `CHSEL2` writer - Channel-2 selection"]
pub type CHSEL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHSEL3` reader - Channel-3 selection"]
pub type CHSEL3_R = crate::BitReader;
#[doc = "Field `CHSEL3` writer - Channel-3 selection"]
pub type CHSEL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHSEL4` reader - Channel-4 selection"]
pub type CHSEL4_R = crate::BitReader;
#[doc = "Field `CHSEL4` writer - Channel-4 selection"]
pub type CHSEL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHSEL5` reader - Channel-5 selection"]
pub type CHSEL5_R = crate::BitReader;
#[doc = "Field `CHSEL5` writer - Channel-5 selection"]
pub type CHSEL5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHSEL6` reader - Channel-6 selection"]
pub type CHSEL6_R = crate::BitReader;
#[doc = "Field `CHSEL6` writer - Channel-6 selection"]
pub type CHSEL6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHSEL7` reader - Channel-7 selection"]
pub type CHSEL7_R = crate::BitReader;
#[doc = "Field `CHSEL7` writer - Channel-7 selection"]
pub type CHSEL7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHSEL8` reader - Channel-8 selection"]
pub type CHSEL8_R = crate::BitReader;
#[doc = "Field `CHSEL8` writer - Channel-8 selection"]
pub type CHSEL8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHSEL9` reader - Channel-9 selection"]
pub type CHSEL9_R = crate::BitReader;
#[doc = "Field `CHSEL9` writer - Channel-9 selection"]
pub type CHSEL9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHSEL11` reader - Channel-11 selection"]
pub type CHSEL11_R = crate::BitReader;
#[doc = "Field `CHSEL11` writer - Channel-11 selection"]
pub type CHSEL11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHSEL12` reader - Channel-12 selection"]
pub type CHSEL12_R = crate::BitReader;
#[doc = "Field `CHSEL12` writer - Channel-12 selection"]
pub type CHSEL12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Channel-0 selection"]
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL0_R {
        CHSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel-1 selection"]
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL1_R {
        CHSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel-2 selection"]
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL2_R {
        CHSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel-3 selection"]
    #[inline(always)]
    pub fn chsel3(&self) -> CHSEL3_R {
        CHSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel-4 selection"]
    #[inline(always)]
    pub fn chsel4(&self) -> CHSEL4_R {
        CHSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel-5 selection"]
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL5_R {
        CHSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel-6 selection"]
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL6_R {
        CHSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel-7 selection"]
    #[inline(always)]
    pub fn chsel7(&self) -> CHSEL7_R {
        CHSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel-8 selection"]
    #[inline(always)]
    pub fn chsel8(&self) -> CHSEL8_R {
        CHSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel-9 selection"]
    #[inline(always)]
    pub fn chsel9(&self) -> CHSEL9_R {
        CHSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel-11 selection"]
    #[inline(always)]
    pub fn chsel11(&self) -> CHSEL11_R {
        CHSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel-12 selection"]
    #[inline(always)]
    pub fn chsel12(&self) -> CHSEL12_R {
        CHSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel-0 selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel0(&mut self) -> CHSEL0_W<CHSELR_SPEC, 0> {
        CHSEL0_W::new(self)
    }
    #[doc = "Bit 1 - Channel-1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel1(&mut self) -> CHSEL1_W<CHSELR_SPEC, 1> {
        CHSEL1_W::new(self)
    }
    #[doc = "Bit 2 - Channel-2 selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel2(&mut self) -> CHSEL2_W<CHSELR_SPEC, 2> {
        CHSEL2_W::new(self)
    }
    #[doc = "Bit 3 - Channel-3 selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel3(&mut self) -> CHSEL3_W<CHSELR_SPEC, 3> {
        CHSEL3_W::new(self)
    }
    #[doc = "Bit 4 - Channel-4 selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel4(&mut self) -> CHSEL4_W<CHSELR_SPEC, 4> {
        CHSEL4_W::new(self)
    }
    #[doc = "Bit 5 - Channel-5 selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel5(&mut self) -> CHSEL5_W<CHSELR_SPEC, 5> {
        CHSEL5_W::new(self)
    }
    #[doc = "Bit 6 - Channel-6 selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel6(&mut self) -> CHSEL6_W<CHSELR_SPEC, 6> {
        CHSEL6_W::new(self)
    }
    #[doc = "Bit 7 - Channel-7 selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel7(&mut self) -> CHSEL7_W<CHSELR_SPEC, 7> {
        CHSEL7_W::new(self)
    }
    #[doc = "Bit 8 - Channel-8 selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel8(&mut self) -> CHSEL8_W<CHSELR_SPEC, 8> {
        CHSEL8_W::new(self)
    }
    #[doc = "Bit 9 - Channel-9 selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel9(&mut self) -> CHSEL9_W<CHSELR_SPEC, 9> {
        CHSEL9_W::new(self)
    }
    #[doc = "Bit 11 - Channel-11 selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel11(&mut self) -> CHSEL11_W<CHSELR_SPEC, 11> {
        CHSEL11_W::new(self)
    }
    #[doc = "Bit 12 - Channel-12 selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel12(&mut self) -> CHSEL12_W<CHSELR_SPEC, 12> {
        CHSEL12_W::new(self)
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
#[doc = "ADC group regular sequencer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHSELR_SPEC;
impl crate::RegisterSpec for CHSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chselr::R`](R) reader structure"]
impl crate::Readable for CHSELR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chselr::W`](W) writer structure"]
impl crate::Writable for CHSELR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHSELR to value 0x0fff_0000"]
impl crate::Resettable for CHSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_0000;
}
