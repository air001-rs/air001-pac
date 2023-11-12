#[doc = "Register `ODR` reader"]
pub type R = crate::R<ODR_SPEC>;
#[doc = "Register `ODR` writer"]
pub type W = crate::W<ODR_SPEC>;
#[doc = "Field `OD0` reader - Port output data (y = 0..15)"]
pub type OD0_R = crate::BitReader;
#[doc = "Field `OD0` writer - Port output data (y = 0..15)"]
pub type OD0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OD1` reader - Port output data (y = 0..15)"]
pub type OD1_R = crate::BitReader;
#[doc = "Field `OD1` writer - Port output data (y = 0..15)"]
pub type OD1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OD2` reader - Port output data (y = 0..15)"]
pub type OD2_R = crate::BitReader;
#[doc = "Field `OD2` writer - Port output data (y = 0..15)"]
pub type OD2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OD3` reader - Port output data (y = 0..15)"]
pub type OD3_R = crate::BitReader;
#[doc = "Field `OD3` writer - Port output data (y = 0..15)"]
pub type OD3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OD4` reader - Port output data (y = 0..15)"]
pub type OD4_R = crate::BitReader;
#[doc = "Field `OD4` writer - Port output data (y = 0..15)"]
pub type OD4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OD5` reader - Port output data (y = 0..15)"]
pub type OD5_R = crate::BitReader;
#[doc = "Field `OD5` writer - Port output data (y = 0..15)"]
pub type OD5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OD6` reader - Port output data (y = 0..15)"]
pub type OD6_R = crate::BitReader;
#[doc = "Field `OD6` writer - Port output data (y = 0..15)"]
pub type OD6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OD7` reader - Port output data (y = 0..15)"]
pub type OD7_R = crate::BitReader;
#[doc = "Field `OD7` writer - Port output data (y = 0..15)"]
pub type OD7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OD8` reader - Port output data (y = 0..15)"]
pub type OD8_R = crate::BitReader;
#[doc = "Field `OD8` writer - Port output data (y = 0..15)"]
pub type OD8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OD9` reader - Port output data (y = 0..15)"]
pub type OD9_R = crate::BitReader;
#[doc = "Field `OD9` writer - Port output data (y = 0..15)"]
pub type OD9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OD10` reader - Port output data (y = 0..15)"]
pub type OD10_R = crate::BitReader;
#[doc = "Field `OD10` writer - Port output data (y = 0..15)"]
pub type OD10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OD11` reader - Port output data (y = 0..15)"]
pub type OD11_R = crate::BitReader;
#[doc = "Field `OD11` writer - Port output data (y = 0..15)"]
pub type OD11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OD12` reader - Port output data (y = 0..15)"]
pub type OD12_R = crate::BitReader;
#[doc = "Field `OD12` writer - Port output data (y = 0..15)"]
pub type OD12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OD13` reader - Port output data (y = 0..15)"]
pub type OD13_R = crate::BitReader;
#[doc = "Field `OD13` writer - Port output data (y = 0..15)"]
pub type OD13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OD14` reader - Port output data (y = 0..15)"]
pub type OD14_R = crate::BitReader;
#[doc = "Field `OD14` writer - Port output data (y = 0..15)"]
pub type OD14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OD15` reader - Port output data (y = 0..15)"]
pub type OD15_R = crate::BitReader;
#[doc = "Field `OD15` writer - Port output data (y = 0..15)"]
pub type OD15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od0(&self) -> OD0_R {
        OD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od1(&self) -> OD1_R {
        OD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od2(&self) -> OD2_R {
        OD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od3(&self) -> OD3_R {
        OD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od4(&self) -> OD4_R {
        OD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od5(&self) -> OD5_R {
        OD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od6(&self) -> OD6_R {
        OD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od7(&self) -> OD7_R {
        OD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od8(&self) -> OD8_R {
        OD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od9(&self) -> OD9_R {
        OD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od10(&self) -> OD10_R {
        OD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od11(&self) -> OD11_R {
        OD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od12(&self) -> OD12_R {
        OD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od13(&self) -> OD13_R {
        OD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od14(&self) -> OD14_R {
        OD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od15(&self) -> OD15_R {
        OD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn od0(&mut self) -> OD0_W<ODR_SPEC, 0> {
        OD0_W::new(self)
    }
    #[doc = "Bit 1 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn od1(&mut self) -> OD1_W<ODR_SPEC, 1> {
        OD1_W::new(self)
    }
    #[doc = "Bit 2 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn od2(&mut self) -> OD2_W<ODR_SPEC, 2> {
        OD2_W::new(self)
    }
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn od3(&mut self) -> OD3_W<ODR_SPEC, 3> {
        OD3_W::new(self)
    }
    #[doc = "Bit 4 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn od4(&mut self) -> OD4_W<ODR_SPEC, 4> {
        OD4_W::new(self)
    }
    #[doc = "Bit 5 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn od5(&mut self) -> OD5_W<ODR_SPEC, 5> {
        OD5_W::new(self)
    }
    #[doc = "Bit 6 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn od6(&mut self) -> OD6_W<ODR_SPEC, 6> {
        OD6_W::new(self)
    }
    #[doc = "Bit 7 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn od7(&mut self) -> OD7_W<ODR_SPEC, 7> {
        OD7_W::new(self)
    }
    #[doc = "Bit 8 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn od8(&mut self) -> OD8_W<ODR_SPEC, 8> {
        OD8_W::new(self)
    }
    #[doc = "Bit 9 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn od9(&mut self) -> OD9_W<ODR_SPEC, 9> {
        OD9_W::new(self)
    }
    #[doc = "Bit 10 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn od10(&mut self) -> OD10_W<ODR_SPEC, 10> {
        OD10_W::new(self)
    }
    #[doc = "Bit 11 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn od11(&mut self) -> OD11_W<ODR_SPEC, 11> {
        OD11_W::new(self)
    }
    #[doc = "Bit 12 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn od12(&mut self) -> OD12_W<ODR_SPEC, 12> {
        OD12_W::new(self)
    }
    #[doc = "Bit 13 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn od13(&mut self) -> OD13_W<ODR_SPEC, 13> {
        OD13_W::new(self)
    }
    #[doc = "Bit 14 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn od14(&mut self) -> OD14_W<ODR_SPEC, 14> {
        OD14_W::new(self)
    }
    #[doc = "Bit 15 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn od15(&mut self) -> OD15_W<ODR_SPEC, 15> {
        OD15_W::new(self)
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
#[doc = "GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODR_SPEC;
impl crate::RegisterSpec for ODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odr::R`](R) reader structure"]
impl crate::Readable for ODR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`odr::W`](W) writer structure"]
impl crate::Writable for ODR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ODR to value 0"]
impl crate::Resettable for ODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
