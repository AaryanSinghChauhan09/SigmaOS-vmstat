//! Vmstat model — report virtual memory statistics.

#[derive(Debug, Clone)]
pub struct MemStats {
    pub r: u64,
    pub b: u64,
    pub swap_in: u64,
    pub swap_out: u64,
    pub free: u64,
    pub buff: u64,
    pub cache: u64,
    pub si: u64,
    pub so: u64,
    pub bi: u64,
    pub bo: u64,
    pub in_: u64,
    pub cs: u64,
    pub us: u64,
    pub sy: u64,
    pub id: u64,
    pub wa: u64,
    pub st: u64,
}

#[derive(Debug, Clone)]
pub struct Vmstat {
    pub interval: u64,
    pub count: usize,
    pub disk_stats: bool,
}

impl Default for Vmstat {
    fn default() -> Self { Self { interval: 0, count: 1, disk_stats: false } }
}

impl Vmstat {
    pub fn new() -> Self { Self::default() }

    pub fn collect(&self) -> MemStats {
        MemStats { r: 0, b: 0, swap_in: 0, swap_out: 0, free: 0, buff: 0, cache: 0, si: 0, so: 0, bi: 0, bo: 0, in_: 0, cs: 0, us: 0, sy: 0, id: 0, wa: 0, st: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collect() {
        let v = Vmstat::new();
        assert_eq!(v.collect().free, 0);
    }
}
