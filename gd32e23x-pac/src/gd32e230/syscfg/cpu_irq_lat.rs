#[doc = "Reader of register CPU_IRQ_LAT"]
pub type R = crate::R<u32, super::CPU_IRQ_LAT>;
#[doc = "Writer for register CPU_IRQ_LAT"]
pub type W = crate::W<u32, super::CPU_IRQ_LAT>;
#[doc = "Register CPU_IRQ_LAT `reset()`'s with value 0"]
impl crate::ResetValue for super::CPU_IRQ_LAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IRQ_LATENCY`"]
pub type IRQ_LATENCY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRQ_LATENCY`"]
pub struct IRQ_LATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_LATENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - specifies the minimum number of cycles between an interrupt"]
    #[inline(always)]
    pub fn irq_latency(&self) -> IRQ_LATENCY_R {
        IRQ_LATENCY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - specifies the minimum number of cycles between an interrupt"]
    #[inline(always)]
    pub fn irq_latency(&mut self) -> IRQ_LATENCY_W {
        IRQ_LATENCY_W { w: self }
    }
}
