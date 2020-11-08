#[doc = "Reader of register APB2RST"]
pub type R = crate::R<u32, super::APB2RST>;
#[doc = "Writer for register APB2RST"]
pub type W = crate::W<u32, super::APB2RST>;
#[doc = "Register APB2RST `reset()`'s with value 0"]
impl crate::ResetValue for super::APB2RST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER16RST`"]
pub type TIMER16RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER16RST`"]
pub struct TIMER16RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER16RST_W<'a> {
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
#[doc = "Reader of field `TIMER15RST`"]
pub type TIMER15RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER15RST`"]
pub struct TIMER15RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER15RST_W<'a> {
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
#[doc = "Reader of field `TIMER14RST`"]
pub type TIMER14RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER14RST`"]
pub struct TIMER14RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER14RST_W<'a> {
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
#[doc = "Reader of field `USART0RST`"]
pub type USART0RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART0RST`"]
pub struct USART0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USART0RST_W<'a> {
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
#[doc = "Reader of field `SPI0RST`"]
pub type SPI0RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI0RST`"]
pub struct SPI0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0RST_W<'a> {
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
#[doc = "Reader of field `TIMER0RST`"]
pub type TIMER0RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER0RST`"]
pub struct TIMER0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0RST_W<'a> {
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
#[doc = "Reader of field `ADCRST`"]
pub type ADCRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCRST`"]
pub struct ADCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCRST_W<'a> {
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
#[doc = "Reader of field `CFGCMPRST`"]
pub type CFGCMPRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFGCMPRST`"]
pub struct CFGCMPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGCMPRST_W<'a> {
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
    #[doc = "Bit 18 - TIMER16 reset"]
    #[inline(always)]
    pub fn timer16rst(&self) -> TIMER16RST_R {
        TIMER16RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIMER15 reset"]
    #[inline(always)]
    pub fn timer15rst(&self) -> TIMER15RST_R {
        TIMER15RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TIMER14 reset"]
    #[inline(always)]
    pub fn timer14rst(&self) -> TIMER14RST_R {
        TIMER14RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    pub fn usart0rst(&self) -> USART0RST_R {
        USART0RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI0 Reset"]
    #[inline(always)]
    pub fn spi0rst(&self) -> SPI0RST_R {
        SPI0RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TIMER0 reset"]
    #[inline(always)]
    pub fn timer0rst(&self) -> TIMER0RST_R {
        TIMER0RST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 0 - System configuration and comparator reset"]
    #[inline(always)]
    pub fn cfgcmprst(&self) -> CFGCMPRST_R {
        CFGCMPRST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - TIMER16 reset"]
    #[inline(always)]
    pub fn timer16rst(&mut self) -> TIMER16RST_W {
        TIMER16RST_W { w: self }
    }
    #[doc = "Bit 17 - TIMER15 reset"]
    #[inline(always)]
    pub fn timer15rst(&mut self) -> TIMER15RST_W {
        TIMER15RST_W { w: self }
    }
    #[doc = "Bit 16 - TIMER14 reset"]
    #[inline(always)]
    pub fn timer14rst(&mut self) -> TIMER14RST_W {
        TIMER14RST_W { w: self }
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    pub fn usart0rst(&mut self) -> USART0RST_W {
        USART0RST_W { w: self }
    }
    #[doc = "Bit 12 - SPI0 Reset"]
    #[inline(always)]
    pub fn spi0rst(&mut self) -> SPI0RST_W {
        SPI0RST_W { w: self }
    }
    #[doc = "Bit 11 - TIMER0 reset"]
    #[inline(always)]
    pub fn timer0rst(&mut self) -> TIMER0RST_W {
        TIMER0RST_W { w: self }
    }
    #[doc = "Bit 9 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W {
        ADCRST_W { w: self }
    }
    #[doc = "Bit 0 - System configuration and comparator reset"]
    #[inline(always)]
    pub fn cfgcmprst(&mut self) -> CFGCMPRST_W {
        CFGCMPRST_W { w: self }
    }
}
