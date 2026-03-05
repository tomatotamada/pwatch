use crate::port::PortInfo;

pub trait PortScanner {
    fn scan() -> Vec<PortInfo>;
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::LinuxScanner as PlatformScanner;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::MacosScanner as PlatformScanner;
