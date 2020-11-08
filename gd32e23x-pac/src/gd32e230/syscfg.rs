#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System configuration register 0"]
    pub cfg0: CFG0,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - EXTI sources selection register 0"]
    pub extiss0: EXTISS0,
    #[doc = "0x0c - EXTI sources selection register 1"]
    pub extiss1: EXTISS1,
    #[doc = "0x10 - EXTI sources selection register 2"]
    pub extiss2: EXTISS2,
    #[doc = "0x14 - EXTI sources selection register 3"]
    pub extiss3: EXTISS3,
    #[doc = "0x18 - System configuration register 2"]
    pub cfg2: CFG2,
    _reserved6: [u8; 228usize],
    #[doc = "0x100 - IRQ Latency register"]
    pub cpu_irq_lat: CPU_IRQ_LAT,
}
#[doc = "System configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg0](cfg0) module"]
pub type CFG0 = crate::Reg<u32, _CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG0;
#[doc = "`read()` method returns [cfg0::R](cfg0::R) reader structure"]
impl crate::Readable for CFG0 {}
#[doc = "`write(|w| ..)` method takes [cfg0::W](cfg0::W) writer structure"]
impl crate::Writable for CFG0 {}
#[doc = "System configuration register 0"]
pub mod cfg0;
#[doc = "EXTI sources selection register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extiss0](extiss0) module"]
pub type EXTISS0 = crate::Reg<u32, _EXTISS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTISS0;
#[doc = "`read()` method returns [extiss0::R](extiss0::R) reader structure"]
impl crate::Readable for EXTISS0 {}
#[doc = "`write(|w| ..)` method takes [extiss0::W](extiss0::W) writer structure"]
impl crate::Writable for EXTISS0 {}
#[doc = "EXTI sources selection register 0"]
pub mod extiss0;
#[doc = "EXTI sources selection register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extiss1](extiss1) module"]
pub type EXTISS1 = crate::Reg<u32, _EXTISS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTISS1;
#[doc = "`read()` method returns [extiss1::R](extiss1::R) reader structure"]
impl crate::Readable for EXTISS1 {}
#[doc = "`write(|w| ..)` method takes [extiss1::W](extiss1::W) writer structure"]
impl crate::Writable for EXTISS1 {}
#[doc = "EXTI sources selection register 1"]
pub mod extiss1;
#[doc = "EXTI sources selection register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extiss2](extiss2) module"]
pub type EXTISS2 = crate::Reg<u32, _EXTISS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTISS2;
#[doc = "`read()` method returns [extiss2::R](extiss2::R) reader structure"]
impl crate::Readable for EXTISS2 {}
#[doc = "`write(|w| ..)` method takes [extiss2::W](extiss2::W) writer structure"]
impl crate::Writable for EXTISS2 {}
#[doc = "EXTI sources selection register 2"]
pub mod extiss2;
#[doc = "EXTI sources selection register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extiss3](extiss3) module"]
pub type EXTISS3 = crate::Reg<u32, _EXTISS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTISS3;
#[doc = "`read()` method returns [extiss3::R](extiss3::R) reader structure"]
impl crate::Readable for EXTISS3 {}
#[doc = "`write(|w| ..)` method takes [extiss3::W](extiss3::W) writer structure"]
impl crate::Writable for EXTISS3 {}
#[doc = "EXTI sources selection register 3"]
pub mod extiss3;
#[doc = "System configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](cfg2) module"]
pub type CFG2 = crate::Reg<u32, _CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG2;
#[doc = "`read()` method returns [cfg2::R](cfg2::R) reader structure"]
impl crate::Readable for CFG2 {}
#[doc = "`write(|w| ..)` method takes [cfg2::W](cfg2::W) writer structure"]
impl crate::Writable for CFG2 {}
#[doc = "System configuration register 2"]
pub mod cfg2;
#[doc = "IRQ Latency register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_irq_lat](cpu_irq_lat) module"]
pub type CPU_IRQ_LAT = crate::Reg<u32, _CPU_IRQ_LAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU_IRQ_LAT;
#[doc = "`read()` method returns [cpu_irq_lat::R](cpu_irq_lat::R) reader structure"]
impl crate::Readable for CPU_IRQ_LAT {}
#[doc = "`write(|w| ..)` method takes [cpu_irq_lat::W](cpu_irq_lat::W) writer structure"]
impl crate::Writable for CPU_IRQ_LAT {}
#[doc = "IRQ Latency register"]
pub mod cpu_irq_lat;
