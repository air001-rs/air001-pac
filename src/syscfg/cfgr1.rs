#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1_SPEC>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1_SPEC>;
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits"]
pub type MEM_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub type MEM_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `I2C_PA2_ANF` reader - Analog filter enable control driving capability activation bits PA2"]
pub type I2C_PA2_ANF_R = crate::BitReader;
#[doc = "Field `I2C_PA2_ANF` writer - Analog filter enable control driving capability activation bits PA2"]
pub type I2C_PA2_ANF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_PA3_ANF` reader - Analog filter enable control driving capability activation bits PA3"]
pub type I2C_PA3_ANF_R = crate::BitReader;
#[doc = "Field `I2C_PA3_ANF` writer - Analog filter enable control driving capability activation bits PA3"]
pub type I2C_PA3_ANF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_PA7_ANF` reader - Analog filter enable control driving capability activation bits PA7"]
pub type I2C_PA7_ANF_R = crate::BitReader;
#[doc = "Field `I2C_PA7_ANF` writer - Analog filter enable control driving capability activation bits PA7"]
pub type I2C_PA7_ANF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_PA8_ANF` reader - Analog filter enable control driving capability activation bits PA8"]
pub type I2C_PA8_ANF_R = crate::BitReader;
#[doc = "Field `I2C_PA8_ANF` writer - Analog filter enable control driving capability activation bits PA8"]
pub type I2C_PA8_ANF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_PA9_ANF` reader - Analog filter enable control driving capability activation bits PA9"]
pub type I2C_PA9_ANF_R = crate::BitReader;
#[doc = "Field `I2C_PA9_ANF` writer - Analog filter enable control driving capability activation bits PA9"]
pub type I2C_PA9_ANF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_PA10_ANF` reader - Analog filter enable control driving capability activation bits PA10"]
pub type I2C_PA10_ANF_R = crate::BitReader;
#[doc = "Field `I2C_PA10_ANF` writer - Analog filter enable control driving capability activation bits PA10"]
pub type I2C_PA10_ANF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_PA11_ANF` reader - Analog filter enable control driving capability activation bits PA11"]
pub type I2C_PA11_ANF_R = crate::BitReader;
#[doc = "Field `I2C_PA11_ANF` writer - Analog filter enable control driving capability activation bits PA11"]
pub type I2C_PA11_ANF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_PA12_ANF` reader - Analog filter enable control driving capability activation bits PA12"]
pub type I2C_PA12_ANF_R = crate::BitReader;
#[doc = "Field `I2C_PA12_ANF` writer - Analog filter enable control driving capability activation bits PA12"]
pub type I2C_PA12_ANF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_PB6_ANF` reader - Analog filter enable control driving capability activation bits PB6"]
pub type I2C_PB6_ANF_R = crate::BitReader;
#[doc = "Field `I2C_PB6_ANF` writer - Analog filter enable control driving capability activation bits PB6"]
pub type I2C_PB6_ANF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_PB7_ANF` reader - Analog filter enable control driving capability activation bits PB7"]
pub type I2C_PB7_ANF_R = crate::BitReader;
#[doc = "Field `I2C_PB7_ANF` writer - Analog filter enable control driving capability activation bits PB7"]
pub type I2C_PB7_ANF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_PB8_ANF` reader - Analog filter enable control driving capability activation bits PB8"]
pub type I2C_PB8_ANF_R = crate::BitReader;
#[doc = "Field `I2C_PB8_ANF` writer - Analog filter enable control driving capability activation bits PB8"]
pub type I2C_PB8_ANF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_PF0_ANF` reader - Analog filter enable control driving capability activation bits PF0"]
pub type I2C_PF0_ANF_R = crate::BitReader;
#[doc = "Field `I2C_PF0_ANF` writer - Analog filter enable control driving capability activation bits PF0"]
pub type I2C_PF0_ANF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_PF1_ANF` reader - Analog filter enable control driving capability activation bits PF1"]
pub type I2C_PF1_ANF_R = crate::BitReader;
#[doc = "Field `I2C_PF1_ANF` writer - Analog filter enable control driving capability activation bits PF1"]
pub type I2C_PF1_ANF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 18 - Analog filter enable control driving capability activation bits PA2"]
    #[inline(always)]
    pub fn i2c_pa2_anf(&self) -> I2C_PA2_ANF_R {
        I2C_PA2_ANF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Analog filter enable control driving capability activation bits PA3"]
    #[inline(always)]
    pub fn i2c_pa3_anf(&self) -> I2C_PA3_ANF_R {
        I2C_PA3_ANF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Analog filter enable control driving capability activation bits PA7"]
    #[inline(always)]
    pub fn i2c_pa7_anf(&self) -> I2C_PA7_ANF_R {
        I2C_PA7_ANF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Analog filter enable control driving capability activation bits PA8"]
    #[inline(always)]
    pub fn i2c_pa8_anf(&self) -> I2C_PA8_ANF_R {
        I2C_PA8_ANF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Analog filter enable control driving capability activation bits PA9"]
    #[inline(always)]
    pub fn i2c_pa9_anf(&self) -> I2C_PA9_ANF_R {
        I2C_PA9_ANF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog filter enable control driving capability activation bits PA10"]
    #[inline(always)]
    pub fn i2c_pa10_anf(&self) -> I2C_PA10_ANF_R {
        I2C_PA10_ANF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Analog filter enable control driving capability activation bits PA11"]
    #[inline(always)]
    pub fn i2c_pa11_anf(&self) -> I2C_PA11_ANF_R {
        I2C_PA11_ANF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Analog filter enable control driving capability activation bits PA12"]
    #[inline(always)]
    pub fn i2c_pa12_anf(&self) -> I2C_PA12_ANF_R {
        I2C_PA12_ANF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Analog filter enable control driving capability activation bits PB6"]
    #[inline(always)]
    pub fn i2c_pb6_anf(&self) -> I2C_PB6_ANF_R {
        I2C_PB6_ANF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Analog filter enable control driving capability activation bits PB7"]
    #[inline(always)]
    pub fn i2c_pb7_anf(&self) -> I2C_PB7_ANF_R {
        I2C_PB7_ANF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Analog filter enable control driving capability activation bits PB8"]
    #[inline(always)]
    pub fn i2c_pb8_anf(&self) -> I2C_PB8_ANF_R {
        I2C_PB8_ANF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Analog filter enable control driving capability activation bits PF0"]
    #[inline(always)]
    pub fn i2c_pf0_anf(&self) -> I2C_PF0_ANF_R {
        I2C_PF0_ANF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Analog filter enable control driving capability activation bits PF1"]
    #[inline(always)]
    pub fn i2c_pf1_anf(&self) -> I2C_PF1_ANF_R {
        I2C_PF1_ANF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<CFGR1_SPEC, 0> {
        MEM_MODE_W::new(self)
    }
    #[doc = "Bit 18 - Analog filter enable control driving capability activation bits PA2"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pa2_anf(&mut self) -> I2C_PA2_ANF_W<CFGR1_SPEC, 18> {
        I2C_PA2_ANF_W::new(self)
    }
    #[doc = "Bit 19 - Analog filter enable control driving capability activation bits PA3"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pa3_anf(&mut self) -> I2C_PA3_ANF_W<CFGR1_SPEC, 19> {
        I2C_PA3_ANF_W::new(self)
    }
    #[doc = "Bit 20 - Analog filter enable control driving capability activation bits PA7"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pa7_anf(&mut self) -> I2C_PA7_ANF_W<CFGR1_SPEC, 20> {
        I2C_PA7_ANF_W::new(self)
    }
    #[doc = "Bit 21 - Analog filter enable control driving capability activation bits PA8"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pa8_anf(&mut self) -> I2C_PA8_ANF_W<CFGR1_SPEC, 21> {
        I2C_PA8_ANF_W::new(self)
    }
    #[doc = "Bit 22 - Analog filter enable control driving capability activation bits PA9"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pa9_anf(&mut self) -> I2C_PA9_ANF_W<CFGR1_SPEC, 22> {
        I2C_PA9_ANF_W::new(self)
    }
    #[doc = "Bit 23 - Analog filter enable control driving capability activation bits PA10"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pa10_anf(&mut self) -> I2C_PA10_ANF_W<CFGR1_SPEC, 23> {
        I2C_PA10_ANF_W::new(self)
    }
    #[doc = "Bit 24 - Analog filter enable control driving capability activation bits PA11"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pa11_anf(&mut self) -> I2C_PA11_ANF_W<CFGR1_SPEC, 24> {
        I2C_PA11_ANF_W::new(self)
    }
    #[doc = "Bit 25 - Analog filter enable control driving capability activation bits PA12"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pa12_anf(&mut self) -> I2C_PA12_ANF_W<CFGR1_SPEC, 25> {
        I2C_PA12_ANF_W::new(self)
    }
    #[doc = "Bit 26 - Analog filter enable control driving capability activation bits PB6"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb6_anf(&mut self) -> I2C_PB6_ANF_W<CFGR1_SPEC, 26> {
        I2C_PB6_ANF_W::new(self)
    }
    #[doc = "Bit 27 - Analog filter enable control driving capability activation bits PB7"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb7_anf(&mut self) -> I2C_PB7_ANF_W<CFGR1_SPEC, 27> {
        I2C_PB7_ANF_W::new(self)
    }
    #[doc = "Bit 28 - Analog filter enable control driving capability activation bits PB8"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb8_anf(&mut self) -> I2C_PB8_ANF_W<CFGR1_SPEC, 28> {
        I2C_PB8_ANF_W::new(self)
    }
    #[doc = "Bit 29 - Analog filter enable control driving capability activation bits PF0"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pf0_anf(&mut self) -> I2C_PF0_ANF_W<CFGR1_SPEC, 29> {
        I2C_PF0_ANF_W::new(self)
    }
    #[doc = "Bit 30 - Analog filter enable control driving capability activation bits PF1"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pf1_anf(&mut self) -> I2C_PF1_ANF_W<CFGR1_SPEC, 30> {
        I2C_PF1_ANF_W::new(self)
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
#[doc = "SYSCFG configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
