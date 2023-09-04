#[cfg(all(target_arch = "arm", target_os = "none"))]
pub mod rp2040;
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use rp2040 as uart;

#[cfg(all(target_arch = "arm", target_os = "linux"))]
pub mod rpi4;
#[cfg(all(target_arch = "arm", target_os = "linux"))]
pub use rpi4 as uart;

