#[doc = "Register `PUPDR` reader"]
pub type R = crate::R<PUPDR_SPEC>;
#[doc = "Register `PUPDR` writer"]
pub type W = crate::W<PUPDR_SPEC>;
#[doc = "Field `PUPD0` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD0_R = crate::FieldReader;
#[doc = "Field `PUPD0` writer - Port x configuration bits (y = 0..15)"]
pub type PUPD0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PUPD1` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD1_R = crate::FieldReader;
#[doc = "Field `PUPD1` writer - Port x configuration bits (y = 0..15)"]
pub type PUPD1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PUPD2` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD2_R = crate::FieldReader;
#[doc = "Field `PUPD2` writer - Port x configuration bits (y = 0..15)"]
pub type PUPD2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PUPD3` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD3_R = crate::FieldReader;
#[doc = "Field `PUPD3` writer - Port x configuration bits (y = 0..15)"]
pub type PUPD3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PUPD4` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD4_R = crate::FieldReader;
#[doc = "Field `PUPD4` writer - Port x configuration bits (y = 0..15)"]
pub type PUPD4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PUPD5` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD5_R = crate::FieldReader;
#[doc = "Field `PUPD5` writer - Port x configuration bits (y = 0..15)"]
pub type PUPD5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PUPD6` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD6_R = crate::FieldReader;
#[doc = "Field `PUPD6` writer - Port x configuration bits (y = 0..15)"]
pub type PUPD6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PUPD7` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD7_R = crate::FieldReader;
#[doc = "Field `PUPD7` writer - Port x configuration bits (y = 0..15)"]
pub type PUPD7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PUPD8` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD8_R = crate::FieldReader;
#[doc = "Field `PUPD8` writer - Port x configuration bits (y = 0..15)"]
pub type PUPD8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd0(&self) -> PUPD0_R {
        PUPD0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd1(&self) -> PUPD1_R {
        PUPD1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd2(&self) -> PUPD2_R {
        PUPD2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd3(&self) -> PUPD3_R {
        PUPD3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd4(&self) -> PUPD4_R {
        PUPD4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd5(&self) -> PUPD5_R {
        PUPD5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd6(&self) -> PUPD6_R {
        PUPD6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd7(&self) -> PUPD7_R {
        PUPD7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd8(&self) -> PUPD8_R {
        PUPD8_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd0(&mut self) -> PUPD0_W<PUPDR_SPEC, 0> {
        PUPD0_W::new(self)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd1(&mut self) -> PUPD1_W<PUPDR_SPEC, 2> {
        PUPD1_W::new(self)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd2(&mut self) -> PUPD2_W<PUPDR_SPEC, 4> {
        PUPD2_W::new(self)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd3(&mut self) -> PUPD3_W<PUPDR_SPEC, 6> {
        PUPD3_W::new(self)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd4(&mut self) -> PUPD4_W<PUPDR_SPEC, 8> {
        PUPD4_W::new(self)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd5(&mut self) -> PUPD5_W<PUPDR_SPEC, 10> {
        PUPD5_W::new(self)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd6(&mut self) -> PUPD6_W<PUPDR_SPEC, 12> {
        PUPD6_W::new(self)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd7(&mut self) -> PUPD7_W<PUPDR_SPEC, 14> {
        PUPD7_W::new(self)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd8(&mut self) -> PUPD8_W<PUPDR_SPEC, 16> {
        PUPD8_W::new(self)
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
#[doc = "GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pupdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUPDR_SPEC;
impl crate::RegisterSpec for PUPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pupdr::R`](R) reader structure"]
impl crate::Readable for PUPDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pupdr::W`](W) writer structure"]
impl crate::Writable for PUPDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUPDR to value 0"]
impl crate::Resettable for PUPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
