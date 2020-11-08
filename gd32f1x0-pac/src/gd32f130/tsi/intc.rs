# [ doc = "Reader of register INTC" ] pub type R = crate :: R < u32 , super :: INTC > ; # [ doc = "Writer for register INTC" ] pub type W = crate :: W < u32 , super :: INTC > ; # [ doc = "Register INTC `reset()`'s with value 0" ] impl crate :: ResetValue for super :: INTC { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `CMNERR`" ] pub type CMNERR_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `CMNERR`" ] pub struct CMNERR_W < 'a > { w : & 'a mut W , } impl < 'a > CMNERR_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 1 ) ) | ( ( ( value as u32 ) & 0x01 ) << 1 ) ; self . w } } # [ doc = "Reader of field `CCTCF`" ] pub type CCTCF_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `CCTCF`" ] pub struct CCTCF_W < 'a > { w : & 'a mut W , } impl < 'a > CCTCF_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x01 ) | ( ( value as u32 ) & 0x01 ) ; self . w } } impl R { # [ doc = "Bit 1 - Clear max cycle number error" ] # [ inline ( always ) ] pub fn cmnerr ( & self ) -> CMNERR_R { CMNERR_R :: new ( ( ( self . bits >> 1 ) & 0x01 ) != 0 ) } # [ doc = "Bit 0 - Clear charge-transfer complete flag" ] # [ inline ( always ) ] pub fn cctcf ( & self ) -> CCTCF_R { CCTCF_R :: new ( ( self . bits & 0x01 ) != 0 ) } } impl W { # [ doc = "Bit 1 - Clear max cycle number error" ] # [ inline ( always ) ] pub fn cmnerr ( & mut self ) -> CMNERR_W { CMNERR_W { w : self } } # [ doc = "Bit 0 - Clear charge-transfer complete flag" ] # [ inline ( always ) ] pub fn cctcf ( & mut self ) -> CCTCF_W { CCTCF_W { w : self } } }