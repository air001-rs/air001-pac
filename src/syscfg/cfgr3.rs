#[doc = "Register `CFGR3` reader"]
pub type R = crate::R<CFGR3_SPEC>;
#[doc = "Register `CFGR3` writer"]
pub type W = crate::W<CFGR3_SPEC>;
#[doc = "Field `DMA1_MAP` reader - DMA channel1 requeset selection"]
pub type DMA1_MAP_R = crate::FieldReader;
#[doc = "Field `DMA1_MAP` writer - DMA channel1 requeset selection"]
pub type DMA1_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `DMA2_MAP` reader - DMA channel2 requeset selection"]
pub type DMA2_MAP_R = crate::FieldReader;
#[doc = "Field `DMA2_MAP` writer - DMA channel2 requeset selection"]
pub type DMA2_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `DMA3_MAP` reader - DMA channel3 requeset selection"]
pub type DMA3_MAP_R = crate::FieldReader;
#[doc = "Field `DMA3_MAP` writer - DMA channel3 requeset selection"]
pub type DMA3_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - DMA channel1 requeset selection"]
    #[inline(always)]
    pub fn dma1_map(&self) -> DMA1_MAP_R {
        DMA1_MAP_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA channel2 requeset selection"]
    #[inline(always)]
    pub fn dma2_map(&self) -> DMA2_MAP_R {
        DMA2_MAP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - DMA channel3 requeset selection"]
    #[inline(always)]
    pub fn dma3_map(&self) -> DMA3_MAP_R {
        DMA3_MAP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA channel1 requeset selection"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_map(&mut self) -> DMA1_MAP_W<CFGR3_SPEC, 0> {
        DMA1_MAP_W::new(self)
    }
    #[doc = "Bits 8:12 - DMA channel2 requeset selection"]
    #[inline(always)]
    #[must_use]
    pub fn dma2_map(&mut self) -> DMA2_MAP_W<CFGR3_SPEC, 8> {
        DMA2_MAP_W::new(self)
    }
    #[doc = "Bits 16:20 - DMA channel3 requeset selection"]
    #[inline(always)]
    #[must_use]
    pub fn dma3_map(&mut self) -> DMA3_MAP_W<CFGR3_SPEC, 16> {
        DMA3_MAP_W::new(self)
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
#[doc = "SYSCFG configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR3_SPEC;
impl crate::RegisterSpec for CFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr3::R`](R) reader structure"]
impl crate::Readable for CFGR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure"]
impl crate::Writable for CFGR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for CFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
