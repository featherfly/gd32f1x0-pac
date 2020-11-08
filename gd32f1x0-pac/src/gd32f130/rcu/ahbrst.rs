# [ doc = "Reader of register AHBRST" ] pub type R = crate :: R < u32 , super :: AHBRST > ; # [ doc = "Writer for register AHBRST" ] pub type W = crate :: W < u32 , super :: AHBRST > ; # [ doc = "Register AHBRST `reset()`'s with value 0" ] impl crate :: ResetValue for super :: AHBRST { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `PARST`" ] pub type PARST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `PARST`" ] pub struct PARST_W < 'a > { w : & 'a mut W , } impl < 'a > PARST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 17 ) ) | ( ( ( value as u32 ) & 0x01 ) << 17 ) ; self . w } } # [ doc = "Reader of field `PBRST`" ] pub type PBRST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `PBRST`" ] pub struct PBRST_W < 'a > { w : & 'a mut W , } impl < 'a > PBRST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 18 ) ) | ( ( ( value as u32 ) & 0x01 ) << 18 ) ; self . w } } # [ doc = "Reader of field `PCRST`" ] pub type PCRST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `PCRST`" ] pub struct PCRST_W < 'a > { w : & 'a mut W , } impl < 'a > PCRST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 19 ) ) | ( ( ( value as u32 ) & 0x01 ) << 19 ) ; self . w } } # [ doc = "Reader of field `PDRST`" ] pub type PDRST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `PDRST`" ] pub struct PDRST_W < 'a > { w : & 'a mut W , } impl < 'a > PDRST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 20 ) ) | ( ( ( value as u32 ) & 0x01 ) << 20 ) ; self . w } } # [ doc = "Reader of field `PFRST`" ] pub type PFRST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `PFRST`" ] pub struct PFRST_W < 'a > { w : & 'a mut W , } impl < 'a > PFRST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 22 ) ) | ( ( ( value as u32 ) & 0x01 ) << 22 ) ; self . w } } # [ doc = "Reader of field `TSIRST`" ] pub type TSIRST_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `TSIRST`" ] pub struct TSIRST_W < 'a > { w : & 'a mut W , } impl < 'a > TSIRST_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 24 ) ) | ( ( ( value as u32 ) & 0x01 ) << 24 ) ; self . w } } impl R { # [ doc = "Bit 17 - GPIO port A reset" ] # [ inline ( always ) ] pub fn parst ( & self ) -> PARST_R { PARST_R :: new ( ( ( self . bits >> 17 ) & 0x01 ) != 0 ) } # [ doc = "Bit 18 - GPIO port B reset" ] # [ inline ( always ) ] pub fn pbrst ( & self ) -> PBRST_R { PBRST_R :: new ( ( ( self . bits >> 18 ) & 0x01 ) != 0 ) } # [ doc = "Bit 19 - GPIO port C reset" ] # [ inline ( always ) ] pub fn pcrst ( & self ) -> PCRST_R { PCRST_R :: new ( ( ( self . bits >> 19 ) & 0x01 ) != 0 ) } # [ doc = "Bit 20 - GPIO port D reset" ] # [ inline ( always ) ] pub fn pdrst ( & self ) -> PDRST_R { PDRST_R :: new ( ( ( self . bits >> 20 ) & 0x01 ) != 0 ) } # [ doc = "Bit 22 - GPIO port F reset" ] # [ inline ( always ) ] pub fn pfrst ( & self ) -> PFRST_R { PFRST_R :: new ( ( ( self . bits >> 22 ) & 0x01 ) != 0 ) } # [ doc = "Bit 24 - TSI unit reset" ] # [ inline ( always ) ] pub fn tsirst ( & self ) -> TSIRST_R { TSIRST_R :: new ( ( ( self . bits >> 24 ) & 0x01 ) != 0 ) } } impl W { # [ doc = "Bit 17 - GPIO port A reset" ] # [ inline ( always ) ] pub fn parst ( & mut self ) -> PARST_W { PARST_W { w : self } } # [ doc = "Bit 18 - GPIO port B reset" ] # [ inline ( always ) ] pub fn pbrst ( & mut self ) -> PBRST_W { PBRST_W { w : self } } # [ doc = "Bit 19 - GPIO port C reset" ] # [ inline ( always ) ] pub fn pcrst ( & mut self ) -> PCRST_W { PCRST_W { w : self } } # [ doc = "Bit 20 - GPIO port D reset" ] # [ inline ( always ) ] pub fn pdrst ( & mut self ) -> PDRST_W { PDRST_W { w : self } } # [ doc = "Bit 22 - GPIO port F reset" ] # [ inline ( always ) ] pub fn pfrst ( & mut self ) -> PFRST_W { PFRST_W { w : self } } # [ doc = "Bit 24 - TSI unit reset" ] # [ inline ( always ) ] pub fn tsirst ( & mut self ) -> TSIRST_W { TSIRST_W { w : self } } }