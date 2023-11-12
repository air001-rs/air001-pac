#[doc = "Register `BRR` writer"]
pub type W = crate::W<BRR_SPEC>;
#[doc = "Field `BR0` writer - Port Reset bit"]
pub type BR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR1` writer - Port Reset bit"]
pub type BR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR2` writer - Port Reset bit"]
pub type BR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR3` writer - Port Reset bit"]
pub type BR3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR4` writer - Port Reset bit"]
pub type BR4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR5` writer - Port Reset bit"]
pub type BR5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR6` writer - Port Reset bit"]
pub type BR6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR7` writer - Port Reset bit"]
pub type BR7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR8` writer - Port Reset bit"]
pub type BR8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR9` writer - Port Reset bit"]
pub type BR9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR10` writer - Port Reset bit"]
pub type BR10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR11` writer - Port Reset bit"]
pub type BR11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR12` writer - Port Reset bit"]
pub type BR12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR13` writer - Port Reset bit"]
pub type BR13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR14` writer - Port Reset bit"]
pub type BR14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BR15` writer - Port Reset bit"]
pub type BR15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<BRR_SPEC, 0> {
        BR0_W::new(self)
    }
    #[doc = "Bit 1 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<BRR_SPEC, 1> {
        BR1_W::new(self)
    }
    #[doc = "Bit 2 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<BRR_SPEC, 2> {
        BR2_W::new(self)
    }
    #[doc = "Bit 3 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<BRR_SPEC, 3> {
        BR3_W::new(self)
    }
    #[doc = "Bit 4 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<BRR_SPEC, 4> {
        BR4_W::new(self)
    }
    #[doc = "Bit 5 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<BRR_SPEC, 5> {
        BR5_W::new(self)
    }
    #[doc = "Bit 6 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<BRR_SPEC, 6> {
        BR6_W::new(self)
    }
    #[doc = "Bit 7 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR7_W<BRR_SPEC, 7> {
        BR7_W::new(self)
    }
    #[doc = "Bit 8 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR8_W<BRR_SPEC, 8> {
        BR8_W::new(self)
    }
    #[doc = "Bit 9 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> BR9_W<BRR_SPEC, 9> {
        BR9_W::new(self)
    }
    #[doc = "Bit 10 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> BR10_W<BRR_SPEC, 10> {
        BR10_W::new(self)
    }
    #[doc = "Bit 11 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> BR11_W<BRR_SPEC, 11> {
        BR11_W::new(self)
    }
    #[doc = "Bit 12 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> BR12_W<BRR_SPEC, 12> {
        BR12_W::new(self)
    }
    #[doc = "Bit 13 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR13_W<BRR_SPEC, 13> {
        BR13_W::new(self)
    }
    #[doc = "Bit 14 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR14_W<BRR_SPEC, 14> {
        BR14_W::new(self)
    }
    #[doc = "Bit 15 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> BR15_W<BRR_SPEC, 15> {
        BR15_W::new(self)
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
#[doc = "port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
