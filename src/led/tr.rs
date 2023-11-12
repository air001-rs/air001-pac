#[doc = "Register `TR` reader"]
pub type R = crate::R<TR_SPEC>;
#[doc = "Register `TR` writer"]
pub type W = crate::W<TR_SPEC>;
#[doc = "Field `T1` reader - Light on time"]
pub type T1_R = crate::FieldReader;
#[doc = "Field `T1` writer - Light on time"]
pub type T1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `T2` reader - Switch time"]
pub type T2_R = crate::FieldReader;
#[doc = "Field `T2` writer - Switch time"]
pub type T2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Light on time"]
    #[inline(always)]
    pub fn t1(&self) -> T1_R {
        T1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Switch time"]
    #[inline(always)]
    pub fn t2(&self) -> T2_R {
        T2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Light on time"]
    #[inline(always)]
    #[must_use]
    pub fn t1(&mut self) -> T1_W<TR_SPEC, 0> {
        T1_W::new(self)
    }
    #[doc = "Bits 8:15 - Switch time"]
    #[inline(always)]
    #[must_use]
    pub fn t2(&mut self) -> T2_W<TR_SPEC, 8> {
        T2_W::new(self)
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
#[doc = "Time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TR_SPEC;
impl crate::RegisterSpec for TR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr::R`](R) reader structure"]
impl crate::Readable for TR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tr::W`](W) writer structure"]
impl crate::Writable for TR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR to value 0"]
impl crate::Resettable for TR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
