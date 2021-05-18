pub trait SwapNibbles {
    fn swap_nibbles(&self) -> Self;
}

impl SwapNibbles for u32 {
    fn swap_nibbles(&self) -> Self {
        ((self >> 4) & 0x0F0F0F0F) | ((self & 0x0F0F0F0F) << 4)
    }
}

impl SwapNibbles for u64 {
    fn swap_nibbles(&self) -> Self {
        ((self >> 4) & 0x0F0F0F0F0F0F0F0F) | ((self & 0x0F0F0F0F0F0F0F0F) << 4)
    }
}
