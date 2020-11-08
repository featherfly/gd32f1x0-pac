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
