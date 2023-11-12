#[doc = "Register `DR2` reader"]
pub type R = crate::R<DR2_SPEC>;
#[doc = "Register `DR2` writer"]
pub type W = crate::W<DR2_SPEC>;
#[doc = "Field `DATA2_A` reader - 8-bit data register"]
pub type DATA2_A_R = crate::BitReader;
#[doc = "Field `DATA2_A` writer - 8-bit data register"]
pub type DATA2_A_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA2_B` reader - 8-bit data register"]
pub type DATA2_B_R = crate::BitReader;
#[doc = "Field `DATA2_B` writer - 8-bit data register"]
pub type DATA2_B_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA2_C` reader - 8-bit data register"]
pub type DATA2_C_R = crate::BitReader;
#[doc = "Field `DATA2_C` writer - 8-bit data register"]
pub type DATA2_C_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA2_D` reader - 8-bit data register"]
pub type DATA2_D_R = crate::BitReader;
#[doc = "Field `DATA2_D` writer - 8-bit data register"]
pub type DATA2_D_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA2_E` reader - 8-bit data register"]
pub type DATA2_E_R = crate::BitReader;
#[doc = "Field `DATA2_E` writer - 8-bit data register"]
pub type DATA2_E_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA2_F` reader - 8-bit data register"]
pub type DATA2_F_R = crate::BitReader;
#[doc = "Field `DATA2_F` writer - 8-bit data register"]
pub type DATA2_F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA2_G` reader - 8-bit data register"]
pub type DATA2_G_R = crate::BitReader;
#[doc = "Field `DATA2_G` writer - 8-bit data register"]
pub type DATA2_G_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATA2_DP` reader - 8-bit data register"]
pub type DATA2_DP_R = crate::BitReader;
#[doc = "Field `DATA2_DP` writer - 8-bit data register"]
pub type DATA2_DP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_a(&self) -> DATA2_A_R {
        DATA2_A_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_b(&self) -> DATA2_B_R {
        DATA2_B_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_c(&self) -> DATA2_C_R {
        DATA2_C_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_d(&self) -> DATA2_D_R {
        DATA2_D_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_e(&self) -> DATA2_E_R {
        DATA2_E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_f(&self) -> DATA2_F_R {
        DATA2_F_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_g(&self) -> DATA2_G_R {
        DATA2_G_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_dp(&self) -> DATA2_DP_R {
        DATA2_DP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data2_a(&mut self) -> DATA2_A_W<DR2_SPEC, 0> {
        DATA2_A_W::new(self)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data2_b(&mut self) -> DATA2_B_W<DR2_SPEC, 1> {
        DATA2_B_W::new(self)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data2_c(&mut self) -> DATA2_C_W<DR2_SPEC, 2> {
        DATA2_C_W::new(self)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data2_d(&mut self) -> DATA2_D_W<DR2_SPEC, 3> {
        DATA2_D_W::new(self)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data2_e(&mut self) -> DATA2_E_W<DR2_SPEC, 4> {
        DATA2_E_W::new(self)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data2_f(&mut self) -> DATA2_F_W<DR2_SPEC, 5> {
        DATA2_F_W::new(self)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data2_g(&mut self) -> DATA2_G_W<DR2_SPEC, 6> {
        DATA2_G_W::new(self)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    #[must_use]
    pub fn data2_dp(&mut self) -> DATA2_DP_W<DR2_SPEC, 7> {
        DATA2_DP_W::new(self)
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
#[doc = "Data2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR2_SPEC;
impl crate::RegisterSpec for DR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr2::R`](R) reader structure"]
impl crate::Readable for DR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr2::W`](W) writer structure"]
impl crate::Writable for DR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR2 to value 0"]
impl crate::Resettable for DR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
