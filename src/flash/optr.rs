#[doc = "Register `OPTR` reader"]
pub type R = crate::R<OPTR_SPEC>;
#[doc = "Register `OPTR` writer"]
pub type W = crate::W<OPTR_SPEC>;
#[doc = "Field `RDP` reader - Read Protection"]
pub type RDP_R = crate::FieldReader;
#[doc = "Field `RDP` writer - Read Protection"]
pub type RDP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `BOREN` reader - BOR reset Level"]
pub type BOREN_R = crate::BitReader;
#[doc = "Field `BOREN` writer - BOR reset Level"]
pub type BOREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BORF_LEV` reader - These bits contain the VDD supply level threshold that activates the reset"]
pub type BORF_LEV_R = crate::FieldReader;
#[doc = "Field `BORF_LEV` writer - These bits contain the VDD supply level threshold that activates the reset"]
pub type BORF_LEV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `IDWG_SW` reader - Independent watchdog selection"]
pub type IDWG_SW_R = crate::BitReader;
#[doc = "Field `IDWG_SW` writer - Independent watchdog selection"]
pub type IDWG_SW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WWDG_SW` reader - Window watchdog selection"]
pub type WWDG_SW_R = crate::BitReader;
#[doc = "Field `WWDG_SW` writer - Window watchdog selection"]
pub type WWDG_SW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NRST_MODE` reader - NRST_MODE"]
pub type NRST_MODE_R = crate::BitReader;
#[doc = "Field `NRST_MODE` writer - NRST_MODE"]
pub type NRST_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `nBOOT1` reader - Boot configuration"]
pub type N_BOOT1_R = crate::BitReader;
#[doc = "Field `nBOOT1` writer - Boot configuration"]
pub type N_BOOT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - Read Protection"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - BOR reset Level"]
    #[inline(always)]
    pub fn boren(&self) -> BOREN_R {
        BOREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - These bits contain the VDD supply level threshold that activates the reset"]
    #[inline(always)]
    pub fn borf_lev(&self) -> BORF_LEV_R {
        BORF_LEV_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Independent watchdog selection"]
    #[inline(always)]
    pub fn idwg_sw(&self) -> IDWG_SW_R {
        IDWG_SW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Window watchdog selection"]
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NRST_MODE"]
    #[inline(always)]
    pub fn nrst_mode(&self) -> NRST_MODE_R {
        NRST_MODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Boot configuration"]
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Protection"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<OPTR_SPEC, 0> {
        RDP_W::new(self)
    }
    #[doc = "Bit 8 - BOR reset Level"]
    #[inline(always)]
    #[must_use]
    pub fn boren(&mut self) -> BOREN_W<OPTR_SPEC, 8> {
        BOREN_W::new(self)
    }
    #[doc = "Bits 9:11 - These bits contain the VDD supply level threshold that activates the reset"]
    #[inline(always)]
    #[must_use]
    pub fn borf_lev(&mut self) -> BORF_LEV_W<OPTR_SPEC, 9> {
        BORF_LEV_W::new(self)
    }
    #[doc = "Bit 12 - Independent watchdog selection"]
    #[inline(always)]
    #[must_use]
    pub fn idwg_sw(&mut self) -> IDWG_SW_W<OPTR_SPEC, 12> {
        IDWG_SW_W::new(self)
    }
    #[doc = "Bit 13 - Window watchdog selection"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<OPTR_SPEC, 13> {
        WWDG_SW_W::new(self)
    }
    #[doc = "Bit 14 - NRST_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_mode(&mut self) -> NRST_MODE_W<OPTR_SPEC, 14> {
        NRST_MODE_W::new(self)
    }
    #[doc = "Bit 15 - Boot configuration"]
    #[inline(always)]
    #[must_use]
    pub fn n_boot1(&mut self) -> N_BOOT1_W<OPTR_SPEC, 15> {
        N_BOOT1_W::new(self)
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
#[doc = "Flash option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTR_SPEC;
impl crate::RegisterSpec for OPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optr::R`](R) reader structure"]
impl crate::Readable for OPTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`optr::W`](W) writer structure"]
impl crate::Writable for OPTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTR to value 0x4f55_b0aa"]
impl crate::Resettable for OPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4f55_b0aa;
}
