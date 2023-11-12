#[doc = "Register `CCIPR` reader"]
pub type R = crate::R<CCIPR_SPEC>;
#[doc = "Register `CCIPR` writer"]
pub type W = crate::W<CCIPR_SPEC>;
#[doc = "Field `PVDSEL` reader - PVD detect clock source selection"]
pub type PVDSEL_R = crate::BitReader;
#[doc = "Field `PVDSEL` writer - PVD detect clock source selection"]
pub type PVDSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP1SEL` reader - COMP1 clock source selection"]
pub type COMP1SEL_R = crate::BitReader;
#[doc = "Field `COMP1SEL` writer - COMP1 clock source selection"]
pub type COMP1SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP2SEL` reader - COMP2 clock source selection"]
pub type COMP2SEL_R = crate::BitReader;
#[doc = "Field `COMP2SEL` writer - COMP2 clock source selection"]
pub type COMP2SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM1SEL` reader - LPTIM1 clock source selection"]
pub type LPTIM1SEL_R = crate::FieldReader;
#[doc = "Field `LPTIM1SEL` writer - LPTIM1 clock source selection"]
pub type LPTIM1SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 7 - PVD detect clock source selection"]
    #[inline(always)]
    pub fn pvdsel(&self) -> PVDSEL_R {
        PVDSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - COMP1 clock source selection"]
    #[inline(always)]
    pub fn comp1sel(&self) -> COMP1SEL_R {
        COMP1SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - COMP2 clock source selection"]
    #[inline(always)]
    pub fn comp2sel(&self) -> COMP2SEL_R {
        COMP2SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 18:19 - LPTIM1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - PVD detect clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn pvdsel(&mut self) -> PVDSEL_W<CCIPR_SPEC, 7> {
        PVDSEL_W::new(self)
    }
    #[doc = "Bit 8 - COMP1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp1sel(&mut self) -> COMP1SEL_W<CCIPR_SPEC, 8> {
        COMP1SEL_W::new(self)
    }
    #[doc = "Bit 9 - COMP2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2sel(&mut self) -> COMP2SEL_W<CCIPR_SPEC, 9> {
        COMP2SEL_W::new(self)
    }
    #[doc = "Bits 18:19 - LPTIM1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<CCIPR_SPEC, 18> {
        LPTIM1SEL_W::new(self)
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
#[doc = "Peripherals independent clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR_SPEC;
impl crate::RegisterSpec for CCIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr::R`](R) reader structure"]
impl crate::Readable for CCIPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccipr::W`](W) writer structure"]
impl crate::Writable for CCIPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCIPR to value 0"]
impl crate::Resettable for CCIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
