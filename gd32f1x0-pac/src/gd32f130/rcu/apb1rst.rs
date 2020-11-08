# [ doc = "Reader of register APB1RST" ] pub type R = crate :: R < u32 , super :: APB1RST > ; # [ doc = "Writer for register APB1RST" ] pub type W = crate :: W < u32 , super :: APB1RST > ; # [ doc = "Register APB1RST `reset()`'s with value 0" ] impl crate :: ResetValue for super :: APB1RST { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `TIMER1RST`" ] pub type TIMER1RST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `TIMER1RST`" ] pub struct TIMER1RST_W < 'a > { w : & 'a mut W , } impl < 'a > TIMER1RST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x01 ) | ( ( value as u32 ) & 0x01 ) ; self . w } } # [ doc = "Reader of field `TIMER2RST`" ] pub type TIMER2RST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `TIMER2RST`" ] pub struct TIMER2RST_W < 'a > { w : & 'a mut W , } impl < 'a > TIMER2RST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 1 ) ) | ( ( ( value as u32 ) & 0x01 ) << 1 ) ; self . w } } # [ doc = "Reader of field `TIMER5RST`" ] pub type TIMER5RST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `TIMER5RST`" ] pub struct TIMER5RST_W < 'a > { w : & 'a mut W , } impl < 'a > TIMER5RST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 4 ) ) | ( ( ( value as u32 ) & 0x01 ) << 4 ) ; self . w } } # [ doc = "Reader of field `TIMER13RST`" ] pub type TIMER13RST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `TIMER13RST`" ] pub struct TIMER13RST_W < 'a > { w : & 'a mut W , } impl < 'a > TIMER13RST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 8 ) ) | ( ( ( value as u32 ) & 0x01 ) << 8 ) ; self . w } } # [ doc = "Reader of field `SLCDRST`" ] pub type SLCDRST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `SLCDRST`" ] pub struct SLCDRST_W < 'a > { w : & 'a mut W , } impl < 'a > SLCDRST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 9 ) ) | ( ( ( value as u32 ) & 0x01 ) << 9 ) ; self . w } } # [ doc = "Reader of field `WWDGTRST`" ] pub type WWDGTRST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `WWDGTRST`" ] pub struct WWDGTRST_W < 'a > { w : & 'a mut W , } impl < 'a > WWDGTRST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 11 ) ) | ( ( ( value as u32 ) & 0x01 ) << 11 ) ; self . w } } # [ doc = "Reader of field `SPI1RST`" ] pub type SPI1RST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `SPI1RST`" ] pub struct SPI1RST_W < 'a > { w : & 'a mut W , } impl < 'a > SPI1RST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 14 ) ) | ( ( ( value as u32 ) & 0x01 ) << 14 ) ; self . w } } # [ doc = "Reader of field `SPI2RST`" ] pub type SPI2RST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `SPI2RST`" ] pub struct SPI2RST_W < 'a > { w : & 'a mut W , } impl < 'a > SPI2RST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 15 ) ) | ( ( ( value as u32 ) & 0x01 ) << 15 ) ; self . w } } # [ doc = "Reader of field `USART1RST`" ] pub type USART1RST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `USART1RST`" ] pub struct USART1RST_W < 'a > { w : & 'a mut W , } impl < 'a > USART1RST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 17 ) ) | ( ( ( value as u32 ) & 0x01 ) << 17 ) ; self . w } } # [ doc = "Reader of field `I2C0RST`" ] pub type I2C0RST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `I2C0RST`" ] pub struct I2C0RST_W < 'a > { w : & 'a mut W , } impl < 'a > I2C0RST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 21 ) ) | ( ( ( value as u32 ) & 0x01 ) << 21 ) ; self . w } } # [ doc = "Reader of field `I2C1RST`" ] pub type I2C1RST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `I2C1RST`" ] pub struct I2C1RST_W < 'a > { w : & 'a mut W , } impl < 'a > I2C1RST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 22 ) ) | ( ( ( value as u32 ) & 0x01 ) << 22 ) ; self . w } } # [ doc = "Reader of field `USBDRST`" ] pub type USBDRST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `USBDRST`" ] pub struct USBDRST_W < 'a > { w : & 'a mut W , } impl < 'a > USBDRST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 23 ) ) | ( ( ( value as u32 ) & 0x01 ) << 23 ) ; self . w } } # [ doc = "Reader of field `CAN0RST`" ] pub type CAN0RST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `CAN0RST`" ] pub struct CAN0RST_W < 'a > { w : & 'a mut W , } impl < 'a > CAN0RST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 25 ) ) | ( ( ( value as u32 ) & 0x01 ) << 25 ) ; self . w } } # [ doc = "Reader of field `CAN1RST`" ] pub type CAN1RST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `CAN1RST`" ] pub struct CAN1RST_W < 'a > { w : & 'a mut W , } impl < 'a > CAN1RST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 26 ) ) | ( ( ( value as u32 ) & 0x01 ) << 26 ) ; self . w } } # [ doc = "Reader of field `PMURST`" ] pub type PMURST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `PMURST`" ] pub struct PMURST_W < 'a > { w : & 'a mut W , } impl < 'a > PMURST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 28 ) ) | ( ( ( value as u32 ) & 0x01 ) << 28 ) ; self . w } } # [ doc = "Reader of field `DACRST`" ] pub type DACRST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `DACRST`" ] pub struct DACRST_W < 'a > { w : & 'a mut W , } impl < 'a > DACRST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 29 ) ) | ( ( ( value as u32 ) & 0x01 ) << 29 ) ; self . w } } # [ doc = "Reader of field `CECRST`" ] pub type CECRST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `CECRST`" ] pub struct CECRST_W < 'a > { w : & 'a mut W , } impl < 'a > CECRST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 30 ) ) | ( ( ( value as u32 ) & 0x01 ) << 30 ) ; self . w } } # [ doc = "Reader of field `OPAIVREFRST`" ] pub type OPAIVREFRST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `OPAIVREFRST`" ] pub struct OPAIVREFRST_W < 'a > { w : & 'a mut W , } impl < 'a > OPAIVREFRST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 31 ) ) | ( ( ( value as u32 ) & 0x01 ) << 31 ) ; self . w } } impl R { # [ doc = "Bit 0 - TIMER1 timer reset" ] # [ inline ( always ) ] pub fn timer1rst ( & self ) -> TIMER1RST_R { TIMER1RST_R :: new ( ( self . bits & 0x01 ) != 0 ) } # [ doc = "Bit 1 - TIMER2 timer reset" ] # [ inline ( always ) ] pub fn timer2rst ( & self ) -> TIMER2RST_R { TIMER2RST_R :: new ( ( ( self . bits >> 1 ) & 0x01 ) != 0 ) } # [ doc = "Bit 4 - TIMER5 timer reset" ] # [ inline ( always ) ] pub fn timer5rst ( & self ) -> TIMER5RST_R { TIMER5RST_R :: new ( ( ( self . bits >> 4 ) & 0x01 ) != 0 ) } # [ doc = "Bit 8 - TIMER13 timer reset" ] # [ inline ( always ) ] pub fn timer13rst ( & self ) -> TIMER13RST_R { TIMER13RST_R :: new ( ( ( self . bits >> 8 ) & 0x01 ) != 0 ) } # [ doc = "Bit 9 - SLCD reset" ] # [ inline ( always ) ] pub fn slcdrst ( & self ) -> SLCDRST_R { SLCDRST_R :: new ( ( ( self . bits >> 9 ) & 0x01 ) != 0 ) } # [ doc = "Bit 11 - Window watchdog timer reset" ] # [ inline ( always ) ] pub fn wwdgtrst ( & self ) -> WWDGTRST_R { WWDGTRST_R :: new ( ( ( self . bits >> 11 ) & 0x01 ) != 0 ) } # [ doc = "Bit 14 - SPI1 reset" ] # [ inline ( always ) ] pub fn spi1rst ( & self ) -> SPI1RST_R { SPI1RST_R :: new ( ( ( self . bits >> 14 ) & 0x01 ) != 0 ) } # [ doc = "Bit 15 - SPI2 reset" ] # [ inline ( always ) ] pub fn spi2rst ( & self ) -> SPI2RST_R { SPI2RST_R :: new ( ( ( self . bits >> 15 ) & 0x01 ) != 0 ) } # [ doc = "Bit 17 - USART1 reset" ] # [ inline ( always ) ] pub fn usart1rst ( & self ) -> USART1RST_R { USART1RST_R :: new ( ( ( self . bits >> 17 ) & 0x01 ) != 0 ) } # [ doc = "Bit 21 - I2C0 reset" ] # [ inline ( always ) ] pub fn i2c0rst ( & self ) -> I2C0RST_R { I2C0RST_R :: new ( ( ( self . bits >> 21 ) & 0x01 ) != 0 ) } # [ doc = "Bit 22 - I2C1 reset" ] # [ inline ( always ) ] pub fn i2c1rst ( & self ) -> I2C1RST_R { I2C1RST_R :: new ( ( ( self . bits >> 22 ) & 0x01 ) != 0 ) } # [ doc = "Bit 23 - USBD reset" ] # [ inline ( always ) ] pub fn usbdrst ( & self ) -> USBDRST_R { USBDRST_R :: new ( ( ( self . bits >> 23 ) & 0x01 ) != 0 ) } # [ doc = "Bit 25 - CAN0 reset" ] # [ inline ( always ) ] pub fn can0rst ( & self ) -> CAN0RST_R { CAN0RST_R :: new ( ( ( self . bits >> 25 ) & 0x01 ) != 0 ) } # [ doc = "Bit 26 - CAN1 reset" ] # [ inline ( always ) ] pub fn can1rst ( & self ) -> CAN1RST_R { CAN1RST_R :: new ( ( ( self . bits >> 26 ) & 0x01 ) != 0 ) } # [ doc = "Bit 28 - Power control reset" ] # [ inline ( always ) ] pub fn pmurst ( & self ) -> PMURST_R { PMURST_R :: new ( ( ( self . bits >> 28 ) & 0x01 ) != 0 ) } # [ doc = "Bit 29 - DAC reset" ] # [ inline ( always ) ] pub fn dacrst ( & self ) -> DACRST_R { DACRST_R :: new ( ( ( self . bits >> 29 ) & 0x01 ) != 0 ) } # [ doc = "Bit 30 - HDMI CEC reset" ] # [ inline ( always ) ] pub fn cecrst ( & self ) -> CECRST_R { CECRST_R :: new ( ( ( self . bits >> 30 ) & 0x01 ) != 0 ) } # [ doc = "Bit 31 - OPA and IVREF reset" ] # [ inline ( always ) ] pub fn opaivrefrst ( & self ) -> OPAIVREFRST_R { OPAIVREFRST_R :: new ( ( ( self . bits >> 31 ) & 0x01 ) != 0 ) } } impl W { # [ doc = "Bit 0 - TIMER1 timer reset" ] # [ inline ( always ) ] pub fn timer1rst ( & mut self ) -> TIMER1RST_W { TIMER1RST_W { w : self } } # [ doc = "Bit 1 - TIMER2 timer reset" ] # [ inline ( always ) ] pub fn timer2rst ( & mut self ) -> TIMER2RST_W { TIMER2RST_W { w : self } } # [ doc = "Bit 4 - TIMER5 timer reset" ] # [ inline ( always ) ] pub fn timer5rst ( & mut self ) -> TIMER5RST_W { TIMER5RST_W { w : self } } # [ doc = "Bit 8 - TIMER13 timer reset" ] # [ inline ( always ) ] pub fn timer13rst ( & mut self ) -> TIMER13RST_W { TIMER13RST_W { w : self } } # [ doc = "Bit 9 - SLCD reset" ] # [ inline ( always ) ] pub fn slcdrst ( & mut self ) -> SLCDRST_W { SLCDRST_W { w : self } } # [ doc = "Bit 11 - Window watchdog timer reset" ] # [ inline ( always ) ] pub fn wwdgtrst ( & mut self ) -> WWDGTRST_W { WWDGTRST_W { w : self } } # [ doc = "Bit 14 - SPI1 reset" ] # [ inline ( always ) ] pub fn spi1rst ( & mut self ) -> SPI1RST_W { SPI1RST_W { w : self } } # [ doc = "Bit 15 - SPI2 reset" ] # [ inline ( always ) ] pub fn spi2rst ( & mut self ) -> SPI2RST_W { SPI2RST_W { w : self } } # [ doc = "Bit 17 - USART1 reset" ] # [ inline ( always ) ] pub fn usart1rst ( & mut self ) -> USART1RST_W { USART1RST_W { w : self } } # [ doc = "Bit 21 - I2C0 reset" ] # [ inline ( always ) ] pub fn i2c0rst ( & mut self ) -> I2C0RST_W { I2C0RST_W { w : self } } # [ doc = "Bit 22 - I2C1 reset" ] # [ inline ( always ) ] pub fn i2c1rst ( & mut self ) -> I2C1RST_W { I2C1RST_W { w : self } } # [ doc = "Bit 23 - USBD reset" ] # [ inline ( always ) ] pub fn usbdrst ( & mut self ) -> USBDRST_W { USBDRST_W { w : self } } # [ doc = "Bit 25 - CAN0 reset" ] # [ inline ( always ) ] pub fn can0rst ( & mut self ) -> CAN0RST_W { CAN0RST_W { w : self } } # [ doc = "Bit 26 - CAN1 reset" ] # [ inline ( always ) ] pub fn can1rst ( & mut self ) -> CAN1RST_W { CAN1RST_W { w : self } } # [ doc = "Bit 28 - Power control reset" ] # [ inline ( always ) ] pub fn pmurst ( & mut self ) -> PMURST_W { PMURST_W { w : self } } # [ doc = "Bit 29 - DAC reset" ] # [ inline ( always ) ] pub fn dacrst ( & mut self ) -> DACRST_W { DACRST_W { w : self } } # [ doc = "Bit 30 - HDMI CEC reset" ] # [ inline ( always ) ] pub fn cecrst ( & mut self ) -> CECRST_W { CECRST_W { w : self } } # [ doc = "Bit 31 - OPA and IVREF reset" ] # [ inline ( always ) ] pub fn opaivrefrst ( & mut self ) -> OPAIVREFRST_W { OPAIVREFRST_W { w : self } } }