
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::{Registers, swap_registers};

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

