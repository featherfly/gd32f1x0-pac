#[doc = "Reader of register IDATA3"]
pub type R = crate::R<u32, super::IDATA3>;
#[doc = "Reader of field `IDATAn`"]
pub type IDATAN_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Injected data"]
    #[inline(always)]
    pub fn idatan(&self) -> IDATAN_R {
        IDATAN_R::new((self.bits & 0xffff) as u16)
    }
}
