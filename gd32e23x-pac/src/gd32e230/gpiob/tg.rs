#[doc = "Writer for register TG"]
pub type W = crate::W<u32, super::TG>;
#[doc = "Register TG `reset()`'s with value 0"]
impl crate::ResetValue for super::TG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TG0`"]
pub struct TG0_W<'a> {
    w: &'a mut W,
}
impl<'a> TG0_W<'a> {
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
#[doc = "Write proxy for field `TG1`"]
pub struct TG1_W<'a> {
    w: &'a mut W,
}
impl<'a> TG1_W<'a> {
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
#[doc = "Write proxy for field `TG2`"]
pub struct TG2_W<'a> {
    w: &'a mut W,
}
impl<'a> TG2_W<'a> {
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
#[doc = "Write proxy for field `TG3`"]
pub struct TG3_W<'a> {
    w: &'a mut W,
}
impl<'a> TG3_W<'a> {
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
#[doc = "Write proxy for field `TG4`"]
pub struct TG4_W<'a> {
    w: &'a mut W,
}
impl<'a> TG4_W<'a> {
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
#[doc = "Write proxy for field `TG5`"]
pub struct TG5_W<'a> {
    w: &'a mut W,
}
impl<'a> TG5_W<'a> {
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
#[doc = "Write proxy for field `TG6`"]
pub struct TG6_W<'a> {
    w: &'a mut W,
}
impl<'a> TG6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `TG7`"]
pub struct TG7_W<'a> {
    w: &'a mut W,
}
impl<'a> TG7_W<'a> {
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
#[doc = "Write proxy for field `TG8`"]
pub struct TG8_W<'a> {
    w: &'a mut W,
}
impl<'a> TG8_W<'a> {
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
#[doc = "Write proxy for field `TG9`"]
pub struct TG9_W<'a> {
    w: &'a mut W,
}
impl<'a> TG9_W<'a> {
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
#[doc = "Write proxy for field `TG10`"]
pub struct TG10_W<'a> {
    w: &'a mut W,
}
impl<'a> TG10_W<'a> {
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
#[doc = "Write proxy for field `TG11`"]
pub struct TG11_W<'a> {
    w: &'a mut W,
}
impl<'a> TG11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `TG12`"]
pub struct TG12_W<'a> {
    w: &'a mut W,
}
impl<'a> TG12_W<'a> {
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
#[doc = "Write proxy for field `TG13`"]
pub struct TG13_W<'a> {
    w: &'a mut W,
}
impl<'a> TG13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `TG14`"]
pub struct TG14_W<'a> {
    w: &'a mut W,
}
impl<'a> TG14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `TG15`"]
pub struct TG15_W<'a> {
    w: &'a mut W,
}
impl<'a> TG15_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Port toggle bit"]
    #[inline(always)]
    pub fn tg0(&mut self) -> TG0_W {
        TG0_W { w: self }
    }
    #[doc = "Bit 1 - Port toggle bit"]
    #[inline(always)]
    pub fn tg1(&mut self) -> TG1_W {
        TG1_W { w: self }
    }
    #[doc = "Bit 2 - Port toggle bit"]
    #[inline(always)]
    pub fn tg2(&mut self) -> TG2_W {
        TG2_W { w: self }
    }
    #[doc = "Bit 3 - Port toggle bit"]
    #[inline(always)]
    pub fn tg3(&mut self) -> TG3_W {
        TG3_W { w: self }
    }
    #[doc = "Bit 4 - Port toggle bit"]
    #[inline(always)]
    pub fn tg4(&mut self) -> TG4_W {
        TG4_W { w: self }
    }
    #[doc = "Bit 5 - Port toggle bit"]
    #[inline(always)]
    pub fn tg5(&mut self) -> TG5_W {
        TG5_W { w: self }
    }
    #[doc = "Bit 6 - Port toggle bit"]
    #[inline(always)]
    pub fn tg6(&mut self) -> TG6_W {
        TG6_W { w: self }
    }
    #[doc = "Bit 7 - Port toggle bit"]
    #[inline(always)]
    pub fn tg7(&mut self) -> TG7_W {
        TG7_W { w: self }
    }
    #[doc = "Bit 8 - Port toggle bit"]
    #[inline(always)]
    pub fn tg8(&mut self) -> TG8_W {
        TG8_W { w: self }
    }
    #[doc = "Bit 9 - Port toggle bit"]
    #[inline(always)]
    pub fn tg9(&mut self) -> TG9_W {
        TG9_W { w: self }
    }
    #[doc = "Bit 10 - Port toggle bit"]
    #[inline(always)]
    pub fn tg10(&mut self) -> TG10_W {
        TG10_W { w: self }
    }
    #[doc = "Bit 11 - Port toggle bit"]
    #[inline(always)]
    pub fn tg11(&mut self) -> TG11_W {
        TG11_W { w: self }
    }
    #[doc = "Bit 12 - Port toggle bit"]
    #[inline(always)]
    pub fn tg12(&mut self) -> TG12_W {
        TG12_W { w: self }
    }
    #[doc = "Bit 13 - Port toggle bit"]
    #[inline(always)]
    pub fn tg13(&mut self) -> TG13_W {
        TG13_W { w: self }
    }
    #[doc = "Bit 14 - Port toggle bit"]
    #[inline(always)]
    pub fn tg14(&mut self) -> TG14_W {
        TG14_W { w: self }
    }
    #[doc = "Bit 15 - Port toggle bit"]
    #[inline(always)]
    pub fn tg15(&mut self) -> TG15_W {
        TG15_W { w: self }
    }
}
