# [ doc = "Writer for register SWEVG" ] pub type W = crate :: W < u16 , super :: SWEVG > ; # [ doc = "Register SWEVG `reset()`'s with value 0" ] impl crate :: ResetValue for super :: SWEVG { type Type = u16 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Write proxy for field `BRKG`" ] pub struct BRKG_W < 'a > { w : & 'a mut W , } impl < 'a > BRKG_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 7 ) ) | ( ( ( value as u16 ) & 0x01 ) << 7 ) ; self . w } } # [ doc = "Write proxy for field `CMTG`" ] pub struct CMTG_W < 'a > { w : & 'a mut W , } impl < 'a > CMTG_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 5 ) ) | ( ( ( value as u16 ) & 0x01 ) << 5 ) ; self . w } } # [ doc = "Write proxy for field `CH0G`" ] pub struct CH0G_W < 'a > { w : & 'a mut W , } impl < 'a > CH0G_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 1 ) ) | ( ( ( value as u16 ) & 0x01 ) << 1 ) ; self . w } } # [ doc = "Write proxy for field `UPG`" ] pub struct UPG_W < 'a > { w : & 'a mut W , } impl < 'a > UPG_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x01 ) | ( ( value as u16 ) & 0x01 ) ; self . w } } impl W { # [ doc = "Bit 7 - Break event generation" ] # [ inline ( always ) ] pub fn brkg ( & mut self ) -> BRKG_W { BRKG_W { w : self } } # [ doc = "Bit 5 - Channel commutation event generation" ] # [ inline ( always ) ] pub fn cmtg ( & mut self ) -> CMTG_W { CMTG_W { w : self } } # [ doc = "Bit 1 - Channel 0 capture or compare event generation" ] # [ inline ( always ) ] pub fn ch0g ( & mut self ) -> CH0G_W { CH0G_W { w : self } } # [ doc = "Bit 0 - Update generation" ] # [ inline ( always ) ] pub fn upg ( & mut self ) -> UPG_W { UPG_W { w : self } } }