#[doc = "Register `LCKR` reader"]
pub type R = crate::R<LCKR_SPEC>;
#[doc = "Register `LCKR` writer"]
pub type W = crate::W<LCKR_SPEC>;
#[doc = "Field `LCK0` reader - Port x lock bit y (y= 0..15)"]
pub type LCK0_R = crate::BitReader;
#[doc = "Field `LCK0` writer - Port x lock bit y (y= 0..15)"]
pub type LCK0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCK1` reader - Port x lock bit y (y= 0..15)"]
pub type LCK1_R = crate::BitReader;
#[doc = "Field `LCK1` writer - Port x lock bit y (y= 0..15)"]
pub type LCK1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCK2` reader - Port x lock bit y (y= 0..15)"]
pub type LCK2_R = crate::BitReader;
#[doc = "Field `LCK2` writer - Port x lock bit y (y= 0..15)"]
pub type LCK2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCK3` reader - Port x lock bit y (y= 0..15)"]
pub type LCK3_R = crate::BitReader;
#[doc = "Field `LCK3` writer - Port x lock bit y (y= 0..15)"]
pub type LCK3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCK4` reader - Port x lock bit y (y= 0..15)"]
pub type LCK4_R = crate::BitReader;
#[doc = "Field `LCK4` writer - Port x lock bit y (y= 0..15)"]
pub type LCK4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCK5` reader - Port x lock bit y (y= 0..15)"]
pub type LCK5_R = crate::BitReader;
#[doc = "Field `LCK5` writer - Port x lock bit y (y= 0..15)"]
pub type LCK5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCK6` reader - Port x lock bit y (y= 0..15)"]
pub type LCK6_R = crate::BitReader;
#[doc = "Field `LCK6` writer - Port x lock bit y (y= 0..15)"]
pub type LCK6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCK7` reader - Port x lock bit y (y= 0..15)"]
pub type LCK7_R = crate::BitReader;
#[doc = "Field `LCK7` writer - Port x lock bit y (y= 0..15)"]
pub type LCK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCK8` reader - Port x lock bit y (y= 0..15)"]
pub type LCK8_R = crate::BitReader;
#[doc = "Field `LCK8` writer - Port x lock bit y (y= 0..15)"]
pub type LCK8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCKK` reader - Port x lock bit y (y= 0..15)"]
pub type LCKK_R = crate::BitReader;
#[doc = "Field `LCKK` writer - Port x lock bit y (y= 0..15)"]
pub type LCKK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck2(&self) -> LCK2_R {
        LCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck4(&self) -> LCK4_R {
        LCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck5(&self) -> LCK5_R {
        LCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck6(&self) -> LCK6_R {
        LCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck7(&self) -> LCK7_R {
        LCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck8(&self) -> LCK8_R {
        LCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck0(&mut self) -> LCK0_W<LCKR_SPEC, 0> {
        LCK0_W::new(self)
    }
    #[doc = "Bit 1 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck1(&mut self) -> LCK1_W<LCKR_SPEC, 1> {
        LCK1_W::new(self)
    }
    #[doc = "Bit 2 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck2(&mut self) -> LCK2_W<LCKR_SPEC, 2> {
        LCK2_W::new(self)
    }
    #[doc = "Bit 3 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck3(&mut self) -> LCK3_W<LCKR_SPEC, 3> {
        LCK3_W::new(self)
    }
    #[doc = "Bit 4 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck4(&mut self) -> LCK4_W<LCKR_SPEC, 4> {
        LCK4_W::new(self)
    }
    #[doc = "Bit 5 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck5(&mut self) -> LCK5_W<LCKR_SPEC, 5> {
        LCK5_W::new(self)
    }
    #[doc = "Bit 6 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck6(&mut self) -> LCK6_W<LCKR_SPEC, 6> {
        LCK6_W::new(self)
    }
    #[doc = "Bit 7 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck7(&mut self) -> LCK7_W<LCKR_SPEC, 7> {
        LCK7_W::new(self)
    }
    #[doc = "Bit 8 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck8(&mut self) -> LCK8_W<LCKR_SPEC, 8> {
        LCK8_W::new(self)
    }
    #[doc = "Bit 16 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lckk(&mut self) -> LCKK_W<LCKR_SPEC, 16> {
        LCKK_W::new(self)
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
#[doc = "GPIO port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lckr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lckr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCKR_SPEC;
impl crate::RegisterSpec for LCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lckr::R`](R) reader structure"]
impl crate::Readable for LCKR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lckr::W`](W) writer structure"]
impl crate::Writable for LCKR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCKR to value 0"]
impl crate::Resettable for LCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
