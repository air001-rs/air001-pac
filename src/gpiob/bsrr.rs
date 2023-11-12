#[doc = "Register `BSRR` writer"]
pub type W = crate::W<BSRR_SPEC>;
#[doc = "Field `BS0` writer - Port x set bit y (y= 0..15)"]
pub type BS0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS1` writer - Port x set bit y (y= 0..15)"]
pub type BS1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS2` writer - Port x set bit y (y= 0..15)"]
pub type BS2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS3` writer - Port x set bit y (y= 0..15)"]
pub type BS3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS4` writer - Port x set bit y (y= 0..15)"]
pub type BS4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS5` writer - Port x set bit y (y= 0..15)"]
pub type BS5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS6` writer - Port x set bit y (y= 0..15)"]
pub type BS6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS7` writer - Port x set bit y (y= 0..15)"]
pub type BS7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BS8` writer - Port x set bit y (y= 0..15)"]
pub type BS8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR0` writer - Port x set bit y (y= 0..15)"]
pub type BR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR1` writer - Port x reset bit y (y = 0..15)"]
pub type BR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR2` writer - Port x reset bit y (y = 0..15)"]
pub type BR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR3` writer - Port x reset bit y (y = 0..15)"]
pub type BR3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR4` writer - Port x reset bit y (y = 0..15)"]
pub type BR4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR5` writer - Port x reset bit y (y = 0..15)"]
pub type BR5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR6` writer - Port x reset bit y (y = 0..15)"]
pub type BR6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR7` writer - Port x reset bit y (y = 0..15)"]
pub type BR7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR8` writer - Port x reset bit y (y = 0..15)"]
pub type BR8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs0(&mut self) -> BS0_W<BSRR_SPEC, 0> {
        BS0_W::new(self)
    }
    #[doc = "Bit 1 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs1(&mut self) -> BS1_W<BSRR_SPEC, 1> {
        BS1_W::new(self)
    }
    #[doc = "Bit 2 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs2(&mut self) -> BS2_W<BSRR_SPEC, 2> {
        BS2_W::new(self)
    }
    #[doc = "Bit 3 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs3(&mut self) -> BS3_W<BSRR_SPEC, 3> {
        BS3_W::new(self)
    }
    #[doc = "Bit 4 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs4(&mut self) -> BS4_W<BSRR_SPEC, 4> {
        BS4_W::new(self)
    }
    #[doc = "Bit 5 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs5(&mut self) -> BS5_W<BSRR_SPEC, 5> {
        BS5_W::new(self)
    }
    #[doc = "Bit 6 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs6(&mut self) -> BS6_W<BSRR_SPEC, 6> {
        BS6_W::new(self)
    }
    #[doc = "Bit 7 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs7(&mut self) -> BS7_W<BSRR_SPEC, 7> {
        BS7_W::new(self)
    }
    #[doc = "Bit 8 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs8(&mut self) -> BS8_W<BSRR_SPEC, 8> {
        BS8_W::new(self)
    }
    #[doc = "Bit 16 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<BSRR_SPEC, 16> {
        BR0_W::new(self)
    }
    #[doc = "Bit 17 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<BSRR_SPEC, 17> {
        BR1_W::new(self)
    }
    #[doc = "Bit 18 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<BSRR_SPEC, 18> {
        BR2_W::new(self)
    }
    #[doc = "Bit 19 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<BSRR_SPEC, 19> {
        BR3_W::new(self)
    }
    #[doc = "Bit 20 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<BSRR_SPEC, 20> {
        BR4_W::new(self)
    }
    #[doc = "Bit 21 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<BSRR_SPEC, 21> {
        BR5_W::new(self)
    }
    #[doc = "Bit 22 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<BSRR_SPEC, 22> {
        BR6_W::new(self)
    }
    #[doc = "Bit 23 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR7_W<BSRR_SPEC, 23> {
        BR7_W::new(self)
    }
    #[doc = "Bit 24 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR8_W<BSRR_SPEC, 24> {
        BR8_W::new(self)
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
#[doc = "GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSRR_SPEC;
impl crate::RegisterSpec for BSRR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bsrr::W`](W) writer structure"]
impl crate::Writable for BSRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
