#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSR_SPEC>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSR_SPEC>;
#[doc = "Field `COMP_EN` reader - COMP enable bit"]
pub type COMP_EN_R = crate::BitReader;
#[doc = "Field `COMP_EN` writer - COMP enable bit"]
pub type COMP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCALER_EN` reader - SCALER enable bit"]
pub type SCALER_EN_R = crate::BitReader;
#[doc = "Field `SCALER_EN` writer - SCALER enable bit"]
pub type SCALER_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INMSEL` reader - Comparator signal selector for inverting input INM"]
pub type INMSEL_R = crate::FieldReader;
#[doc = "Field `INMSEL` writer - Comparator signal selector for inverting input INM"]
pub type INMSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `INPSEL` reader - Comparator signal selector for non-inverting input"]
pub type INPSEL_R = crate::FieldReader;
#[doc = "Field `INPSEL` writer - Comparator signal selector for non-inverting input"]
pub type INPSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `WINMODE` reader - Comparator non-inverting input selector for window mode"]
pub type WINMODE_R = crate::BitReader;
#[doc = "Field `WINMODE` writer - Comparator non-inverting input selector for window mode"]
pub type WINMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POLARITY` reader - Comparator polarity selector"]
pub type POLARITY_R = crate::BitReader;
#[doc = "Field `POLARITY` writer - Comparator polarity selector"]
pub type POLARITY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HYST` reader - Comparator hysteresis enable selector"]
pub type HYST_R = crate::BitReader;
#[doc = "Field `HYST` writer - Comparator hysteresis enable selector"]
pub type HYST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWRMODE` reader - Comparator power mode selector"]
pub type PWRMODE_R = crate::FieldReader;
#[doc = "Field `PWRMODE` writer - Comparator power mode selector"]
pub type PWRMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `COMP_OUT` reader - Comparator output status"]
pub type COMP_OUT_R = crate::BitReader;
#[doc = "Field `COMP_OUT` writer - Comparator output status"]
pub type COMP_OUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK` reader - CSR register lock"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - CSR register lock"]
pub type LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - COMP enable bit"]
    #[inline(always)]
    pub fn comp_en(&self) -> COMP_EN_R {
        COMP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCALER enable bit"]
    #[inline(always)]
    pub fn scaler_en(&self) -> SCALER_EN_R {
        SCALER_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Comparator signal selector for inverting input INM"]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Comparator signal selector for non-inverting input"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Comparator non-inverting input selector for window mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator polarity selector"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Comparator hysteresis enable selector"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Comparator power mode selector"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 30 - Comparator output status"]
    #[inline(always)]
    pub fn comp_out(&self) -> COMP_OUT_R {
        COMP_OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CSR register lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COMP enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp_en(&mut self) -> COMP_EN_W<CSR_SPEC, 0> {
        COMP_EN_W::new(self)
    }
    #[doc = "Bit 1 - SCALER enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn scaler_en(&mut self) -> SCALER_EN_W<CSR_SPEC, 1> {
        SCALER_EN_W::new(self)
    }
    #[doc = "Bits 4:7 - Comparator signal selector for inverting input INM"]
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> INMSEL_W<CSR_SPEC, 4> {
        INMSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Comparator signal selector for non-inverting input"]
    #[inline(always)]
    #[must_use]
    pub fn inpsel(&mut self) -> INPSEL_W<CSR_SPEC, 8> {
        INPSEL_W::new(self)
    }
    #[doc = "Bit 11 - Comparator non-inverting input selector for window mode"]
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WINMODE_W<CSR_SPEC, 11> {
        WINMODE_W::new(self)
    }
    #[doc = "Bit 15 - Comparator polarity selector"]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<CSR_SPEC, 15> {
        POLARITY_W::new(self)
    }
    #[doc = "Bit 16 - Comparator hysteresis enable selector"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<CSR_SPEC, 16> {
        HYST_W::new(self)
    }
    #[doc = "Bits 18:19 - Comparator power mode selector"]
    #[inline(always)]
    #[must_use]
    pub fn pwrmode(&mut self) -> PWRMODE_W<CSR_SPEC, 18> {
        PWRMODE_W::new(self)
    }
    #[doc = "Bit 30 - Comparator output status"]
    #[inline(always)]
    #[must_use]
    pub fn comp_out(&mut self) -> COMP_OUT_W<CSR_SPEC, 30> {
        COMP_OUT_W::new(self)
    }
    #[doc = "Bit 31 - CSR register lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<CSR_SPEC, 31> {
        LOCK_W::new(self)
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
#[doc = "COMP control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
