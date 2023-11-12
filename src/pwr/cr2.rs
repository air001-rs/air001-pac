#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2_SPEC>;
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub type PVDE_R = crate::BitReader;
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PVDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRCSEL` reader - Power voltage detector volatage selection"]
pub type SRCSEL_R = crate::BitReader;
#[doc = "Field `SRCSEL` writer - Power voltage detector volatage selection"]
pub type SRCSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PVDT` reader - Power voltage detector threshold selection"]
pub type PVDT_R = crate::FieldReader;
#[doc = "Field `PVDT` writer - Power voltage detector threshold selection"]
pub type PVDT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `FLTEN` reader - Digital filter enable"]
pub type FLTEN_R = crate::BitReader;
#[doc = "Field `FLTEN` writer - Digital filter enable"]
pub type FLTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT_TIME` reader - Digital filter time configuration"]
pub type FLT_TIME_R = crate::FieldReader;
#[doc = "Field `FLT_TIME` writer - Digital filter time configuration"]
pub type FLT_TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Power voltage detector volatage selection"]
    #[inline(always)]
    pub fn srcsel(&self) -> SRCSEL_R {
        SRCSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Power voltage detector threshold selection"]
    #[inline(always)]
    pub fn pvdt(&self) -> PVDT_R {
        PVDT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Digital filter enable"]
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Digital filter time configuration"]
    #[inline(always)]
    pub fn flt_time(&self) -> FLT_TIME_R {
        FLT_TIME_R::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<CR2_SPEC, 0> {
        PVDE_W::new(self)
    }
    #[doc = "Bit 2 - Power voltage detector volatage selection"]
    #[inline(always)]
    #[must_use]
    pub fn srcsel(&mut self) -> SRCSEL_W<CR2_SPEC, 2> {
        SRCSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Power voltage detector threshold selection"]
    #[inline(always)]
    #[must_use]
    pub fn pvdt(&mut self) -> PVDT_W<CR2_SPEC, 4> {
        PVDT_W::new(self)
    }
    #[doc = "Bit 8 - Digital filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn flten(&mut self) -> FLTEN_W<CR2_SPEC, 8> {
        FLTEN_W::new(self)
    }
    #[doc = "Bits 9:11 - Digital filter time configuration"]
    #[inline(always)]
    #[must_use]
    pub fn flt_time(&mut self) -> FLT_TIME_W<CR2_SPEC, 9> {
        FLT_TIME_W::new(self)
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
#[doc = "Power control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0x0500"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0500;
}
