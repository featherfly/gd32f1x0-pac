#[doc = "Reader of register OBSTAT"]
pub type R = crate::R<u32, super::OBSTAT>;
#[doc = "Reader of field `OBERR`"]
pub type OBERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLEVEL`"]
pub type PLEVEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `USER`"]
pub type USER_R = crate::R<u8, u8>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 0 - Option bytes read error bit"]
    #[inline(always)]
    pub fn oberr(&self) -> OBERR_R {
        OBERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Option bytes security protection level"]
    #[inline(always)]
    pub fn plevel(&self) -> PLEVEL_R {
        PLEVEL_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - Store USER of option bytes block after system reset"]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Store DATA\\[15:0\\]
of option bytes block after system reset"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
