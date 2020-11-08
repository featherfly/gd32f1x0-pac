#[doc = "Reader of register SSTS"]
pub type R = crate::R<u32, super::SSTS>;
#[doc = "Reader of field `SSC`"]
pub type SSC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value"]
    #[inline(always)]
    pub fn ssc(&self) -> SSC_R {
        SSC_R::new((self.bits & 0xffff) as u16)
    }
}
