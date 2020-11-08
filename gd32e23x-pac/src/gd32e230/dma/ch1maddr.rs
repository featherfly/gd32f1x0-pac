#[doc = "Reader of register CH1MADDR"]
pub type R = crate::R<u32, super::CH1MADDR>;
#[doc = "Writer for register CH1MADDR"]
pub type W = crate::W<u32, super::CH1MADDR>;
#[doc = "Register CH1MADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1MADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MADDR`"]
pub type MADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MADDR`"]
pub struct MADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> MADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn maddr(&self) -> MADDR_R {
        MADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn maddr(&mut self) -> MADDR_W {
        MADDR_W { w: self }
    }
}
