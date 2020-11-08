# [ doc = "Reader of register ADDEN" ] pub type R = crate :: R < u32 , super :: ADDEN > ; # [ doc = "Writer for register ADDEN" ] pub type W = crate :: W < u32 , super :: ADDEN > ; # [ doc = "Register ADDEN `reset()`'s with value 0" ] impl crate :: ResetValue for super :: ADDEN { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `I2C2EN`" ] pub type I2C2EN_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `I2C2EN`" ] pub struct I2C2EN_W < 'a > { w : & 'a mut W , } impl < 'a > I2C2EN_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x01 ) | ( ( value as u32 ) & 0x01 ) ; self . w } } impl R { # [ doc = "Bit 0 - I2C2 unit clock enable" ] # [ inline ( always ) ] pub fn i2c2en ( & self ) -> I2C2EN_R { I2C2EN_R :: new ( ( self . bits & 0x01 ) != 0 ) } } impl W { # [ doc = "Bit 0 - I2C2 unit clock enable" ] # [ inline ( always ) ] pub fn i2c2en ( & mut self ) -> I2C2EN_W { I2C2EN_W { w : self } } }