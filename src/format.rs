use num_enum::TryFromPrimitive;

macro_rules! fourcc_code {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        (($a as u32) | (($b as u32) << 8) | (($c as u32) << 16) | (($d as u32) << 24)) as u32
    };
}

/// Representation of a pixel formats
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum Format {
    /// \[23:0\] R:G:B 8:8:8 little endian
    RGB888 = fourcc_code!('R', 'G', '2', '4'),

    /// \[31:0\] x:R:G:B 8:8:8:8 little endian
    XRGB8888 = fourcc_code!('X', 'R', '2', '4'),
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_format_enum() {
        assert_eq!(super::Format::RGB888 as u32, 0x34324752);
    }
}
