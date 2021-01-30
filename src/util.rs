use std::mem::size_of_val;

#[inline(always)]
pub fn read_u16_from_byteslice(slice: &[u8]) -> u16 {
    u16::from(slice[0]) | u16::from(slice[1]) << 8
}

#[inline(always)]
pub fn read_u32_from_byteslice(slice: &[u8]) -> u32 {
    u32::from(slice[0])
        | u32::from(slice[1]) << 8
        | u32::from(slice[2]) << 16
        | u32::from(slice[3]) << 24
}

#[inline(always)]
pub fn read_u32_from_byteslice_fast(slice: &[u8]) -> u32 {
    let result = unsafe {
         *std::mem::transmute::<*const u8, *const u32>(slice.as_ptr())
    };

    result
}


#[inline(always)]
pub fn write_u16_to_byteslice(slice: &mut [u8], value: u16) {
    slice[0] = value as u8;
    slice[1] = (value >> 8) as u8;
}

#[inline(always)]
pub fn write_u32_to_byteslice(slice: &mut [u8], value: u32) {
    slice[0] = value as u8;
    slice[1] = (value >> 8) as u8;
    slice[2] = (value >> 16) as u8;
    slice[3] = (value >> 24) as u8;
}

pub fn sign_extend(x: i32, nbits: u32) -> i32 {
    let notherbits = size_of_val(&x) as u32 * 8 - nbits;
    x.wrapping_shl(notherbits).wrapping_shr(notherbits)
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
    fn test_read_u32_from_byteslice_fast() {
        let buffer: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
        let result = read_u32_from_byteslice_fast(&buffer);
        assert_eq!(0xDDCCBBAA, result);
    }

    #[test]
    fn test_read_u32_from_byteslice_fast2() {
        let buffer: [u8; 8] = [0xAA, 0xBB, 0xCC, 0xDD, 0x00, 0x11, 0x22, 0x33];
        let result = read_u32_from_byteslice_fast(&buffer[4..8]);
        assert_eq!(0x33221100, result);
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

    #[test]
    fn test_sign_extend() {
        assert_eq!(sign_extend(0b100000000000, 12), -2048)
    }
}
