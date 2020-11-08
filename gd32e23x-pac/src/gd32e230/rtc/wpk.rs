#[doc = "Writer for register WPK"]
pub type W = crate::W<u32, super::WPK>;
#[doc = "Register WPK `reset()`'s with value 0"]
impl crate::ResetValue for super::WPK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `WPK`"]
pub struct WPK_W<'a> {
    w: &'a mut W,
}
impl<'a> WPK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Write protection key"]
    #[inline(always)]
    pub fn wpk(&mut self) -> WPK_W {
        WPK_W { w: self }
    }
}
