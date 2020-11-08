# [ doc = "Reader of register INTEN" ] pub type R = crate :: R < u32 , super :: INTEN > ; # [ doc = "Writer for register INTEN" ] pub type W = crate :: W < u32 , super :: INTEN > ; # [ doc = "Register INTEN `reset()`'s with value 0" ] impl crate :: ResetValue for super :: INTEN { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `MNERRIE`" ] pub type MNERRIE_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `MNERRIE`" ] pub struct MNERRIE_W < 'a > { w : & 'a mut W , } impl < 'a > MNERRIE_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 1 ) ) | ( ( ( value as u32 ) & 0x01 ) << 1 ) ; self . w } } # [ doc = "Reader of field `CTCFIE`" ] pub type CTCFIE_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `CTCFIE`" ] pub struct CTCFIE_W < 'a > { w : & 'a mut W , } impl < 'a > CTCFIE_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x01 ) | ( ( value as u32 ) & 0x01 ) ; self . w } } impl R { # [ doc = "Bit 1 - Max Cycle Number Error Interrupt Enable" ] # [ inline ( always ) ] pub fn mnerrie ( & self ) -> MNERRIE_R { MNERRIE_R :: new ( ( ( self . bits >> 1 ) & 0x01 ) != 0 ) } # [ doc = "Bit 0 - Charge-transfer complete flag Interrupt Enable" ] # [ inline ( always ) ] pub fn ctcfie ( & self ) -> CTCFIE_R { CTCFIE_R :: new ( ( self . bits & 0x01 ) != 0 ) } } impl W { # [ doc = "Bit 1 - Max Cycle Number Error Interrupt Enable" ] # [ inline ( always ) ] pub fn mnerrie ( & mut self ) -> MNERRIE_W { MNERRIE_W { w : self } } # [ doc = "Bit 0 - Charge-transfer complete flag Interrupt Enable" ] # [ inline ( always ) ] pub fn ctcfie ( & mut self ) -> CTCFIE_W { CTCFIE_W { w : self } } }