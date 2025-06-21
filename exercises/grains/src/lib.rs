pub fn square(s: u32) -> u64 {
    // todo!("grains of rice on square {s}");
    assert!(s > 0 && s <= 64, "Square must be between 1 and 64");
    1u64 << (s -1)
}

pub fn total() -> u64 {
    // todo!();
    u64::MAX
}
