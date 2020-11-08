# [ doc = "Reader of register CFG" ] pub type R = crate :: R < u16 , super :: CFG > ; # [ doc = "Writer for register CFG" ] pub type W = crate :: W < u16 , super :: CFG > ; # [ doc = "Register CFG `reset()`'s with value 0" ] impl crate :: ResetValue for super :: CFG { type Type = u16 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `CCSEL`" ] pub type CCSEL_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `CCSEL`" ] pub struct CCSEL_W < 'a > { w : & 'a mut W , } impl < 'a > CCSEL_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 1 ) ) | ( ( ( value as u16 ) & 0x01 ) << 1 ) ; self . w } } # [ doc = "Reader of field `OUTSEL`" ] pub type OUTSEL_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `OUTSEL`" ] pub struct OUTSEL_W < 'a > { w : & 'a mut W , } impl < 'a > OUTSEL_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x01 ) | ( ( value as u16 ) & 0x01 ) ; self . w } } impl R { # [ doc = "Bit 1 - Write Capture/Compare register selection" ] # [ inline ( always ) ] pub fn ccsel ( & self ) -> CCSEL_R { CCSEL_R :: new ( ( ( self . bits >> 1 ) & 0x01 ) != 0 ) } # [ doc = "Bit 0 - The output value selection" ] # [ inline ( always ) ] pub fn outsel ( & self ) -> OUTSEL_R { OUTSEL_R :: new ( ( self . bits & 0x01 ) != 0 ) } } impl W { # [ doc = "Bit 1 - Write Capture/Compare register selection" ] # [ inline ( always ) ] pub fn ccsel ( & mut self ) -> CCSEL_W { CCSEL_W { w : self } } # [ doc = "Bit 0 - The output value selection" ] # [ inline ( always ) ] pub fn outsel ( & mut self ) -> OUTSEL_W { OUTSEL_W { w : self } } }