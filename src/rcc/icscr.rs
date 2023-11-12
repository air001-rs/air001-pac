#[doc = "Register `ICSCR` reader"]
pub type R = crate::R<ICSCR_SPEC>;
#[doc = "Register `ICSCR` writer"]
pub type W = crate::W<ICSCR_SPEC>;
#[doc = "Field `HSI_TRIM` reader - HSI clock trimming"]
pub type HSI_TRIM_R = crate::FieldReader<u16>;
#[doc = "Field `HSI_TRIM` writer - HSI clock trimming"]
pub type HSI_TRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
#[doc = "Field `HSI_FS` reader - HSI frequency selection"]
pub type HSI_FS_R = crate::FieldReader;
#[doc = "Field `HSI_FS` writer - HSI frequency selection"]
pub type HSI_FS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `LSI_TRIM` reader - LSI clock trimming"]
pub type LSI_TRIM_R = crate::FieldReader<u16>;
#[doc = "Field `LSI_TRIM` writer - LSI clock trimming"]
pub type LSI_TRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `LSI_STARTUP` reader - LSI startup time"]
pub type LSI_STARTUP_R = crate::FieldReader;
#[doc = "Field `LSI_STARTUP` writer - LSI startup time"]
pub type LSI_STARTUP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:12 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsi_trim(&self) -> HSI_TRIM_R {
        HSI_TRIM_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:15 - HSI frequency selection"]
    #[inline(always)]
    pub fn hsi_fs(&self) -> HSI_FS_R {
        HSI_FS_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:24 - LSI clock trimming"]
    #[inline(always)]
    pub fn lsi_trim(&self) -> LSI_TRIM_R {
        LSI_TRIM_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 26:27 - LSI startup time"]
    #[inline(always)]
    pub fn lsi_startup(&self) -> LSI_STARTUP_R {
        LSI_STARTUP_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - HSI clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn hsi_trim(&mut self) -> HSI_TRIM_W<ICSCR_SPEC, 0> {
        HSI_TRIM_W::new(self)
    }
    #[doc = "Bits 13:15 - HSI frequency selection"]
    #[inline(always)]
    #[must_use]
    pub fn hsi_fs(&mut self) -> HSI_FS_W<ICSCR_SPEC, 13> {
        HSI_FS_W::new(self)
    }
    #[doc = "Bits 16:24 - LSI clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn lsi_trim(&mut self) -> LSI_TRIM_W<ICSCR_SPEC, 16> {
        LSI_TRIM_W::new(self)
    }
    #[doc = "Bits 26:27 - LSI startup time"]
    #[inline(always)]
    #[must_use]
    pub fn lsi_startup(&mut self) -> LSI_STARTUP_W<ICSCR_SPEC, 26> {
        LSI_STARTUP_W::new(self)
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
#[doc = "Internal clock sources calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICSCR_SPEC;
impl crate::RegisterSpec for ICSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icscr::R`](R) reader structure"]
impl crate::Readable for ICSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icscr::W`](W) writer structure"]
impl crate::Writable for ICSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICSCR to value 0x1000_0000"]
impl crate::Resettable for ICSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0000;
}
