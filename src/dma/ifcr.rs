#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IFCR_SPEC>;
#[doc = "Field `CGIF1` writer - Channel 1 Global interrupt clear"]
pub type CGIF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF1` writer - Channel 1 Transfer Complete clear"]
pub type CTCIF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF1` writer - Channel 1 Half Transfer clear"]
pub type CHTIF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF1` writer - Channel 1 Transfer Error clear"]
pub type CTEIF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CGIF2` writer - Channel 2 Global interrupt clear"]
pub type CGIF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF2` writer - Channel 2 Transfer Complete clear"]
pub type CTCIF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF2` writer - Channel 2 Half Transfer clear"]
pub type CHTIF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF2` writer - Channel 2 Transfer Error clear"]
pub type CTEIF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CGIF3` writer - Channel 3 Global interrupt clear"]
pub type CGIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF3` writer - Channel 3 Transfer Complete clear"]
pub type CTCIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF3` writer - Channel 3 Half Transfer clear"]
pub type CHTIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF3` writer - Channel 3 Transfer Error clear"]
pub type CTEIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Channel 1 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif1(&mut self) -> CGIF1_W<IFCR_SPEC, 0> {
        CGIF1_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif1(&mut self) -> CTCIF1_W<IFCR_SPEC, 1> {
        CTCIF1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif1(&mut self) -> CHTIF1_W<IFCR_SPEC, 2> {
        CHTIF1_W::new(self)
    }
    #[doc = "Bit 3 - Channel 1 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif1(&mut self) -> CTEIF1_W<IFCR_SPEC, 3> {
        CTEIF1_W::new(self)
    }
    #[doc = "Bit 4 - Channel 2 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif2(&mut self) -> CGIF2_W<IFCR_SPEC, 4> {
        CGIF2_W::new(self)
    }
    #[doc = "Bit 5 - Channel 2 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif2(&mut self) -> CTCIF2_W<IFCR_SPEC, 5> {
        CTCIF2_W::new(self)
    }
    #[doc = "Bit 6 - Channel 2 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif2(&mut self) -> CHTIF2_W<IFCR_SPEC, 6> {
        CHTIF2_W::new(self)
    }
    #[doc = "Bit 7 - Channel 2 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif2(&mut self) -> CTEIF2_W<IFCR_SPEC, 7> {
        CTEIF2_W::new(self)
    }
    #[doc = "Bit 8 - Channel 3 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif3(&mut self) -> CGIF3_W<IFCR_SPEC, 8> {
        CGIF3_W::new(self)
    }
    #[doc = "Bit 9 - Channel 3 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif3(&mut self) -> CTCIF3_W<IFCR_SPEC, 9> {
        CTCIF3_W::new(self)
    }
    #[doc = "Bit 10 - Channel 3 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif3(&mut self) -> CHTIF3_W<IFCR_SPEC, 10> {
        CHTIF3_W::new(self)
    }
    #[doc = "Bit 11 - Channel 3 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif3(&mut self) -> CTEIF3_W<IFCR_SPEC, 11> {
        CTEIF3_W::new(self)
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
#[doc = "DMA interrupt flag clear register (DMA_IFCR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
