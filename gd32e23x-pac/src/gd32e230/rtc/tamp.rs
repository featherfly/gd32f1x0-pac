#[doc = "Reader of register TAMP"]
pub type R = crate::R<u32, super::TAMP>;
#[doc = "Writer for register TAMP"]
pub type W = crate::W<u32, super::TAMP>;
#[doc = "Register TAMP `reset()`'s with value 0"]
impl crate::ResetValue for super::TAMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PC15MDE`"]
pub type PC15MDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC15MDE`"]
pub struct PC15MDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PC15MDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `PC15VAL`"]
pub type PC15VAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC15VAL`"]
pub struct PC15VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PC15VAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `PC14MDE`"]
pub type PC14MDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC14MDE`"]
pub struct PC14MDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PC14MDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `PC14VAL`"]
pub type PC14VAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC14VAL`"]
pub struct PC14VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PC14VAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `PC13MDE`"]
pub type PC13MDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC13MDE`"]
pub struct PC13MDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PC13MDE_W<'a> {
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
#[doc = "Reader of field `PC13VAL`"]
pub type PC13VAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC13VAL`"]
pub struct PC13VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PC13VAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `DISPU`"]
pub type DISPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISPU`"]
pub struct DISPU_W<'a> {
    w: &'a mut W,
}
impl<'a> DISPU_W<'a> {
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
#[doc = "Reader of field `PRCH`"]
pub type PRCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRCH`"]
pub struct PRCH_W<'a> {
    w: &'a mut W,
}
impl<'a> PRCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `FLT`"]
pub type FLT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLT`"]
pub struct FLT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `FREQ`"]
pub type FREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FREQ`"]
pub struct FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `TPTS`"]
pub type TPTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPTS`"]
pub struct TPTS_W<'a> {
    w: &'a mut W,
}
impl<'a> TPTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TP1EG`"]
pub type TP1EG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TP1EG`"]
pub struct TP1EG_W<'a> {
    w: &'a mut W,
}
impl<'a> TP1EG_W<'a> {
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
#[doc = "Reader of field `TP1EN`"]
pub type TP1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TP1EN`"]
pub struct TP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TP1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TPIE`"]
pub type TPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPIE`"]
pub struct TPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TPIE_W<'a> {
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
#[doc = "Reader of field `TP0EG`"]
pub type TP0EG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TP0EG`"]
pub struct TP0EG_W<'a> {
    w: &'a mut W,
}
impl<'a> TP0EG_W<'a> {
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
#[doc = "Reader of field `TP0EN`"]
pub type TP0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TP0EN`"]
pub struct TP0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TP0EN_W<'a> {
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
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    pub fn pc15mde(&self) -> PC15MDE_R {
        PC15MDE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    pub fn pc15val(&self) -> PC15VAL_R {
        PC15VAL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - PC14 mode"]
    #[inline(always)]
    pub fn pc14mde(&self) -> PC14MDE_R {
        PC14MDE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    pub fn pc14val(&self) -> PC14VAL_R {
        PC14VAL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PC13 mode"]
    #[inline(always)]
    pub fn pc13mde(&self) -> PC13MDE_R {
        PC13MDE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RTC_ALARM output type/PC13 value"]
    #[inline(always)]
    pub fn pc13val(&self) -> PC13VAL_R {
        PC13VAL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    pub fn dispu(&self) -> DISPU_R {
        DISPU_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration"]
    #[inline(always)]
    pub fn prch(&self) -> PRCH_R {
        PRCH_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count"]
    #[inline(always)]
    pub fn flt(&self) -> FLT_R {
        FLT_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tpts(&self) -> TPTS_R {
        TPTS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Tamper 1 event trigger edge"]
    #[inline(always)]
    pub fn tp1eg(&self) -> TP1EG_R {
        TP1EG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tp1en(&self) -> TP1EN_R {
        TP1EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Tamper detection interrupt enable"]
    #[inline(always)]
    pub fn tpie(&self) -> TPIE_R {
        TPIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Active level for RTC_TAMP1 input"]
    #[inline(always)]
    pub fn tp0eg(&self) -> TP0EG_R {
        TP0EG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Tamper 0 event trigger edge"]
    #[inline(always)]
    pub fn tp0en(&self) -> TP0EN_R {
        TP0EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    pub fn pc15mde(&mut self) -> PC15MDE_W {
        PC15MDE_W { w: self }
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    pub fn pc15val(&mut self) -> PC15VAL_W {
        PC15VAL_W { w: self }
    }
    #[doc = "Bit 21 - PC14 mode"]
    #[inline(always)]
    pub fn pc14mde(&mut self) -> PC14MDE_W {
        PC14MDE_W { w: self }
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    pub fn pc14val(&mut self) -> PC14VAL_W {
        PC14VAL_W { w: self }
    }
    #[doc = "Bit 19 - PC13 mode"]
    #[inline(always)]
    pub fn pc13mde(&mut self) -> PC13MDE_W {
        PC13MDE_W { w: self }
    }
    #[doc = "Bit 18 - RTC_ALARM output type/PC13 value"]
    #[inline(always)]
    pub fn pc13val(&mut self) -> PC13VAL_W {
        PC13VAL_W { w: self }
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    pub fn dispu(&mut self) -> DISPU_W {
        DISPU_W { w: self }
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration"]
    #[inline(always)]
    pub fn prch(&mut self) -> PRCH_W {
        PRCH_W { w: self }
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count"]
    #[inline(always)]
    pub fn flt(&mut self) -> FLT_W {
        FLT_W { w: self }
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W { w: self }
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tpts(&mut self) -> TPTS_W {
        TPTS_W { w: self }
    }
    #[doc = "Bit 4 - Tamper 1 event trigger edge"]
    #[inline(always)]
    pub fn tp1eg(&mut self) -> TP1EG_W {
        TP1EG_W { w: self }
    }
    #[doc = "Bit 3 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tp1en(&mut self) -> TP1EN_W {
        TP1EN_W { w: self }
    }
    #[doc = "Bit 2 - Tamper detection interrupt enable"]
    #[inline(always)]
    pub fn tpie(&mut self) -> TPIE_W {
        TPIE_W { w: self }
    }
    #[doc = "Bit 1 - Active level for RTC_TAMP1 input"]
    #[inline(always)]
    pub fn tp0eg(&mut self) -> TP0EG_W {
        TP0EG_W { w: self }
    }
    #[doc = "Bit 0 - Tamper 0 event trigger edge"]
    #[inline(always)]
    pub fn tp0en(&mut self) -> TP0EN_W {
        TP0EN_W { w: self }
    }
}
