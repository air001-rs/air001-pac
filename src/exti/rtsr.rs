#[doc = "Register `RTSR` reader"]
pub type R = crate::R<RTSR_SPEC>;
#[doc = "Register `RTSR` writer"]
pub type W = crate::W<RTSR_SPEC>;
#[doc = "Field `RT0` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT0_R = crate::BitReader;
#[doc = "Field `RT0` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT1` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT1_R = crate::BitReader;
#[doc = "Field `RT1` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT2` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT2_R = crate::BitReader;
#[doc = "Field `RT2` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT3` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT3_R = crate::BitReader;
#[doc = "Field `RT3` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT4` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT4_R = crate::BitReader;
#[doc = "Field `RT4` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT5` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT5_R = crate::BitReader;
#[doc = "Field `RT5` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT6` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT6_R = crate::BitReader;
#[doc = "Field `RT6` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT7` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT7_R = crate::BitReader;
#[doc = "Field `RT7` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT8` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT8_R = crate::BitReader;
#[doc = "Field `RT8` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT9` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT9_R = crate::BitReader;
#[doc = "Field `RT9` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT10` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT10_R = crate::BitReader;
#[doc = "Field `RT10` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT11` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT11_R = crate::BitReader;
#[doc = "Field `RT11` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT12` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT12_R = crate::BitReader;
#[doc = "Field `RT12` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT13` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT13_R = crate::BitReader;
#[doc = "Field `RT13` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT14` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT14_R = crate::BitReader;
#[doc = "Field `RT14` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT15` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT15_R = crate::BitReader;
#[doc = "Field `RT15` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT16` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT16_R = crate::BitReader;
#[doc = "Field `RT16` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT17` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT17_R = crate::BitReader;
#[doc = "Field `RT17` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT18` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT18_R = crate::BitReader;
#[doc = "Field `RT18` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt0(&self) -> RT0_R {
        RT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt3(&self) -> RT3_R {
        RT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt4(&self) -> RT4_R {
        RT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt5(&self) -> RT5_R {
        RT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt6(&self) -> RT6_R {
        RT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt7(&self) -> RT7_R {
        RT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt8(&self) -> RT8_R {
        RT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt9(&self) -> RT9_R {
        RT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt10(&self) -> RT10_R {
        RT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt11(&self) -> RT11_R {
        RT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt12(&self) -> RT12_R {
        RT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt13(&self) -> RT13_R {
        RT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt14(&self) -> RT14_R {
        RT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt15(&self) -> RT15_R {
        RT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt16(&self) -> RT16_R {
        RT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt17(&self) -> RT17_R {
        RT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt18(&self) -> RT18_R {
        RT18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt0(&mut self) -> RT0_W<RTSR_SPEC, 0> {
        RT0_W::new(self)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt1(&mut self) -> RT1_W<RTSR_SPEC, 1> {
        RT1_W::new(self)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt2(&mut self) -> RT2_W<RTSR_SPEC, 2> {
        RT2_W::new(self)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt3(&mut self) -> RT3_W<RTSR_SPEC, 3> {
        RT3_W::new(self)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt4(&mut self) -> RT4_W<RTSR_SPEC, 4> {
        RT4_W::new(self)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt5(&mut self) -> RT5_W<RTSR_SPEC, 5> {
        RT5_W::new(self)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt6(&mut self) -> RT6_W<RTSR_SPEC, 6> {
        RT6_W::new(self)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt7(&mut self) -> RT7_W<RTSR_SPEC, 7> {
        RT7_W::new(self)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt8(&mut self) -> RT8_W<RTSR_SPEC, 8> {
        RT8_W::new(self)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt9(&mut self) -> RT9_W<RTSR_SPEC, 9> {
        RT9_W::new(self)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt10(&mut self) -> RT10_W<RTSR_SPEC, 10> {
        RT10_W::new(self)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt11(&mut self) -> RT11_W<RTSR_SPEC, 11> {
        RT11_W::new(self)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt12(&mut self) -> RT12_W<RTSR_SPEC, 12> {
        RT12_W::new(self)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt13(&mut self) -> RT13_W<RTSR_SPEC, 13> {
        RT13_W::new(self)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt14(&mut self) -> RT14_W<RTSR_SPEC, 14> {
        RT14_W::new(self)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt15(&mut self) -> RT15_W<RTSR_SPEC, 15> {
        RT15_W::new(self)
    }
    #[doc = "Bit 16 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt16(&mut self) -> RT16_W<RTSR_SPEC, 16> {
        RT16_W::new(self)
    }
    #[doc = "Bit 17 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt17(&mut self) -> RT17_W<RTSR_SPEC, 17> {
        RT17_W::new(self)
    }
    #[doc = "Bit 18 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt18(&mut self) -> RT18_W<RTSR_SPEC, 18> {
        RT18_W::new(self)
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
#[doc = "EXTI rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTSR_SPEC;
impl crate::RegisterSpec for RTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr::R`](R) reader structure"]
impl crate::Readable for RTSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtsr::W`](W) writer structure"]
impl crate::Writable for RTSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTSR to value 0"]
impl crate::Resettable for RTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
