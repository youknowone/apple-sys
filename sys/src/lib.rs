#[cfg(not(feature = "prebuilt"))]
pub mod gen;
#[cfg(not(feature = "prebuilt"))]
pub use gen::*;

#[cfg(test)]
mod test;

#[cfg(all(feature = "prebuilt", target_os = "ios"))]
pub use apple_sys_ios::*;
#[cfg(all(feature = "prebuilt", target_os = "macos"))]
pub use apple_sys_macos::*;
