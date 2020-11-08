# [ doc = "Reader of register CTL1" ] pub type R = crate :: R < u32 , super :: CTL1 > ; # [ doc = "Writer for register CTL1" ] pub type W = crate :: W < u32 , super :: CTL1 > ; # [ doc = "Register CTL1 `reset()`'s with value 0x80" ] impl crate :: ResetValue for super :: CTL1 { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0x80 } } # [ doc = "Reader of field `IRC14MEN`" ] pub type IRC14MEN_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `IRC14MEN`" ] pub struct IRC14MEN_W < 'a > { w : & 'a mut W , } impl < 'a > IRC14MEN_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x01 ) | ( ( value as u32 ) & 0x01 ) ; self . w } } # [ doc = "Reader of field `IRC14MSTB`" ] pub type IRC14MSTB_R = crate :: R < bool , bool > ; # [ doc = "Reader of field `IRC14MADJ`" ] pub type IRC14MADJ_R = crate :: R < u8 , u8 > ; # [ doc = "Write proxy for field `IRC14MADJ`" ] pub struct IRC14MADJ_W < 'a > { w : & 'a mut W , } impl < 'a > IRC14MADJ_W < 'a > { # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub unsafe fn bits ( self , value : u8 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x1f << 3 ) ) | ( ( ( value as u32 ) & 0x1f ) << 3 ) ; self . w } } # [ doc = "Reader of field `IRC14MCALIB`" ] pub type IRC14MCALIB_R = crate :: R < u8 , u8 > ; impl R { # [ doc = "Bit 0 - IRC14M Internal 14M RC oscillator Enable" ] # [ inline ( always ) ] pub fn irc14men ( & self ) -> IRC14MEN_R { IRC14MEN_R :: new ( ( self . bits & 0x01 ) != 0 ) } # [ doc = "Bit 1 - IRC14M Internal 14M RC Oscillator stabilization Flag" ] # [ inline ( always ) ] pub fn irc14mstb ( & self ) -> IRC14MSTB_R { IRC14MSTB_R :: new ( ( ( self . bits >> 1 ) & 0x01 ) != 0 ) } # [ doc = "Bits 3:7 - Internal 14M RC Oscillator clock trim adjust value" ] # [ inline ( always ) ] pub fn irc14madj ( & self ) -> IRC14MADJ_R { IRC14MADJ_R :: new ( ( ( self . bits >> 3 ) & 0x1f ) as u8 ) } # [ doc = "Bits 8:15 - Internal 14M RC Oscillator calibration value register" ] # [ inline ( always ) ] pub fn irc14mcalib ( & self ) -> IRC14MCALIB_R { IRC14MCALIB_R :: new ( ( ( self . bits >> 8 ) & 0xff ) as u8 ) } } impl W { # [ doc = "Bit 0 - IRC14M Internal 14M RC oscillator Enable" ] # [ inline ( always ) ] pub fn irc14men ( & mut self ) -> IRC14MEN_W { IRC14MEN_W { w : self } } # [ doc = "Bits 3:7 - Internal 14M RC Oscillator clock trim adjust value" ] # [ inline ( always ) ] pub fn irc14madj ( & mut self ) -> IRC14MADJ_W { IRC14MADJ_W { w : self } } }