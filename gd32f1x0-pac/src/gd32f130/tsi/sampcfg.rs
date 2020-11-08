# [ doc = "Reader of register SAMPCFG" ] pub type R = crate :: R < u32 , super :: SAMPCFG > ; # [ doc = "Writer for register SAMPCFG" ] pub type W = crate :: W < u32 , super :: SAMPCFG > ; # [ doc = "Register SAMPCFG `reset()`'s with value 0" ] impl crate :: ResetValue for super :: SAMPCFG { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `G5P3`" ] pub type G5P3_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G5P3`" ] pub struct G5P3_W < 'a > { w : & 'a mut W , } impl < 'a > G5P3_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 23 ) ) | ( ( ( value as u32 ) & 0x01 ) << 23 ) ; self . w } } # [ doc = "Reader of field `G5P2`" ] pub type G5P2_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G5P2`" ] pub struct G5P2_W < 'a > { w : & 'a mut W , } impl < 'a > G5P2_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 22 ) ) | ( ( ( value as u32 ) & 0x01 ) << 22 ) ; self . w } } # [ doc = "Reader of field `G5P1`" ] pub type G5P1_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G5P1`" ] pub struct G5P1_W < 'a > { w : & 'a mut W , } impl < 'a > G5P1_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 21 ) ) | ( ( ( value as u32 ) & 0x01 ) << 21 ) ; self . w } } # [ doc = "Reader of field `G5P0`" ] pub type G5P0_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G5P0`" ] pub struct G5P0_W < 'a > { w : & 'a mut W , } impl < 'a > G5P0_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 20 ) ) | ( ( ( value as u32 ) & 0x01 ) << 20 ) ; self . w } } # [ doc = "Reader of field `G4P3`" ] pub type G4P3_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G4P3`" ] pub struct G4P3_W < 'a > { w : & 'a mut W , } impl < 'a > G4P3_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 19 ) ) | ( ( ( value as u32 ) & 0x01 ) << 19 ) ; self . w } } # [ doc = "Reader of field `G4P2`" ] pub type G4P2_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G4P2`" ] pub struct G4P2_W < 'a > { w : & 'a mut W , } impl < 'a > G4P2_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 18 ) ) | ( ( ( value as u32 ) & 0x01 ) << 18 ) ; self . w } } # [ doc = "Reader of field `G4P1`" ] pub type G4P1_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G4P1`" ] pub struct G4P1_W < 'a > { w : & 'a mut W , } impl < 'a > G4P1_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 17 ) ) | ( ( ( value as u32 ) & 0x01 ) << 17 ) ; self . w } } # [ doc = "Reader of field `G4P0`" ] pub type G4P0_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G4P0`" ] pub struct G4P0_W < 'a > { w : & 'a mut W , } impl < 'a > G4P0_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 16 ) ) | ( ( ( value as u32 ) & 0x01 ) << 16 ) ; self . w } } # [ doc = "Reader of field `G3P3`" ] pub type G3P3_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G3P3`" ] pub struct G3P3_W < 'a > { w : & 'a mut W , } impl < 'a > G3P3_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 15 ) ) | ( ( ( value as u32 ) & 0x01 ) << 15 ) ; self . w } } # [ doc = "Reader of field `G3P2`" ] pub type G3P2_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G3P2`" ] pub struct G3P2_W < 'a > { w : & 'a mut W , } impl < 'a > G3P2_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 14 ) ) | ( ( ( value as u32 ) & 0x01 ) << 14 ) ; self . w } } # [ doc = "Reader of field `G3P1`" ] pub type G3P1_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G3P1`" ] pub struct G3P1_W < 'a > { w : & 'a mut W , } impl < 'a > G3P1_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 13 ) ) | ( ( ( value as u32 ) & 0x01 ) << 13 ) ; self . w } } # [ doc = "Reader of field `G3P0`" ] pub type G3P0_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G3P0`" ] pub struct G3P0_W < 'a > { w : & 'a mut W , } impl < 'a > G3P0_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 12 ) ) | ( ( ( value as u32 ) & 0x01 ) << 12 ) ; self . w } } # [ doc = "Reader of field `G2P3`" ] pub type G2P3_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G2P3`" ] pub struct G2P3_W < 'a > { w : & 'a mut W , } impl < 'a > G2P3_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 11 ) ) | ( ( ( value as u32 ) & 0x01 ) << 11 ) ; self . w } } # [ doc = "Reader of field `G2P2`" ] pub type G2P2_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G2P2`" ] pub struct G2P2_W < 'a > { w : & 'a mut W , } impl < 'a > G2P2_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 10 ) ) | ( ( ( value as u32 ) & 0x01 ) << 10 ) ; self . w } } # [ doc = "Reader of field `G2P1`" ] pub type G2P1_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G2P1`" ] pub struct G2P1_W < 'a > { w : & 'a mut W , } impl < 'a > G2P1_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 9 ) ) | ( ( ( value as u32 ) & 0x01 ) << 9 ) ; self . w } } # [ doc = "Reader of field `G2P0`" ] pub type G2P0_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G2P0`" ] pub struct G2P0_W < 'a > { w : & 'a mut W , } impl < 'a > G2P0_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 8 ) ) | ( ( ( value as u32 ) & 0x01 ) << 8 ) ; self . w } } # [ doc = "Reader of field `G1P3`" ] pub type G1P3_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G1P3`" ] pub struct G1P3_W < 'a > { w : & 'a mut W , } impl < 'a > G1P3_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 7 ) ) | ( ( ( value as u32 ) & 0x01 ) << 7 ) ; self . w } } # [ doc = "Reader of field `G1P2`" ] pub type G1P2_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G1P2`" ] pub struct G1P2_W < 'a > { w : & 'a mut W , } impl < 'a > G1P2_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 6 ) ) | ( ( ( value as u32 ) & 0x01 ) << 6 ) ; self . w } } # [ doc = "Reader of field `G1P1`" ] pub type G1P1_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G1P1`" ] pub struct G1P1_W < 'a > { w : & 'a mut W , } impl < 'a > G1P1_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 5 ) ) | ( ( ( value as u32 ) & 0x01 ) << 5 ) ; self . w } } # [ doc = "Reader of field `G1P0`" ] pub type G1P0_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G1P0`" ] pub struct G1P0_W < 'a > { w : & 'a mut W , } impl < 'a > G1P0_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 4 ) ) | ( ( ( value as u32 ) & 0x01 ) << 4 ) ; self . w } } # [ doc = "Reader of field `G0P3`" ] pub type G0P3_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G0P3`" ] pub struct G0P3_W < 'a > { w : & 'a mut W , } impl < 'a > G0P3_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 3 ) ) | ( ( ( value as u32 ) & 0x01 ) << 3 ) ; self . w } } # [ doc = "Reader of field `G0P2`" ] pub type G0P2_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G0P2`" ] pub struct G0P2_W < 'a > { w : & 'a mut W , } impl < 'a > G0P2_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 2 ) ) | ( ( ( value as u32 ) & 0x01 ) << 2 ) ; self . w } } # [ doc = "Reader of field `G0P1`" ] pub type G0P1_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G0P1`" ] pub struct G0P1_W < 'a > { w : & 'a mut W , } impl < 'a > G0P1_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 1 ) ) | ( ( ( value as u32 ) & 0x01 ) << 1 ) ; self . w } } # [ doc = "Reader of field `G0P0`" ] pub type G0P0_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `G0P0`" ] pub struct G0P0_W < 'a > { w : & 'a mut W , } impl < 'a > G0P0_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x01 ) | ( ( value as u32 ) & 0x01 ) ; self . w } } impl R { # [ doc = "Bit 23 - G5P3 sampling mode" ] # [ inline ( always ) ] pub fn g5p3 ( & self ) -> G5P3_R { G5P3_R :: new ( ( ( self . bits >> 23 ) & 0x01 ) != 0 ) } # [ doc = "Bit 22 - G5P2 sampling mode" ] # [ inline ( always ) ] pub fn g5p2 ( & self ) -> G5P2_R { G5P2_R :: new ( ( ( self . bits >> 22 ) & 0x01 ) != 0 ) } # [ doc = "Bit 21 - G5P1 sampling mode" ] # [ inline ( always ) ] pub fn g5p1 ( & self ) -> G5P1_R { G5P1_R :: new ( ( ( self . bits >> 21 ) & 0x01 ) != 0 ) } # [ doc = "Bit 20 - G5P0 sampling mode" ] # [ inline ( always ) ] pub fn g5p0 ( & self ) -> G5P0_R { G5P0_R :: new ( ( ( self . bits >> 20 ) & 0x01 ) != 0 ) } # [ doc = "Bit 19 - G4P3 sampling mode" ] # [ inline ( always ) ] pub fn g4p3 ( & self ) -> G4P3_R { G4P3_R :: new ( ( ( self . bits >> 19 ) & 0x01 ) != 0 ) } # [ doc = "Bit 18 - G4P2 sampling mode" ] # [ inline ( always ) ] pub fn g4p2 ( & self ) -> G4P2_R { G4P2_R :: new ( ( ( self . bits >> 18 ) & 0x01 ) != 0 ) } # [ doc = "Bit 17 - G4P1 sampling mode" ] # [ inline ( always ) ] pub fn g4p1 ( & self ) -> G4P1_R { G4P1_R :: new ( ( ( self . bits >> 17 ) & 0x01 ) != 0 ) } # [ doc = "Bit 16 - G4P0 sampling mode" ] # [ inline ( always ) ] pub fn g4p0 ( & self ) -> G4P0_R { G4P0_R :: new ( ( ( self . bits >> 16 ) & 0x01 ) != 0 ) } # [ doc = "Bit 15 - G3P3 sampling mode" ] # [ inline ( always ) ] pub fn g3p3 ( & self ) -> G3P3_R { G3P3_R :: new ( ( ( self . bits >> 15 ) & 0x01 ) != 0 ) } # [ doc = "Bit 14 - G3P2 sampling mode" ] # [ inline ( always ) ] pub fn g3p2 ( & self ) -> G3P2_R { G3P2_R :: new ( ( ( self . bits >> 14 ) & 0x01 ) != 0 ) } # [ doc = "Bit 13 - G3P1 sampling mode" ] # [ inline ( always ) ] pub fn g3p1 ( & self ) -> G3P1_R { G3P1_R :: new ( ( ( self . bits >> 13 ) & 0x01 ) != 0 ) } # [ doc = "Bit 12 - G3P0 sampling mode" ] # [ inline ( always ) ] pub fn g3p0 ( & self ) -> G3P0_R { G3P0_R :: new ( ( ( self . bits >> 12 ) & 0x01 ) != 0 ) } # [ doc = "Bit 11 - G2P3 sampling mode" ] # [ inline ( always ) ] pub fn g2p3 ( & self ) -> G2P3_R { G2P3_R :: new ( ( ( self . bits >> 11 ) & 0x01 ) != 0 ) } # [ doc = "Bit 10 - G2P2 sampling mode" ] # [ inline ( always ) ] pub fn g2p2 ( & self ) -> G2P2_R { G2P2_R :: new ( ( ( self . bits >> 10 ) & 0x01 ) != 0 ) } # [ doc = "Bit 9 - G2P1 sampling mode" ] # [ inline ( always ) ] pub fn g2p1 ( & self ) -> G2P1_R { G2P1_R :: new ( ( ( self . bits >> 9 ) & 0x01 ) != 0 ) } # [ doc = "Bit 8 - G2P0 sampling mode" ] # [ inline ( always ) ] pub fn g2p0 ( & self ) -> G2P0_R { G2P0_R :: new ( ( ( self . bits >> 8 ) & 0x01 ) != 0 ) } # [ doc = "Bit 7 - G1P3 sampling mode" ] # [ inline ( always ) ] pub fn g1p3 ( & self ) -> G1P3_R { G1P3_R :: new ( ( ( self . bits >> 7 ) & 0x01 ) != 0 ) } # [ doc = "Bit 6 - G1P2 sampling mode" ] # [ inline ( always ) ] pub fn g1p2 ( & self ) -> G1P2_R { G1P2_R :: new ( ( ( self . bits >> 6 ) & 0x01 ) != 0 ) } # [ doc = "Bit 5 - G1P1 sampling mode" ] # [ inline ( always ) ] pub fn g1p1 ( & self ) -> G1P1_R { G1P1_R :: new ( ( ( self . bits >> 5 ) & 0x01 ) != 0 ) } # [ doc = "Bit 4 - G1P0 sampling mode" ] # [ inline ( always ) ] pub fn g1p0 ( & self ) -> G1P0_R { G1P0_R :: new ( ( ( self . bits >> 4 ) & 0x01 ) != 0 ) } # [ doc = "Bit 3 - G0P3 sampling mode" ] # [ inline ( always ) ] pub fn g0p3 ( & self ) -> G0P3_R { G0P3_R :: new ( ( ( self . bits >> 3 ) & 0x01 ) != 0 ) } # [ doc = "Bit 2 - G0P2 sampling mode" ] # [ inline ( always ) ] pub fn g0p2 ( & self ) -> G0P2_R { G0P2_R :: new ( ( ( self . bits >> 2 ) & 0x01 ) != 0 ) } # [ doc = "Bit 1 - G0P1 sampling mode" ] # [ inline ( always ) ] pub fn g0p1 ( & self ) -> G0P1_R { G0P1_R :: new ( ( ( self . bits >> 1 ) & 0x01 ) != 0 ) } # [ doc = "Bit 0 - G0P0 sampling mode" ] # [ inline ( always ) ] pub fn g0p0 ( & self ) -> G0P0_R { G0P0_R :: new ( ( self . bits & 0x01 ) != 0 ) } } impl W { # [ doc = "Bit 23 - G5P3 sampling mode" ] # [ inline ( always ) ] pub fn g5p3 ( & mut self ) -> G5P3_W { G5P3_W { w : self } } # [ doc = "Bit 22 - G5P2 sampling mode" ] # [ inline ( always ) ] pub fn g5p2 ( & mut self ) -> G5P2_W { G5P2_W { w : self } } # [ doc = "Bit 21 - G5P1 sampling mode" ] # [ inline ( always ) ] pub fn g5p1 ( & mut self ) -> G5P1_W { G5P1_W { w : self } } # [ doc = "Bit 20 - G5P0 sampling mode" ] # [ inline ( always ) ] pub fn g5p0 ( & mut self ) -> G5P0_W { G5P0_W { w : self } } # [ doc = "Bit 19 - G4P3 sampling mode" ] # [ inline ( always ) ] pub fn g4p3 ( & mut self ) -> G4P3_W { G4P3_W { w : self } } # [ doc = "Bit 18 - G4P2 sampling mode" ] # [ inline ( always ) ] pub fn g4p2 ( & mut self ) -> G4P2_W { G4P2_W { w : self } } # [ doc = "Bit 17 - G4P1 sampling mode" ] # [ inline ( always ) ] pub fn g4p1 ( & mut self ) -> G4P1_W { G4P1_W { w : self } } # [ doc = "Bit 16 - G4P0 sampling mode" ] # [ inline ( always ) ] pub fn g4p0 ( & mut self ) -> G4P0_W { G4P0_W { w : self } } # [ doc = "Bit 15 - G3P3 sampling mode" ] # [ inline ( always ) ] pub fn g3p3 ( & mut self ) -> G3P3_W { G3P3_W { w : self } } # [ doc = "Bit 14 - G3P2 sampling mode" ] # [ inline ( always ) ] pub fn g3p2 ( & mut self ) -> G3P2_W { G3P2_W { w : self } } # [ doc = "Bit 13 - G3P1 sampling mode" ] # [ inline ( always ) ] pub fn g3p1 ( & mut self ) -> G3P1_W { G3P1_W { w : self } } # [ doc = "Bit 12 - G3P0 sampling mode" ] # [ inline ( always ) ] pub fn g3p0 ( & mut self ) -> G3P0_W { G3P0_W { w : self } } # [ doc = "Bit 11 - G2P3 sampling mode" ] # [ inline ( always ) ] pub fn g2p3 ( & mut self ) -> G2P3_W { G2P3_W { w : self } } # [ doc = "Bit 10 - G2P2 sampling mode" ] # [ inline ( always ) ] pub fn g2p2 ( & mut self ) -> G2P2_W { G2P2_W { w : self } } # [ doc = "Bit 9 - G2P1 sampling mode" ] # [ inline ( always ) ] pub fn g2p1 ( & mut self ) -> G2P1_W { G2P1_W { w : self } } # [ doc = "Bit 8 - G2P0 sampling mode" ] # [ inline ( always ) ] pub fn g2p0 ( & mut self ) -> G2P0_W { G2P0_W { w : self } } # [ doc = "Bit 7 - G1P3 sampling mode" ] # [ inline ( always ) ] pub fn g1p3 ( & mut self ) -> G1P3_W { G1P3_W { w : self } } # [ doc = "Bit 6 - G1P2 sampling mode" ] # [ inline ( always ) ] pub fn g1p2 ( & mut self ) -> G1P2_W { G1P2_W { w : self } } # [ doc = "Bit 5 - G1P1 sampling mode" ] # [ inline ( always ) ] pub fn g1p1 ( & mut self ) -> G1P1_W { G1P1_W { w : self } } # [ doc = "Bit 4 - G1P0 sampling mode" ] # [ inline ( always ) ] pub fn g1p0 ( & mut self ) -> G1P0_W { G1P0_W { w : self } } # [ doc = "Bit 3 - G0P3 sampling mode" ] # [ inline ( always ) ] pub fn g0p3 ( & mut self ) -> G0P3_W { G0P3_W { w : self } } # [ doc = "Bit 2 - G0P2 sampling mode" ] # [ inline ( always ) ] pub fn g0p2 ( & mut self ) -> G0P2_W { G0P2_W { w : self } } # [ doc = "Bit 1 - G0P1 sampling mode" ] # [ inline ( always ) ] pub fn g0p1 ( & mut self ) -> G0P1_W { G0P1_W { w : self } } # [ doc = "Bit 0 - G0P0 sampling mode" ] # [ inline ( always ) ] pub fn g0p0 ( & mut self ) -> G0P0_W { G0P0_W { w : self } } }