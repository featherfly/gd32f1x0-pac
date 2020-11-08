#[doc = "Reader of register PSC"]
pub type R = crate::R<u32, super::PSC>;
#[doc = "Writer for register PSC"]
pub type W = crate::W<u32, super::PSC>;
#[doc = "Register PSC `reset()`'s with value 0x007f_00ff"]
impl crate::ResetValue for super::PSC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x007f_00ff
    }
}
#[doc = "Reader of field `FACTOR_A`"]
pub type FACTOR_A_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FACTOR_A`"]
pub struct FACTOR_A_W<'a> {
    w: &'a mut W,
}
impl<'a> FACTOR_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `FACTOR_S`"]
pub type FACTOR_S_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FACTOR_S`"]
pub struct FACTOR_S_W<'a> {
    w: &'a mut W,
}
impl<'a> FACTOR_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:22 - Asynchronous prescaler factor"]
    #[inline(always)]
    pub fn factor_a(&self) -> FACTOR_A_R {
        FACTOR_A_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 0:14 - Synchronous prescaler factor"]
    #[inline(always)]
    pub fn factor_s(&self) -> FACTOR_S_R {
        FACTOR_S_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:22 - Asynchronous prescaler factor"]
    #[inline(always)]
    pub fn factor_a(&mut self) -> FACTOR_A_W {
        FACTOR_A_W { w: self }
    }
    #[doc = "Bits 0:14 - Synchronous prescaler factor"]
    #[inline(always)]
    pub fn factor_s(&mut self) -> FACTOR_S_W {
        FACTOR_S_W { w: self }
    }
}
