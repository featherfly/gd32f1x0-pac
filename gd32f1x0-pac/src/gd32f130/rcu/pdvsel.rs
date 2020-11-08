# [ doc = "Reader of register PDVSEL" ] pub type R = crate :: R < u32 , super :: PDVSEL > ; # [ doc = "Writer for register PDVSEL" ] pub type W = crate :: W < u32 , super :: PDVSEL > ; # [ doc = "Register PDVSEL `reset()`'s with value 0" ] impl crate :: ResetValue for super :: PDVSEL { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `PDRVS`" ] pub type PDRVS_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `PDRVS`" ] pub struct PDRVS_W < 'a > { w : & 'a mut W , } impl < 'a > PDRVS_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x01 ) | ( ( value as u32 ) & 0x01 ) ; self . w } } impl R { # [ doc = "Bit 0 - Power down voltage select" ] # [ inline ( always ) ] pub fn pdrvs ( & self ) -> PDRVS_R { PDRVS_R :: new ( ( self . bits & 0x01 ) != 0 ) } } impl W { # [ doc = "Bit 0 - Power down voltage select" ] # [ inline ( always ) ] pub fn pdrvs ( & mut self ) -> PDRVS_W { PDRVS_W { w : self } } }