#[doc = "Reader of register TDATA"]
pub type R = crate::R<u32, super::TDATA>;
#[doc = "Writer for register TDATA"]
pub type W = crate::W<u32, super::TDATA>;
#[doc = "Register TDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::TDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TDATA`"]
pub type TDATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TDATA`"]
pub struct TDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Transmit data value"]
    #[inline(always)]
    pub fn tdata(&self) -> TDATA_R {
        TDATA_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transmit data value"]
    #[inline(always)]
    pub fn tdata(&mut self) -> TDATA_W {
        TDATA_W { w: self }
    }
}
