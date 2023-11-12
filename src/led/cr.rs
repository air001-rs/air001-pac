#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `LEDON` reader - LED enable"]
pub type LEDON_R = crate::BitReader;
#[doc = "Field `LEDON` writer - LED enable"]
pub type LEDON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LED_COM_SEL` reader - LED COM Selection"]
pub type LED_COM_SEL_R = crate::FieldReader;
#[doc = "Field `LED_COM_SEL` writer - LED COM Selection"]
pub type LED_COM_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IE` reader - LED interrupt enable"]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - LED interrupt enable"]
pub type IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EHS` reader - Light control"]
pub type EHS_R = crate::FieldReader;
#[doc = "Field `EHS` writer - Light control"]
pub type EHS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - LED enable"]
    #[inline(always)]
    pub fn ledon(&self) -> LEDON_R {
        LEDON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - LED COM Selection"]
    #[inline(always)]
    pub fn led_com_sel(&self) -> LED_COM_SEL_R {
        LED_COM_SEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - LED interrupt enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Light control"]
    #[inline(always)]
    pub fn ehs(&self) -> EHS_R {
        EHS_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LED enable"]
    #[inline(always)]
    #[must_use]
    pub fn ledon(&mut self) -> LEDON_W<CR_SPEC, 0> {
        LEDON_W::new(self)
    }
    #[doc = "Bits 1:2 - LED COM Selection"]
    #[inline(always)]
    #[must_use]
    pub fn led_com_sel(&mut self) -> LED_COM_SEL_W<CR_SPEC, 1> {
        LED_COM_SEL_W::new(self)
    }
    #[doc = "Bit 3 - LED interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<CR_SPEC, 3> {
        IE_W::new(self)
    }
    #[doc = "Bits 12:13 - Light control"]
    #[inline(always)]
    #[must_use]
    pub fn ehs(&mut self) -> EHS_W<CR_SPEC, 12> {
        EHS_W::new(self)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
