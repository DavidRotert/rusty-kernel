/**
 * Architecture dependent code
 */

#[cfg(target_arch = "x86")]
pub mod i386;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86_common;