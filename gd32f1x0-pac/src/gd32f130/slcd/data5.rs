# [ doc = "Reader of register DATA5" ] pub type R = crate :: R < u32 , super :: DATA5 > ; # [ doc = "Writer for register DATA5" ] pub type W = crate :: W < u32 , super :: DATA5 > ; # [ doc = "Register DATA5 `reset()`'s with value 0" ] impl crate :: ResetValue for super :: DATA5 { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `SEG_DATA5`" ] pub type SEG_DATA5_R = crate :: R < u32 , u32 > ; # [ doc = "Write proxy for field `SEG_DATA5`" ] pub struct SEG_DATA5_W < 'a > { w : & 'a mut W , } impl < 'a > SEG_DATA5_W < 'a > { # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub unsafe fn bits ( self , value : u32 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0xffff_ffff ) | ( ( value as u32 ) & 0xffff_ffff ) ; self . w } } impl R { # [ doc = "Bits 0:31 - Each bit corresponds to one segment to display" ] # [ inline ( always ) ] pub fn seg_data5 ( & self ) -> SEG_DATA5_R { SEG_DATA5_R :: new ( ( self . bits & 0xffff_ffff ) as u32 ) } } impl W { # [ doc = "Bits 0:31 - Each bit corresponds to one segment to display" ] # [ inline ( always ) ] pub fn seg_data5 ( & mut self ) -> SEG_DATA5_W { SEG_DATA5_W { w : self } } }