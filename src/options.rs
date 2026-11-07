//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Default,
    Disk,
    Wide,
    Timestamp,
}

#[derive(Debug, Clone)]
pub struct VmstatOptions {
    pub mode: Mode,
    pub interval: u64,
    pub count: usize,
}

impl Default for VmstatOptions {
    fn default() -> Self { Self { mode: Mode::Default, interval: 0, count: 1 } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opts() { assert_eq!(VmstatOptions::default().mode, Mode::Default); }
}
