#[doc = "Register `FR` reader"]
pub type R = crate::R<FR_SPEC>;
#[doc = "Register `FR` writer"]
pub type W = crate::W<FR_SPEC>;
#[doc = "Field `FLTEN` reader - Filter enable bit"]
pub type FLTEN_R = crate::BitReader;
#[doc = "Field `FLTEN` writer - Filter enable bit"]
pub type FLTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLTCNT` reader - Comparator filter and counter"]
pub type FLTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `FLTCNT` writer - Comparator filter and counter"]
pub type FLTCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - Filter enable bit"]
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - Comparator filter and counter"]
    #[inline(always)]
    pub fn fltcnt(&self) -> FLTCNT_R {
        FLTCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Filter enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn flten(&mut self) -> FLTEN_W<FR_SPEC, 0> {
        FLTEN_W::new(self)
    }
    #[doc = "Bits 16:31 - Comparator filter and counter"]
    #[inline(always)]
    #[must_use]
    pub fn fltcnt(&mut self) -> FLTCNT_W<FR_SPEC, 16> {
        FLTCNT_W::new(self)
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
#[doc = "Comparator Filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FR_SPEC;
impl crate::RegisterSpec for FR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fr::R`](R) reader structure"]
impl crate::Readable for FR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fr::W`](W) writer structure"]
impl crate::Writable for FR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
