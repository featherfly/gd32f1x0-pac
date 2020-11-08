#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x04 - Control register 1"]
    pub ctl1: CTL1,
    #[doc = "0x08 - Control register 2"]
    pub ctl2: CTL2,
    #[doc = "0x0c - Baud rate register"]
    pub baud: BAUD,
    #[doc = "0x10 - Guard time and prescaler register"]
    pub gp: GP,
    #[doc = "0x14 - Receiver timeout register"]
    pub rt: RT,
    #[doc = "0x18 - Request register"]
    pub cmd: CMD,
    #[doc = "0x1c - Interrupt & status register"]
    pub stat: STAT,
    #[doc = "0x20 - Interrupt flag clear register"]
    pub intc: INTC,
    #[doc = "0x24 - Receive data register"]
    pub rdata: RDATA,
    #[doc = "0x28 - Transmit data register"]
    pub tdata: TDATA,
    _reserved11: [u8; 148usize],
    #[doc = "0xc0 - coherence control register"]
    pub chc: CHC,
    _reserved12: [u8; 12usize],
    #[doc = "0xd0 - USART receive FIFO control and status register"]
    pub rfcs: RFCS,
}
#[doc = "Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](ctl0) module"]
pub type CTL0 = crate::Reg<u32, _CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL0;
#[doc = "`read()` method returns [ctl0::R](ctl0::R) reader structure"]
impl crate::Readable for CTL0 {}
#[doc = "`write(|w| ..)` method takes [ctl0::W](ctl0::W) writer structure"]
impl crate::Writable for CTL0 {}
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](ctl1) module"]
pub type CTL1 = crate::Reg<u32, _CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL1;
#[doc = "`read()` method returns [ctl1::R](ctl1::R) reader structure"]
impl crate::Readable for CTL1 {}
#[doc = "`write(|w| ..)` method takes [ctl1::W](ctl1::W) writer structure"]
impl crate::Writable for CTL1 {}
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl2](ctl2) module"]
pub type CTL2 = crate::Reg<u32, _CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL2;
#[doc = "`read()` method returns [ctl2::R](ctl2::R) reader structure"]
impl crate::Readable for CTL2 {}
#[doc = "`write(|w| ..)` method takes [ctl2::W](ctl2::W) writer structure"]
impl crate::Writable for CTL2 {}
#[doc = "Control register 2"]
pub mod ctl2;
#[doc = "Baud rate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud](baud) module"]
pub type BAUD = crate::Reg<u32, _BAUD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAUD;
#[doc = "`read()` method returns [baud::R](baud::R) reader structure"]
impl crate::Readable for BAUD {}
#[doc = "`write(|w| ..)` method takes [baud::W](baud::W) writer structure"]
impl crate::Writable for BAUD {}
#[doc = "Baud rate register"]
pub mod baud;
#[doc = "Guard time and prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp](gp) module"]
pub type GP = crate::Reg<u32, _GP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GP;
#[doc = "`read()` method returns [gp::R](gp::R) reader structure"]
impl crate::Readable for GP {}
#[doc = "`write(|w| ..)` method takes [gp::W](gp::W) writer structure"]
impl crate::Writable for GP {}
#[doc = "Guard time and prescaler register"]
pub mod gp;
#[doc = "Receiver timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rt](rt) module"]
pub type RT = crate::Reg<u32, _RT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RT;
#[doc = "`read()` method returns [rt::R](rt::R) reader structure"]
impl crate::Readable for RT {}
#[doc = "`write(|w| ..)` method takes [rt::W](rt::W) writer structure"]
impl crate::Writable for RT {}
#[doc = "Receiver timeout register"]
pub mod rt;
#[doc = "Request register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Request register"]
pub mod cmd;
#[doc = "Interrupt & status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "Interrupt & status register"]
pub mod stat;
#[doc = "Interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intc](intc) module"]
pub type INTC = crate::Reg<u32, _INTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTC;
#[doc = "`write(|w| ..)` method takes [intc::W](intc::W) writer structure"]
impl crate::Writable for INTC {}
#[doc = "Interrupt flag clear register"]
pub mod intc;
#[doc = "Receive data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdata](rdata) module"]
pub type RDATA = crate::Reg<u32, _RDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDATA;
#[doc = "`read()` method returns [rdata::R](rdata::R) reader structure"]
impl crate::Readable for RDATA {}
#[doc = "Receive data register"]
pub mod rdata;
#[doc = "Transmit data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdata](tdata) module"]
pub type TDATA = crate::Reg<u32, _TDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDATA;
#[doc = "`read()` method returns [tdata::R](tdata::R) reader structure"]
impl crate::Readable for TDATA {}
#[doc = "`write(|w| ..)` method takes [tdata::W](tdata::W) writer structure"]
impl crate::Writable for TDATA {}
#[doc = "Transmit data register"]
pub mod tdata;
#[doc = "coherence control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chc](chc) module"]
pub type CHC = crate::Reg<u32, _CHC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHC;
#[doc = "`read()` method returns [chc::R](chc::R) reader structure"]
impl crate::Readable for CHC {}
#[doc = "`write(|w| ..)` method takes [chc::W](chc::W) writer structure"]
impl crate::Writable for CHC {}
#[doc = "coherence control register"]
pub mod chc;
#[doc = "USART receive FIFO control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcs](rfcs) module"]
pub type RFCS = crate::Reg<u32, _RFCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCS;
#[doc = "`read()` method returns [rfcs::R](rfcs::R) reader structure"]
impl crate::Readable for RFCS {}
#[doc = "`write(|w| ..)` method takes [rfcs::W](rfcs::W) writer structure"]
impl crate::Writable for RFCS {}
#[doc = "USART receive FIFO control and status register"]
pub mod rfcs;
