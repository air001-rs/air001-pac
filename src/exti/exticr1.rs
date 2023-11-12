#[doc = "Register `EXTICR1` reader"]
pub type R = crate::R<EXTICR1_SPEC>;
#[doc = "Register `EXTICR1` writer"]
pub type W = crate::W<EXTICR1_SPEC>;
#[doc = "Field `EXTI0` reader - GPIO port selection"]
pub type EXTI0_R = crate::FieldReader;
#[doc = "Field `EXTI0` writer - GPIO port selection"]
pub type EXTI0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXTI1` reader - GPIO port selection"]
pub type EXTI1_R = crate::FieldReader;
#[doc = "Field `EXTI1` writer - GPIO port selection"]
pub type EXTI1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXTI2` reader - GPIO port selection"]
pub type EXTI2_R = crate::FieldReader;
#[doc = "Field `EXTI2` writer - GPIO port selection"]
pub type EXTI2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXTI3` reader - GPIO port selection"]
pub type EXTI3_R = crate::FieldReader;
#[doc = "Field `EXTI3` writer - GPIO port selection"]
pub type EXTI3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - GPIO port selection"]
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO port selection"]
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPIO port selection"]
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - GPIO port selection"]
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti0(&mut self) -> EXTI0_W<EXTICR1_SPEC, 0> {
        EXTI0_W::new(self)
    }
    #[doc = "Bits 8:9 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti1(&mut self) -> EXTI1_W<EXTICR1_SPEC, 8> {
        EXTI1_W::new(self)
    }
    #[doc = "Bits 16:17 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti2(&mut self) -> EXTI2_W<EXTICR1_SPEC, 16> {
        EXTI2_W::new(self)
    }
    #[doc = "Bits 24:25 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti3(&mut self) -> EXTI3_W<EXTICR1_SPEC, 24> {
        EXTI3_W::new(self)
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
#[doc = "EXTI external interrupt selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR1_SPEC;
impl crate::RegisterSpec for EXTICR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr1::R`](R) reader structure"]
impl crate::Readable for EXTICR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exticr1::W`](W) writer structure"]
impl crate::Writable for EXTICR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTICR1 to value 0"]
impl crate::Resettable for EXTICR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
