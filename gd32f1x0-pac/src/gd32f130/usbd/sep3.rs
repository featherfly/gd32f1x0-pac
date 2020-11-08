# [ doc = "Reader of register SEP3" ] pub type R = crate :: R < u16 , super :: SEP3 > ; # [ doc = "Writer for register SEP3" ] pub type W = crate :: W < u16 , super :: SEP3 > ; # [ doc = "Register SEP3 `reset()`'s with value 0" ] impl crate :: ResetValue for super :: SEP3 { type Type = u16 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `SUB_ST`" ] pub type SUB_ST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `SUB_ST`" ] pub struct SUB_ST_W < 'a > { w : & 'a mut W , } impl < 'a > SUB_ST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 15 ) ) | ( ( ( value as u16 ) & 0x01 ) << 15 ) ; self . w } } # [ doc = "Reader of field `SUB_STA`" ] pub type SUB_STA_R = crate :: R < u8 , u8 > ; # [ doc = "Write proxy for field `SUB_STA`" ] pub struct SUB_STA_W < 'a > { w : & 'a mut W , } impl < 'a > SUB_STA_W < 'a > { # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub unsafe fn bits ( self , value : u8 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x03 << 12 ) ) | ( ( ( value as u16 ) & 0x03 ) << 12 ) ; self . w } } # [ doc = "Reader of field `SUBPID_ATTR`" ] pub type SUBPID_ATTR_R = crate :: R < u16 , u16 > ; # [ doc = "Write proxy for field `SUBPID_ATTR`" ] pub struct SUBPID_ATTR_W < 'a > { w : & 'a mut W , } impl < 'a > SUBPID_ATTR_W < 'a > { # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub unsafe fn bits ( self , value : u16 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x07ff ) | ( ( value as u16 ) & 0x07ff ) ; self . w } } impl R { # [ doc = "Bit 15 - Successful Receive for LPM Token" ] # [ inline ( always ) ] pub fn sub_st ( & self ) -> SUB_ST_R { SUB_ST_R :: new ( ( ( self . bits >> 15 ) & 0x01 ) != 0 ) } # [ doc = "Bits 12:13 - Status bits, for the handshake of receiving subpid LPM" ] # [ inline ( always ) ] pub fn sub_sta ( & self ) -> SUB_STA_R { SUB_STA_R :: new ( ( ( self . bits >> 12 ) & 0x03 ) as u8 ) } # [ doc = "Bits 0:10 - LPM Token bmAttribute Field." ] # [ inline ( always ) ] pub fn subpid_attr ( & self ) -> SUBPID_ATTR_R { SUBPID_ATTR_R :: new ( ( self . bits & 0x07ff ) as u16 ) } } impl W { # [ doc = "Bit 15 - Successful Receive for LPM Token" ] # [ inline ( always ) ] pub fn sub_st ( & mut self ) -> SUB_ST_W { SUB_ST_W { w : self } } # [ doc = "Bits 12:13 - Status bits, for the handshake of receiving subpid LPM" ] # [ inline ( always ) ] pub fn sub_sta ( & mut self ) -> SUB_STA_W { SUB_STA_W { w : self } } # [ doc = "Bits 0:10 - LPM Token bmAttribute Field." ] # [ inline ( always ) ] pub fn subpid_attr ( & mut self ) -> SUBPID_ATTR_W { SUBPID_ATTR_W { w : self } } }