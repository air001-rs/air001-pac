#[doc = "Register `DR0` reader"]
pub type R = crate::R<DR0_SPEC>;
#[doc = "Register `DR0` writer"]
pub type W = crate::W<DR0_SPEC>;
#[doc = "Field `DATA0_A` reader - 8-bit data register"]
pub type DATA0_A_R = crate::BitReader;
#[doc = "Field `DATA0_A` writer - 8-bit data register"]
pub type DATA0_A_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA0_B` reader - 8-bit data register"]
pub type DATA0_B_R = crate::BitReader;
#[doc = "Field `DATA0_B` writer - 8-bit data register"]
pub type DATA0_B_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA0_C` reader - 8-bit data register"]
pub type DATA0_C_R = crate::BitReader;
#[doc = "Field `DATA0_C` writer - 8-bit data register"]
pub type DATA0_C_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA0_D` reader - 8-bit data register"]
pub type DATA0_D_R = crate::BitReader;
#[doc = "Field `DATA0_D` writer - 8-bit data register"]
pub type DATA0_D_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA0_E` reader - 8-bit data register"]
pub type DATA0_E_R = crate::BitReader;
#[doc = "Field `DATA0_E` writer - 8-bit data register"]
pub type DATA0_E_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA0_F` reader - 8-bit data register"]
pub type DATA0_F_R = crate::BitReader;
#[doc = "Field `DATA0_F` writer - 8-bit data register"]
pub type DATA0_F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA0_G` reader - 8-bit data register"]
pub type DATA0_G_R = crate::BitReader;
#[doc = "Field `DATA0_G` writer - 8-bit data register"]
pub type DATA0_G_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA0_DP` reader - 8-bit data register"]
pub type DATA0_DP_R = crate::BitReader;
#[doc = "Field `DATA0_DP` writer - 8-bit data register"]
pub type DATA0_DP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_a(&self) -> DATA0_A_R {
        DATA0_A_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_b(&self) -> DATA0_B_R {
        DATA0_B_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_c(&self) -> DATA0_C_R {
        DATA0_C_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_d(&self) -> DATA0_D_R {
        DATA0_D_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_e(&self) -> DATA0_E_R {
        DATA0_E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_f(&self) -> DATA0_F_R {
        DATA0_F_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_g(&self) -> DATA0_G_R {
        DATA0_G_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_dp(&self) -> DATA0_DP_R {
        DATA0_DP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data0_a(&mut self) -> DATA0_A_W<DR0_SPEC, 0> {
        DATA0_A_W::new(self)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data0_b(&mut self) -> DATA0_B_W<DR0_SPEC, 1> {
        DATA0_B_W::new(self)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data0_c(&mut self) -> DATA0_C_W<DR0_SPEC, 2> {
        DATA0_C_W::new(self)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data0_d(&mut self) -> DATA0_D_W<DR0_SPEC, 3> {
        DATA0_D_W::new(self)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data0_e(&mut self) -> DATA0_E_W<DR0_SPEC, 4> {
        DATA0_E_W::new(self)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data0_f(&mut self) -> DATA0_F_W<DR0_SPEC, 5> {
        DATA0_F_W::new(self)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data0_g(&mut self) -> DATA0_G_W<DR0_SPEC, 6> {
        DATA0_G_W::new(self)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data0_dp(&mut self) -> DATA0_DP_W<DR0_SPEC, 7> {
        DATA0_DP_W::new(self)
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
#[doc = "Data0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR0_SPEC;
impl crate::RegisterSpec for DR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr0::R`](R) reader structure"]
impl crate::Readable for DR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr0::W`](W) writer structure"]
impl crate::Writable for DR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR0 to value 0"]
impl crate::Resettable for DR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
