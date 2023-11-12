#[doc = "Register `IOPENR` reader"]
pub type R = crate::R<IOPENR_SPEC>;
#[doc = "Register `IOPENR` writer"]
pub type W = crate::W<IOPENR_SPEC>;
#[doc = "Field `GPIOAEN` reader - I/O port A clock enable"]
pub type GPIOAEN_R = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - I/O port A clock enable"]
pub type GPIOAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOBEN` reader - I/O port B clock enable"]
pub type GPIOBEN_R = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - I/O port B clock enable"]
pub type GPIOBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOFEN` reader - I/O port F clock enable"]
pub type GPIOFEN_R = crate::BitReader;
#[doc = "Field `GPIOFEN` writer - I/O port F clock enable"]
pub type GPIOFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - I/O port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<IOPENR_SPEC, 0> {
        GPIOAEN_W::new(self)
    }
    #[doc = "Bit 1 - I/O port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<IOPENR_SPEC, 1> {
        GPIOBEN_W::new(self)
    }
    #[doc = "Bit 5 - I/O port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<IOPENR_SPEC, 5> {
        GPIOFEN_W::new(self)
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
#[doc = "GPIO clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iopenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iopenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOPENR_SPEC;
impl crate::RegisterSpec for IOPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopenr::R`](R) reader structure"]
impl crate::Readable for IOPENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iopenr::W`](W) writer structure"]
impl crate::Writable for IOPENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOPENR to value 0"]
impl crate::Resettable for IOPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
