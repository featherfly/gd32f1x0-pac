#[doc = "Reader of register CTL0"]
pub type R = crate::R<u32, super::CTL0>;
#[doc = "Writer for register CTL0"]
pub type W = crate::W<u32, super::CTL0>;
#[doc = "Register CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLP_HOLD`"]
pub type SLP_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLP_HOLD`"]
pub struct SLP_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_HOLD_W<'a> {
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
#[doc = "Reader of field `DSLP_HOLD`"]
pub type DSLP_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSLP_HOLD`"]
pub struct DSLP_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DSLP_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `STB_HOLD`"]
pub type STB_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STB_HOLD`"]
pub struct STB_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> STB_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `FWDGT_HOLD`"]
pub type FWDGT_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FWDGT_HOLD`"]
pub struct FWDGT_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> FWDGT_HOLD_W<'a> {
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
#[doc = "Reader of field `WWDGT_HOLD`"]
pub type WWDGT_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WWDGT_HOLD`"]
pub struct WWDGT_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGT_HOLD_W<'a> {
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
#[doc = "Reader of field `TIMER0_HOLD`"]
pub type TIMER0_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER0_HOLD`"]
pub struct TIMER0_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TIMER2_HOLD`"]
pub type TIMER2_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER2_HOLD`"]
pub struct TIMER2_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `I2C0_HOLD`"]
pub type I2C0_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0_HOLD`"]
pub struct I2C0_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_HOLD_W<'a> {
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
#[doc = "Reader of field `I2C1_HOLD`"]
pub type I2C1_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1_HOLD`"]
pub struct I2C1_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TIMER5_HOLD`"]
pub type TIMER5_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER5_HOLD`"]
pub struct TIMER5_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER5_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `TIMER13_HOLD`"]
pub type TIMER13_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER13_HOLD`"]
pub struct TIMER13_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER13_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Sleep mode hold register"]
    #[inline(always)]
    pub fn slp_hold(&self) -> SLP_HOLD_R {
        SLP_HOLD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DEEPSLEEP mode hold Mode"]
    #[inline(always)]
    pub fn dslp_hold(&self) -> DSLP_HOLD_R {
        DSLP_HOLD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Standby mode hold Mode"]
    #[inline(always)]
    pub fn stb_hold(&self) -> STB_HOLD_R {
        STB_HOLD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FWDGT hold register"]
    #[inline(always)]
    pub fn fwdgt_hold(&self) -> FWDGT_HOLD_R {
        FWDGT_HOLD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - WWDGT hold register"]
    #[inline(always)]
    pub fn wwdgt_hold(&self) -> WWDGT_HOLD_R {
        WWDGT_HOLD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Timer 0 hold register"]
    #[inline(always)]
    pub fn timer0_hold(&self) -> TIMER0_HOLD_R {
        TIMER0_HOLD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Timer 2 hold register"]
    #[inline(always)]
    pub fn timer2_hold(&self) -> TIMER2_HOLD_R {
        TIMER2_HOLD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - I2C0 hold register"]
    #[inline(always)]
    pub fn i2c0_hold(&self) -> I2C0_HOLD_R {
        I2C0_HOLD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - I2C1 hold register"]
    #[inline(always)]
    pub fn i2c1_hold(&self) -> I2C1_HOLD_R {
        I2C1_HOLD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Timer 5 hold register"]
    #[inline(always)]
    pub fn timer5_hold(&self) -> TIMER5_HOLD_R {
        TIMER5_HOLD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Timer 13 hold register"]
    #[inline(always)]
    pub fn timer13_hold(&self) -> TIMER13_HOLD_R {
        TIMER13_HOLD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sleep mode hold register"]
    #[inline(always)]
    pub fn slp_hold(&mut self) -> SLP_HOLD_W {
        SLP_HOLD_W { w: self }
    }
    #[doc = "Bit 1 - DEEPSLEEP mode hold Mode"]
    #[inline(always)]
    pub fn dslp_hold(&mut self) -> DSLP_HOLD_W {
        DSLP_HOLD_W { w: self }
    }
    #[doc = "Bit 2 - Standby mode hold Mode"]
    #[inline(always)]
    pub fn stb_hold(&mut self) -> STB_HOLD_W {
        STB_HOLD_W { w: self }
    }
    #[doc = "Bit 8 - FWDGT hold register"]
    #[inline(always)]
    pub fn fwdgt_hold(&mut self) -> FWDGT_HOLD_W {
        FWDGT_HOLD_W { w: self }
    }
    #[doc = "Bit 9 - WWDGT hold register"]
    #[inline(always)]
    pub fn wwdgt_hold(&mut self) -> WWDGT_HOLD_W {
        WWDGT_HOLD_W { w: self }
    }
    #[doc = "Bit 10 - Timer 0 hold register"]
    #[inline(always)]
    pub fn timer0_hold(&mut self) -> TIMER0_HOLD_W {
        TIMER0_HOLD_W { w: self }
    }
    #[doc = "Bit 12 - Timer 2 hold register"]
    #[inline(always)]
    pub fn timer2_hold(&mut self) -> TIMER2_HOLD_W {
        TIMER2_HOLD_W { w: self }
    }
    #[doc = "Bit 15 - I2C0 hold register"]
    #[inline(always)]
    pub fn i2c0_hold(&mut self) -> I2C0_HOLD_W {
        I2C0_HOLD_W { w: self }
    }
    #[doc = "Bit 16 - I2C1 hold register"]
    #[inline(always)]
    pub fn i2c1_hold(&mut self) -> I2C1_HOLD_W {
        I2C1_HOLD_W { w: self }
    }
    #[doc = "Bit 19 - Timer 5 hold register"]
    #[inline(always)]
    pub fn timer5_hold(&mut self) -> TIMER5_HOLD_W {
        TIMER5_HOLD_W { w: self }
    }
    #[doc = "Bit 27 - Timer 13 hold register"]
    #[inline(always)]
    pub fn timer13_hold(&mut self) -> TIMER13_HOLD_W {
        TIMER13_HOLD_W { w: self }
    }
}
