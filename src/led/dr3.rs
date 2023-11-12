#[doc = "Register `DR3` reader"]
pub type R = crate::R<DR3_SPEC>;
#[doc = "Register `DR3` writer"]
pub type W = crate::W<DR3_SPEC>;
#[doc = "Field `DATA3_A` reader - 8-bit data register"]
pub type DATA3_A_R = crate::BitReader;
#[doc = "Field `DATA3_A` writer - 8-bit data register"]
pub type DATA3_A_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA3_B` reader - 8-bit data register"]
pub type DATA3_B_R = crate::BitReader;
#[doc = "Field `DATA3_B` writer - 8-bit data register"]
pub type DATA3_B_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA3_C` reader - 8-bit data register"]
pub type DATA3_C_R = crate::BitReader;
#[doc = "Field `DATA3_C` writer - 8-bit data register"]
pub type DATA3_C_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA3_D` reader - 8-bit data register"]
pub type DATA3_D_R = crate::BitReader;
#[doc = "Field `DATA3_D` writer - 8-bit data register"]
pub type DATA3_D_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA3_E` reader - 8-bit data register"]
pub type DATA3_E_R = crate::BitReader;
#[doc = "Field `DATA3_E` writer - 8-bit data register"]
pub type DATA3_E_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA3_F` reader - 8-bit data register"]
pub type DATA3_F_R = crate::BitReader;
#[doc = "Field `DATA3_F` writer - 8-bit data register"]
pub type DATA3_F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA3_G` reader - 8-bit data register"]
pub type DATA3_G_R = crate::BitReader;
#[doc = "Field `DATA3_G` writer - 8-bit data register"]
pub type DATA3_G_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA3_DP` reader - 8-bit data register"]
pub type DATA3_DP_R = crate::BitReader;
#[doc = "Field `DATA3_DP` writer - 8-bit data register"]
pub type DATA3_DP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_a(&self) -> DATA3_A_R {
        DATA3_A_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_b(&self) -> DATA3_B_R {
        DATA3_B_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_c(&self) -> DATA3_C_R {
        DATA3_C_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_d(&self) -> DATA3_D_R {
        DATA3_D_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_e(&self) -> DATA3_E_R {
        DATA3_E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_f(&self) -> DATA3_F_R {
        DATA3_F_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_g(&self) -> DATA3_G_R {
        DATA3_G_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_dp(&self) -> DATA3_DP_R {
        DATA3_DP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data3_a(&mut self) -> DATA3_A_W<DR3_SPEC, 0> {
        DATA3_A_W::new(self)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data3_b(&mut self) -> DATA3_B_W<DR3_SPEC, 1> {
        DATA3_B_W::new(self)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data3_c(&mut self) -> DATA3_C_W<DR3_SPEC, 2> {
        DATA3_C_W::new(self)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data3_d(&mut self) -> DATA3_D_W<DR3_SPEC, 3> {
        DATA3_D_W::new(self)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data3_e(&mut self) -> DATA3_E_W<DR3_SPEC, 4> {
        DATA3_E_W::new(self)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data3_f(&mut self) -> DATA3_F_W<DR3_SPEC, 5> {
        DATA3_F_W::new(self)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data3_g(&mut self) -> DATA3_G_W<DR3_SPEC, 6> {
        DATA3_G_W::new(self)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data3_dp(&mut self) -> DATA3_DP_W<DR3_SPEC, 7> {
        DATA3_DP_W::new(self)
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
#[doc = "Data3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR3_SPEC;
impl crate::RegisterSpec for DR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr3::R`](R) reader structure"]
impl crate::Readable for DR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr3::W`](W) writer structure"]
impl crate::Writable for DR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR3 to value 0"]
impl crate::Resettable for DR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
