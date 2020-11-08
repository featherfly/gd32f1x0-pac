#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `REA`"]
pub type REA_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEA`"]
pub type TEA_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUF`"]
pub type WUF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RWU`"]
pub type RWU_R = crate::R<bool, bool>;
#[doc = "Reader of field `SBF`"]
pub type SBF_R = crate::R<bool, bool>;
#[doc = "Reader of field `AMF`"]
pub type AMF_R = crate::R<bool, bool>;
#[doc = "Reader of field `BSY`"]
pub type BSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `ABDF`"]
pub type ABDF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ABDE`"]
pub type ABDE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EBF`"]
pub type EBF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTF`"]
pub type RTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTS`"]
pub type CTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTSF`"]
pub type CTSF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LBDF`"]
pub type LBDF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TBE`"]
pub type TBE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TC`"]
pub type TC_R = crate::R<bool, bool>;
#[doc = "Reader of field `RBNE`"]
pub type RBNE_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDLEF`"]
pub type IDLEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ORERR`"]
pub type ORERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `NERR`"]
pub type NERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `FERR`"]
pub type FERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PERR`"]
pub type PERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 22 - Receive enable acknowledge flag"]
    #[inline(always)]
    pub fn rea(&self) -> REA_R {
        REA_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transmit enable acknowledge flag"]
    #[inline(always)]
    pub fn tea(&self) -> TEA_R {
        TEA_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Wakeup from Stop mode flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receiver wakeup from Mute mode"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Send break flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - character match flag"]
    #[inline(always)]
    pub fn amf(&self) -> AMF_R {
        AMF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Busy flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Auto baud rate flag"]
    #[inline(always)]
    pub fn abdf(&self) -> ABDF_R {
        ABDF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Auto baud rate error"]
    #[inline(always)]
    pub fn abde(&self) -> ABDE_R {
        ABDE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12 - End of block flag"]
    #[inline(always)]
    pub fn ebf(&self) -> EBF_R {
        EBF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Receiver timeout"]
    #[inline(always)]
    pub fn rtf(&self) -> RTF_R {
        RTF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CTS flag"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CTS interrupt flag"]
    #[inline(always)]
    pub fn ctsf(&self) -> CTSF_R {
        CTSF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit data register empty"]
    #[inline(always)]
    pub fn tbe(&self) -> TBE_R {
        TBE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmission complete"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Read data register not empty"]
    #[inline(always)]
    pub fn rbne(&self) -> RBNE_R {
        RBNE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Idle line detected"]
    #[inline(always)]
    pub fn idlef(&self) -> IDLEF_R {
        IDLEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun error"]
    #[inline(always)]
    pub fn orerr(&self) -> ORERR_R {
        ORERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Noise detected flag"]
    #[inline(always)]
    pub fn nerr(&self) -> NERR_R {
        NERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Framing error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Parity error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new((self.bits & 0x01) != 0)
    }
}
