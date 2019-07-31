pub fn read_u16_from_byteslice(slice: &[u8]) -> u16 {
    ((slice[0]) as u16) << 0 | ((slice[1]) as u16) << 8
}

pub fn read_u32_from_byteslice(slice: &[u8]) -> u32 {
    ((slice[0]) as u32) << 0 |
        ((slice[1]) as u32) << 8 |
        ((slice[2]) as u32) << 16 |
        ((slice[3]) as u32) << 24
}

pub fn write_u16_to_byteslice(slice: &mut [u8], value: u16) {
    slice[0] = value as u8;
    slice[1] = (value >> 8) as u8;
}

pub fn write_u32_to_byteslice(slice: &mut [u8], value: u32) {
    slice[0] = value as u8;
    slice[1] = (value >> 8) as u8;
    slice[2] = (value >> 16) as u8;
    slice[3] = (value >> 24) as u8;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_u16_from_byteslice() {
        let buffer: [u8; 2] = [0xAA, 0xBB];
        let result = read_u16_from_byteslice(&buffer);
        assert_eq!(0xBBAA, result);
    }

    #[test]
    fn test_read_u32_from_byteslice() {
        let buffer: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
        let result = read_u32_from_byteslice(&buffer);
        assert_eq!(0xDDCCBBAA, result);
    }

    #[test]
    fn test_write_u16_to_byteslice() {
        let mut buffer = [0u8; 2];
        write_u16_to_byteslice(&mut buffer, 0xBBAA);
        assert_eq!([0xAA, 0xBB], buffer);
    }

    #[test]
    fn test_write_u32_to_byteslice() {
        let mut buffer = [0u8; 4];
        write_u32_to_byteslice(&mut buffer, 0xDDCCBBAA);
        assert_eq!([0xAA, 0xBB, 0xCC, 0xDD], buffer);
    }
}