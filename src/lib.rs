//! SigmaOS-vmstat — Virtual memory statistics model.

pub mod util;
pub mod options;

pub use util::{Vmstat, MemStats};
pub use options::{VmstatOptions, Mode};
