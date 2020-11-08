# [ doc = "Reader of register CAR" ] pub type R = crate :: R < u16 , super :: CAR > ; # [ doc = "Writer for register CAR" ] pub type W = crate :: W < u16 , super :: CAR > ; # [ doc = "Register CAR `reset()`'s with value 0" ] impl crate :: ResetValue for super :: CAR { type Type = u16 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `CAR`" ] pub type CAR_R = crate :: R < u16 , u16 > ; # [ doc = "Write proxy for field `CAR`" ] pub struct CAR_W < 'a > { w : & 'a mut W , } impl < 'a > CAR_W < 'a > { # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub unsafe fn bits ( self , value : u16 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0xffff ) | ( ( value as u16 ) & 0xffff ) ; self . w } } impl R { # [ doc = "Bits 0:15 - Counter auto reload value" ] # [ inline ( always ) ] pub fn car ( & self ) -> CAR_R { CAR_R :: new ( ( self . bits & 0xffff ) as u16 ) } } impl W { # [ doc = "Bits 0:15 - Counter auto reload value" ] # [ inline ( always ) ] pub fn car ( & mut self ) -> CAR_W { CAR_W { w : self } } }