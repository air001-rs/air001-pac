#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `PE` reader - Peripheral enable"]
pub type PE_R = crate::BitReader;
#[doc = "Field `PE` writer - Peripheral enable"]
pub type PE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENPEC` reader - PEC enable"]
pub type ENPEC_R = crate::BitReader;
#[doc = "Field `ENPEC` writer - PEC enable"]
pub type ENPEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENGC` reader - General call enable"]
pub type ENGC_R = crate::BitReader;
#[doc = "Field `ENGC` writer - General call enable"]
pub type ENGC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NOSTRETCH` reader - Clock stretching disable (Slave mode)"]
pub type NOSTRETCH_R = crate::BitReader;
#[doc = "Field `NOSTRETCH` writer - Clock stretching disable (Slave mode)"]
pub type NOSTRETCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `START` reader - Start generation"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Start generation"]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOP` reader - Stop generation"]
pub type STOP_R = crate::BitReader;
#[doc = "Field `STOP` writer - Stop generation"]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACK` reader - Acknowledge enable"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ACK` writer - Acknowledge enable"]
pub type ACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POS` reader - Acknowledge/PEC Position (for data reception)"]
pub type POS_R = crate::BitReader;
#[doc = "Field `POS` writer - Acknowledge/PEC Position (for data reception)"]
pub type POS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEC` reader - Packet error checking"]
pub type PEC_R = crate::BitReader;
#[doc = "Field `PEC` writer - Packet error checking"]
pub type PEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWRST` reader - Software reset"]
pub type SWRST_R = crate::BitReader;
#[doc = "Field `SWRST` writer - Software reset"]
pub type SWRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    pub fn enpec(&self) -> ENPEC_R {
        ENPEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    pub fn engc(&self) -> ENGC_R {
        ENGC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock stretching disable (Slave mode)"]
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    pub fn pos(&self) -> POS_R {
        POS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<CR1_SPEC, 0> {
        PE_W::new(self)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    #[must_use]
    pub fn enpec(&mut self) -> ENPEC_W<CR1_SPEC, 5> {
        ENPEC_W::new(self)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    #[must_use]
    pub fn engc(&mut self) -> ENGC_W<CR1_SPEC, 6> {
        ENGC_W::new(self)
    }
    #[doc = "Bit 7 - Clock stretching disable (Slave mode)"]
    #[inline(always)]
    #[must_use]
    pub fn nostretch(&mut self) -> NOSTRETCH_W<CR1_SPEC, 7> {
        NOSTRETCH_W::new(self)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CR1_SPEC, 8> {
        START_W::new(self)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CR1_SPEC, 9> {
        STOP_W::new(self)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<CR1_SPEC, 10> {
        ACK_W::new(self)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    #[must_use]
    pub fn pos(&mut self) -> POS_W<CR1_SPEC, 11> {
        POS_W::new(self)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    #[must_use]
    pub fn pec(&mut self) -> PEC_W<CR1_SPEC, 12> {
        PEC_W::new(self)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<CR1_SPEC, 15> {
        SWRST_W::new(self)
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
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
