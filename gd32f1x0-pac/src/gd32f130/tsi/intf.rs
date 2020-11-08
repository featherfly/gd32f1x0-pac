# [ doc = "Reader of register INTF" ] pub type R = crate :: R < u32 , super :: INTF > ; # [ doc = "Writer for register INTF" ] pub type W = crate :: W < u32 , super :: INTF > ; # [ doc = "Register INTF `reset()`'s with value 0" ] impl crate :: ResetValue for super :: INTF { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `MNERR`" ] pub type MNERR_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `MNERR`" ] pub struct MNERR_W < 'a > { w : & 'a mut W , } impl < 'a > MNERR_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 1 ) ) | ( ( ( value as u32 ) & 0x01 ) << 1 ) ; self . w } } # [ doc = "Reader of field `CTCF`" ] pub type CTCF_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `CTCF`" ] pub struct CTCF_W < 'a > { w : & 'a mut W , } impl < 'a > CTCF_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x01 ) | ( ( value as u32 ) & 0x01 ) ; self . w } } impl R { # [ doc = "Bit 1 - Max count error flag" ] # [ inline ( always ) ] pub fn mnerr ( & self ) -> MNERR_R { MNERR_R :: new ( ( ( self . bits >> 1 ) & 0x01 ) != 0 ) } # [ doc = "Bit 0 - Charge-Transfer complete flag" ] # [ inline ( always ) ] pub fn ctcf ( & self ) -> CTCF_R { CTCF_R :: new ( ( self . bits & 0x01 ) != 0 ) } } impl W { # [ doc = "Bit 1 - Max count error flag" ] # [ inline ( always ) ] pub fn mnerr ( & mut self ) -> MNERR_W { MNERR_W { w : self } } # [ doc = "Bit 0 - Charge-Transfer complete flag" ] # [ inline ( always ) ] pub fn ctcf ( & mut self ) -> CTCF_W { CTCF_W { w : self } } }