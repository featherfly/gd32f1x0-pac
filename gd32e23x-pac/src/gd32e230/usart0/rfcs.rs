#[doc = "Reader of register RFCS"]
pub type R = crate::R<u32, super::RFCS>;
#[doc = "Writer for register RFCS"]
pub type W = crate::W<u32, super::RFCS>;
#[doc = "Register RFCS `reset()`'s with value 0x0400"]
impl crate::ResetValue for super::RFCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400
    }
}
#[doc = "Reader of field `RFFINT`"]
pub type RFFINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFFINT`"]
pub struct RFFINT_W<'a> {
    w: &'a mut W,
}
impl<'a> RFFINT_W<'a> {
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
#[doc = "Reader of field `RFCNT`"]
pub type RFCNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `RFF`"]
pub type RFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFE`"]
pub type RFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFFIE`"]
pub type RFFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFFIE`"]
pub struct RFFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFFIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RFEN`"]
pub type RFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFEN`"]
pub struct RFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `ELNACK`"]
pub type ELNACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ELNACK`"]
pub struct ELNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ELNACK_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Receive FIFO full interrupt flag"]
    #[inline(always)]
    pub fn rffint(&self) -> RFFINT_R {
        RFFINT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Receive FIFO count number"]
    #[inline(always)]
    pub fn rfcnt(&self) -> RFCNT_R {
        RFCNT_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Receive FIFO full flag"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive FIFO empty flag"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO full interrupt enable"]
    #[inline(always)]
    pub fn rffie(&self) -> RFFIE_R {
        RFFIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Early NKEN when smartcard mode is selected"]
    #[inline(always)]
    pub fn elnack(&self) -> ELNACK_R {
        ELNACK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Receive FIFO full interrupt flag"]
    #[inline(always)]
    pub fn rffint(&mut self) -> RFFINT_W {
        RFFINT_W { w: self }
    }
    #[doc = "Bit 9 - Receive FIFO full interrupt enable"]
    #[inline(always)]
    pub fn rffie(&mut self) -> RFFIE_W {
        RFFIE_W { w: self }
    }
    #[doc = "Bit 8 - Receive FIFO enable"]
    #[inline(always)]
    pub fn rfen(&mut self) -> RFEN_W {
        RFEN_W { w: self }
    }
    #[doc = "Bit 0 - Early NKEN when smartcard mode is selected"]
    #[inline(always)]
    pub fn elnack(&mut self) -> ELNACK_W {
        ELNACK_W { w: self }
    }
}
