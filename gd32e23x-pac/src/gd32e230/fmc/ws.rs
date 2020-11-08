#[doc = "Reader of register WS"]
pub type R = crate::R<u32, super::WS>;
#[doc = "Writer for register WS"]
pub type W = crate::W<u32, super::WS>;
#[doc = "Register WS `reset()`'s with value 0"]
impl crate::ResetValue for super::WS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WSCNT`"]
pub type WSCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WSCNT`"]
pub struct WSCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> WSCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `PFEN`"]
pub type PFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFEN`"]
pub struct PFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PFEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `PGW`"]
pub type PGW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGW`"]
pub struct PGW_W<'a> {
    w: &'a mut W,
}
impl<'a> PGW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - wait state counter register"]
    #[inline(always)]
    pub fn wscnt(&self) -> WSCNT_R {
        WSCNT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - Pre-fetch enable"]
    #[inline(always)]
    pub fn pfen(&self) -> PFEN_R {
        PFEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Program width to flash memory"]
    #[inline(always)]
    pub fn pgw(&self) -> PGW_R {
        PGW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - wait state counter register"]
    #[inline(always)]
    pub fn wscnt(&mut self) -> WSCNT_W {
        WSCNT_W { w: self }
    }
    #[doc = "Bit 4 - Pre-fetch enable"]
    #[inline(always)]
    pub fn pfen(&mut self) -> PFEN_W {
        PFEN_W { w: self }
    }
    #[doc = "Bit 15 - Program width to flash memory"]
    #[inline(always)]
    pub fn pgw(&mut self) -> PGW_W {
        PGW_W { w: self }
    }
}
