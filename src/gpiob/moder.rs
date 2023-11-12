#[doc = "Register `MODER` reader"]
pub type R = crate::R<MODER_SPEC>;
#[doc = "Register `MODER` writer"]
pub type W = crate::W<MODER_SPEC>;
#[doc = "Field `MODE0` reader - Port x configuration bits (y = 0..15)"]
pub type MODE0_R = crate::FieldReader;
#[doc = "Field `MODE0` writer - Port x configuration bits (y = 0..15)"]
pub type MODE0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE1` reader - Port x configuration bits (y = 0..15)"]
pub type MODE1_R = crate::FieldReader;
#[doc = "Field `MODE1` writer - Port x configuration bits (y = 0..15)"]
pub type MODE1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE2` reader - Port x configuration bits (y = 0..15)"]
pub type MODE2_R = crate::FieldReader;
#[doc = "Field `MODE2` writer - Port x configuration bits (y = 0..15)"]
pub type MODE2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE3` reader - Port x configuration bits (y = 0..15)"]
pub type MODE3_R = crate::FieldReader;
#[doc = "Field `MODE3` writer - Port x configuration bits (y = 0..15)"]
pub type MODE3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE4` reader - Port x configuration bits (y = 0..15)"]
pub type MODE4_R = crate::FieldReader;
#[doc = "Field `MODE4` writer - Port x configuration bits (y = 0..15)"]
pub type MODE4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE5` reader - Port x configuration bits (y = 0..15)"]
pub type MODE5_R = crate::FieldReader;
#[doc = "Field `MODE5` writer - Port x configuration bits (y = 0..15)"]
pub type MODE5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE6` reader - Port x configuration bits (y = 0..15)"]
pub type MODE6_R = crate::FieldReader;
#[doc = "Field `MODE6` writer - Port x configuration bits (y = 0..15)"]
pub type MODE6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE7` reader - Port x configuration bits (y = 0..15)"]
pub type MODE7_R = crate::FieldReader;
#[doc = "Field `MODE7` writer - Port x configuration bits (y = 0..15)"]
pub type MODE7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE8` reader - Port x configuration bits (y = 0..15)"]
pub type MODE8_R = crate::FieldReader;
#[doc = "Field `MODE8` writer - Port x configuration bits (y = 0..15)"]
pub type MODE8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> MODE0_W<MODER_SPEC, 0> {
        MODE0_W::new(self)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<MODER_SPEC, 2> {
        MODE1_W::new(self)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<MODER_SPEC, 4> {
        MODE2_W::new(self)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> MODE3_W<MODER_SPEC, 6> {
        MODE3_W::new(self)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn mode4(&mut self) -> MODE4_W<MODER_SPEC, 8> {
        MODE4_W::new(self)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn mode5(&mut self) -> MODE5_W<MODER_SPEC, 10> {
        MODE5_W::new(self)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn mode6(&mut self) -> MODE6_W<MODER_SPEC, 12> {
        MODE6_W::new(self)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn mode7(&mut self) -> MODE7_W<MODER_SPEC, 14> {
        MODE7_W::new(self)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn mode8(&mut self) -> MODE8_W<MODER_SPEC, 16> {
        MODE8_W::new(self)
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
#[doc = "GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moder::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODER_SPEC;
impl crate::RegisterSpec for MODER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moder::R`](R) reader structure"]
impl crate::Readable for MODER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`moder::W`](W) writer structure"]
impl crate::Writable for MODER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODER to value 0xffff_ffff"]
impl crate::Resettable for MODER_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
