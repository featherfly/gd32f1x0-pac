#[doc = "Reader of register BAUD"]
pub type R = crate::R<u32, super::BAUD>;
#[doc = "Writer for register BAUD"]
pub type W = crate::W<u32, super::BAUD>;
#[doc = "Register BAUD `reset()`'s with value 0"]
impl crate::ResetValue for super::BAUD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BRR_INT`"]
pub type BRR_INT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BRR_INT`"]
pub struct BRR_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> BRR_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Reader of field `BRR_FRA`"]
pub type BRR_FRA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BRR_FRA`"]
pub struct BRR_FRA_W<'a> {
    w: &'a mut W,
}
impl<'a> BRR_FRA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:15 - integer of baud-rate divider"]
    #[inline(always)]
    pub fn brr_int(&self) -> BRR_INT_R {
        BRR_INT_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - integer of baud-rate divider"]
    #[inline(always)]
    pub fn brr_fra(&self) -> BRR_FRA_R {
        BRR_FRA_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:15 - integer of baud-rate divider"]
    #[inline(always)]
    pub fn brr_int(&mut self) -> BRR_INT_W {
        BRR_INT_W { w: self }
    }
    #[doc = "Bits 0:3 - integer of baud-rate divider"]
    #[inline(always)]
    pub fn brr_fra(&mut self) -> BRR_FRA_W {
        BRR_FRA_W { w: self }
    }
}
