#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0x0f94_0000"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f94_0000
    }
}
#[doc = "Reader of field `INTEN0`"]
pub type INTEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN0`"]
pub struct INTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN0_W<'a> {
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
#[doc = "Reader of field `INTEN1`"]
pub type INTEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN1`"]
pub struct INTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN1_W<'a> {
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
#[doc = "Reader of field `INTEN2`"]
pub type INTEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN2`"]
pub struct INTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN2_W<'a> {
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
#[doc = "Reader of field `INTEN3`"]
pub type INTEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN3`"]
pub struct INTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN3_W<'a> {
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
#[doc = "Reader of field `INTEN4`"]
pub type INTEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN4`"]
pub struct INTEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN4_W<'a> {
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
#[doc = "Reader of field `INTEN5`"]
pub type INTEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN5`"]
pub struct INTEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN5_W<'a> {
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
#[doc = "Reader of field `INTEN6`"]
pub type INTEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN6`"]
pub struct INTEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN6_W<'a> {
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
#[doc = "Reader of field `INTEN7`"]
pub type INTEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN7`"]
pub struct INTEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN7_W<'a> {
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
#[doc = "Reader of field `INTEN8`"]
pub type INTEN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN8`"]
pub struct INTEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN8_W<'a> {
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
#[doc = "Reader of field `INTEN9`"]
pub type INTEN9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN9`"]
pub struct INTEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN9_W<'a> {
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
#[doc = "Reader of field `INTEN10`"]
pub type INTEN10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN10`"]
pub struct INTEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN10_W<'a> {
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
#[doc = "Reader of field `INTEN11`"]
pub type INTEN11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN11`"]
pub struct INTEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN11_W<'a> {
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
#[doc = "Reader of field `INTEN12`"]
pub type INTEN12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN12`"]
pub struct INTEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN12_W<'a> {
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
#[doc = "Reader of field `INTEN13`"]
pub type INTEN13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN13`"]
pub struct INTEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN13_W<'a> {
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
#[doc = "Reader of field `INTEN14`"]
pub type INTEN14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN14`"]
pub struct INTEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN14_W<'a> {
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
#[doc = "Reader of field `INTEN15`"]
pub type INTEN15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN15`"]
pub struct INTEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN15_W<'a> {
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
#[doc = "Reader of field `INTEN16`"]
pub type INTEN16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN16`"]
pub struct INTEN16_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN16_W<'a> {
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
#[doc = "Reader of field `INTEN17`"]
pub type INTEN17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN17`"]
pub struct INTEN17_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `INTEN18`"]
pub type INTEN18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN18`"]
pub struct INTEN18_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN18_W<'a> {
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
#[doc = "Reader of field `INTEN19`"]
pub type INTEN19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN19`"]
pub struct INTEN19_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN19_W<'a> {
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
#[doc = "Reader of field `INTEN20`"]
pub type INTEN20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN20`"]
pub struct INTEN20_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN20_W<'a> {
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
#[doc = "Reader of field `INTEN21`"]
pub type INTEN21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN21`"]
pub struct INTEN21_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN21_W<'a> {
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
#[doc = "Reader of field `INTEN22`"]
pub type INTEN22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN22`"]
pub struct INTEN22_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN22_W<'a> {
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
#[doc = "Reader of field `INTEN23`"]
pub type INTEN23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN23`"]
pub struct INTEN23_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN23_W<'a> {
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
#[doc = "Reader of field `INTEN24`"]
pub type INTEN24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN24`"]
pub struct INTEN24_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `INTEN25`"]
pub type INTEN25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN25`"]
pub struct INTEN25_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `INTEN26`"]
pub type INTEN26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN26`"]
pub struct INTEN26_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `INTEN27`"]
pub type INTEN27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN27`"]
pub struct INTEN27_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN27_W<'a> {
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
    #[doc = "Bit 0 - Enable Interrupt on line 0"]
    #[inline(always)]
    pub fn inten0(&self) -> INTEN0_R {
        INTEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Interrupt on line 1"]
    #[inline(always)]
    pub fn inten1(&self) -> INTEN1_R {
        INTEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Interrupt on line 2"]
    #[inline(always)]
    pub fn inten2(&self) -> INTEN2_R {
        INTEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable Interrupt on line 3"]
    #[inline(always)]
    pub fn inten3(&self) -> INTEN3_R {
        INTEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Interrupt on line 4"]
    #[inline(always)]
    pub fn inten4(&self) -> INTEN4_R {
        INTEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable Interrupt on line 5"]
    #[inline(always)]
    pub fn inten5(&self) -> INTEN5_R {
        INTEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable Interrupt on line 6"]
    #[inline(always)]
    pub fn inten6(&self) -> INTEN6_R {
        INTEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable Interrupt on line 7"]
    #[inline(always)]
    pub fn inten7(&self) -> INTEN7_R {
        INTEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Interrupt on line 8"]
    #[inline(always)]
    pub fn inten8(&self) -> INTEN8_R {
        INTEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable Interrupt on line 9"]
    #[inline(always)]
    pub fn inten9(&self) -> INTEN9_R {
        INTEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable Interrupt on line 10"]
    #[inline(always)]
    pub fn inten10(&self) -> INTEN10_R {
        INTEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Interrupt on line 11"]
    #[inline(always)]
    pub fn inten11(&self) -> INTEN11_R {
        INTEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable Interrupt on line 12"]
    #[inline(always)]
    pub fn inten12(&self) -> INTEN12_R {
        INTEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Interrupt on line 13"]
    #[inline(always)]
    pub fn inten13(&self) -> INTEN13_R {
        INTEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable Interrupt on line 14"]
    #[inline(always)]
    pub fn inten14(&self) -> INTEN14_R {
        INTEN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable Interrupt on line 15"]
    #[inline(always)]
    pub fn inten15(&self) -> INTEN15_R {
        INTEN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable Interrupt on line 16"]
    #[inline(always)]
    pub fn inten16(&self) -> INTEN16_R {
        INTEN16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable Interrupt on line 17"]
    #[inline(always)]
    pub fn inten17(&self) -> INTEN17_R {
        INTEN17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable Interrupt on line 18"]
    #[inline(always)]
    pub fn inten18(&self) -> INTEN18_R {
        INTEN18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable Interrupt on line 19"]
    #[inline(always)]
    pub fn inten19(&self) -> INTEN19_R {
        INTEN19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable Interrupt on line 20"]
    #[inline(always)]
    pub fn inten20(&self) -> INTEN20_R {
        INTEN20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable Interrupt on line 21"]
    #[inline(always)]
    pub fn inten21(&self) -> INTEN21_R {
        INTEN21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable Interrupt on line 22"]
    #[inline(always)]
    pub fn inten22(&self) -> INTEN22_R {
        INTEN22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable Interrupt on line 23"]
    #[inline(always)]
    pub fn inten23(&self) -> INTEN23_R {
        INTEN23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable Interrupt on line 24"]
    #[inline(always)]
    pub fn inten24(&self) -> INTEN24_R {
        INTEN24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable Interrupt on line 25"]
    #[inline(always)]
    pub fn inten25(&self) -> INTEN25_R {
        INTEN25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable Interrupt on line 26"]
    #[inline(always)]
    pub fn inten26(&self) -> INTEN26_R {
        INTEN26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable Interrupt on line 27"]
    #[inline(always)]
    pub fn inten27(&self) -> INTEN27_R {
        INTEN27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Interrupt on line 0"]
    #[inline(always)]
    pub fn inten0(&mut self) -> INTEN0_W {
        INTEN0_W { w: self }
    }
    #[doc = "Bit 1 - Enable Interrupt on line 1"]
    #[inline(always)]
    pub fn inten1(&mut self) -> INTEN1_W {
        INTEN1_W { w: self }
    }
    #[doc = "Bit 2 - Enable Interrupt on line 2"]
    #[inline(always)]
    pub fn inten2(&mut self) -> INTEN2_W {
        INTEN2_W { w: self }
    }
    #[doc = "Bit 3 - Enable Interrupt on line 3"]
    #[inline(always)]
    pub fn inten3(&mut self) -> INTEN3_W {
        INTEN3_W { w: self }
    }
    #[doc = "Bit 4 - Enable Interrupt on line 4"]
    #[inline(always)]
    pub fn inten4(&mut self) -> INTEN4_W {
        INTEN4_W { w: self }
    }
    #[doc = "Bit 5 - Enable Interrupt on line 5"]
    #[inline(always)]
    pub fn inten5(&mut self) -> INTEN5_W {
        INTEN5_W { w: self }
    }
    #[doc = "Bit 6 - Enable Interrupt on line 6"]
    #[inline(always)]
    pub fn inten6(&mut self) -> INTEN6_W {
        INTEN6_W { w: self }
    }
    #[doc = "Bit 7 - Enable Interrupt on line 7"]
    #[inline(always)]
    pub fn inten7(&mut self) -> INTEN7_W {
        INTEN7_W { w: self }
    }
    #[doc = "Bit 8 - Enable Interrupt on line 8"]
    #[inline(always)]
    pub fn inten8(&mut self) -> INTEN8_W {
        INTEN8_W { w: self }
    }
    #[doc = "Bit 9 - Enable Interrupt on line 9"]
    #[inline(always)]
    pub fn inten9(&mut self) -> INTEN9_W {
        INTEN9_W { w: self }
    }
    #[doc = "Bit 10 - Enable Interrupt on line 10"]
    #[inline(always)]
    pub fn inten10(&mut self) -> INTEN10_W {
        INTEN10_W { w: self }
    }
    #[doc = "Bit 11 - Enable Interrupt on line 11"]
    #[inline(always)]
    pub fn inten11(&mut self) -> INTEN11_W {
        INTEN11_W { w: self }
    }
    #[doc = "Bit 12 - Enable Interrupt on line 12"]
    #[inline(always)]
    pub fn inten12(&mut self) -> INTEN12_W {
        INTEN12_W { w: self }
    }
    #[doc = "Bit 13 - Enable Interrupt on line 13"]
    #[inline(always)]
    pub fn inten13(&mut self) -> INTEN13_W {
        INTEN13_W { w: self }
    }
    #[doc = "Bit 14 - Enable Interrupt on line 14"]
    #[inline(always)]
    pub fn inten14(&mut self) -> INTEN14_W {
        INTEN14_W { w: self }
    }
    #[doc = "Bit 15 - Enable Interrupt on line 15"]
    #[inline(always)]
    pub fn inten15(&mut self) -> INTEN15_W {
        INTEN15_W { w: self }
    }
    #[doc = "Bit 16 - Enable Interrupt on line 16"]
    #[inline(always)]
    pub fn inten16(&mut self) -> INTEN16_W {
        INTEN16_W { w: self }
    }
    #[doc = "Bit 17 - Enable Interrupt on line 17"]
    #[inline(always)]
    pub fn inten17(&mut self) -> INTEN17_W {
        INTEN17_W { w: self }
    }
    #[doc = "Bit 18 - Enable Interrupt on line 18"]
    #[inline(always)]
    pub fn inten18(&mut self) -> INTEN18_W {
        INTEN18_W { w: self }
    }
    #[doc = "Bit 19 - Enable Interrupt on line 19"]
    #[inline(always)]
    pub fn inten19(&mut self) -> INTEN19_W {
        INTEN19_W { w: self }
    }
    #[doc = "Bit 20 - Enable Interrupt on line 20"]
    #[inline(always)]
    pub fn inten20(&mut self) -> INTEN20_W {
        INTEN20_W { w: self }
    }
    #[doc = "Bit 21 - Enable Interrupt on line 21"]
    #[inline(always)]
    pub fn inten21(&mut self) -> INTEN21_W {
        INTEN21_W { w: self }
    }
    #[doc = "Bit 22 - Enable Interrupt on line 22"]
    #[inline(always)]
    pub fn inten22(&mut self) -> INTEN22_W {
        INTEN22_W { w: self }
    }
    #[doc = "Bit 23 - Enable Interrupt on line 23"]
    #[inline(always)]
    pub fn inten23(&mut self) -> INTEN23_W {
        INTEN23_W { w: self }
    }
    #[doc = "Bit 24 - Enable Interrupt on line 24"]
    #[inline(always)]
    pub fn inten24(&mut self) -> INTEN24_W {
        INTEN24_W { w: self }
    }
    #[doc = "Bit 25 - Enable Interrupt on line 25"]
    #[inline(always)]
    pub fn inten25(&mut self) -> INTEN25_W {
        INTEN25_W { w: self }
    }
    #[doc = "Bit 26 - Enable Interrupt on line 26"]
    #[inline(always)]
    pub fn inten26(&mut self) -> INTEN26_W {
        INTEN26_W { w: self }
    }
    #[doc = "Bit 27 - Enable Interrupt on line 27"]
    #[inline(always)]
    pub fn inten27(&mut self) -> INTEN27_W {
        INTEN27_W { w: self }
    }
}
