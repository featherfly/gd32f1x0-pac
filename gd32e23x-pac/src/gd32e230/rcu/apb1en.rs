#[doc = "Reader of register APB1EN"]
pub type R = crate::R<u32, super::APB1EN>;
#[doc = "Writer for register APB1EN"]
pub type W = crate::W<u32, super::APB1EN>;
#[doc = "Register APB1EN `reset()`'s with value 0"]
impl crate::ResetValue for super::APB1EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PMUEN`"]
pub type PMUEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMUEN`"]
pub struct PMUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PMUEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `I2C1EN`"]
pub type I2C1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1EN`"]
pub struct I2C1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1EN_W<'a> {
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
#[doc = "Reader of field `I2C0EN`"]
pub type I2C0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0EN`"]
pub struct I2C0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0EN_W<'a> {
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
#[doc = "Reader of field `USART1EN`"]
pub type USART1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1EN`"]
pub struct USART1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1EN_W<'a> {
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
#[doc = "Reader of field `SPI1EN`"]
pub type SPI1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1EN`"]
pub struct SPI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1EN_W<'a> {
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
#[doc = "Reader of field `WWDGTEN`"]
pub type WWDGTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WWDGTEN`"]
pub struct WWDGTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGTEN_W<'a> {
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
#[doc = "Reader of field `TIMER13EN`"]
pub type TIMER13EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER13EN`"]
pub struct TIMER13EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER13EN_W<'a> {
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
#[doc = "Reader of field `TIMER5EN`"]
pub type TIMER5EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER5EN`"]
pub struct TIMER5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER5EN_W<'a> {
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
#[doc = "Reader of field `TIMER2EN`"]
pub type TIMER2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER2EN`"]
pub struct TIMER2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2EN_W<'a> {
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
impl R {
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pmuen(&self) -> PMUEN_R {
        PMUEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C0 clock enable"]
    #[inline(always)]
    pub fn i2c0en(&self) -> I2C0EN_R {
        I2C0EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    pub fn wwdgten(&self) -> WWDGTEN_R {
        WWDGTEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIMER13 timer clock enable"]
    #[inline(always)]
    pub fn timer13en(&self) -> TIMER13EN_R {
        TIMER13EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIMER5 timer clock enable"]
    #[inline(always)]
    pub fn timer5en(&self) -> TIMER5EN_R {
        TIMER5EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIMER2 timer clock enable"]
    #[inline(always)]
    pub fn timer2en(&self) -> TIMER2EN_R {
        TIMER2EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pmuen(&mut self) -> PMUEN_W {
        PMUEN_W { w: self }
    }
    #[doc = "Bit 22 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W {
        I2C1EN_W { w: self }
    }
    #[doc = "Bit 21 - I2C0 clock enable"]
    #[inline(always)]
    pub fn i2c0en(&mut self) -> I2C0EN_W {
        I2C0EN_W { w: self }
    }
    #[doc = "Bit 17 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W {
        USART1EN_W { w: self }
    }
    #[doc = "Bit 14 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W {
        SPI1EN_W { w: self }
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    pub fn wwdgten(&mut self) -> WWDGTEN_W {
        WWDGTEN_W { w: self }
    }
    #[doc = "Bit 8 - TIMER13 timer clock enable"]
    #[inline(always)]
    pub fn timer13en(&mut self) -> TIMER13EN_W {
        TIMER13EN_W { w: self }
    }
    #[doc = "Bit 4 - TIMER5 timer clock enable"]
    #[inline(always)]
    pub fn timer5en(&mut self) -> TIMER5EN_W {
        TIMER5EN_W { w: self }
    }
    #[doc = "Bit 1 - TIMER2 timer clock enable"]
    #[inline(always)]
    pub fn timer2en(&mut self) -> TIMER2EN_W {
        TIMER2EN_W { w: self }
    }
}
