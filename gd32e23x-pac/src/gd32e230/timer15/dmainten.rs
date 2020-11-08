#[doc = "Reader of register DMAINTEN"]
pub type R = crate::R<u32, super::DMAINTEN>;
#[doc = "Writer for register DMAINTEN"]
pub type W = crate::W<u32, super::DMAINTEN>;
#[doc = "Register DMAINTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAINTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0DEN`"]
pub type CH0DEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0DEN`"]
pub struct CH0DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0DEN_W<'a> {
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
#[doc = "Reader of field `UPDEN`"]
pub type UPDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPDEN`"]
pub struct UPDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDEN_W<'a> {
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
#[doc = "Reader of field `BRKIE`"]
pub type BRKIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRKIE`"]
pub struct BRKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKIE_W<'a> {
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
#[doc = "Reader of field `CMTIE`"]
pub type CMTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMTIE`"]
pub struct CMTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMTIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CH0IE`"]
pub type CH0IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0IE`"]
pub struct CH0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0IE_W<'a> {
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
#[doc = "Reader of field `UPIE`"]
pub type UPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPIE`"]
pub struct UPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPIE_W<'a> {
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
    #[doc = "Bit 9 - Capture/Compare 0 DMA request enable"]
    #[inline(always)]
    pub fn ch0den(&self) -> CH0DEN_R {
        CH0DEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&self) -> UPDEN_R {
        UPDEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn brkie(&self) -> BRKIE_R {
        BRKIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn cmtie(&self) -> CMTIE_R {
        CMTIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 0 interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&self) -> CH0IE_R {
        CH0IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Capture/Compare 0 DMA request enable"]
    #[inline(always)]
    pub fn ch0den(&mut self) -> CH0DEN_W {
        CH0DEN_W { w: self }
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&mut self) -> UPDEN_W {
        UPDEN_W { w: self }
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn brkie(&mut self) -> BRKIE_W {
        BRKIE_W { w: self }
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn cmtie(&mut self) -> CMTIE_W {
        CMTIE_W { w: self }
    }
    #[doc = "Bit 1 - Capture/Compare 0 interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&mut self) -> CH0IE_W {
        CH0IE_W { w: self }
    }
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W {
        UPIE_W { w: self }
    }
}
