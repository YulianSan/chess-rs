pub struct Move(u16);

pub const MASK_6_BITS: u16 = 0b00111111;

impl Move {
    pub fn new(from: u8, to: u8) -> Self {
        let from = ((from as u16) & MASK_6_BITS) << 6;
        let to = (to as u16) & MASK_6_BITS;

        Move(from | to)
    }

    pub fn from(&self) -> u8 {
        ((self.0 >> 6) & MASK_6_BITS) as u8
    }

    pub fn to(&self) -> u8 {
        (self.0 & MASK_6_BITS) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let mv = Move::new(10, 20);

        assert_eq!(mv.from(), 10);
        assert_eq!(mv.to(), 20);
        assert_eq!(mv.0, 0b1010_010100); // from = 10 (0b00001010), to = 20 (0b00010100)
    }

    #[test]
    fn test_move_overflow() {
        let mv = Move::new(64, 128);

        assert_eq!(mv.from(), 0); // 01000000 & 0b00111111 = 0
        assert_eq!(mv.to(), 0); // 10000000 & 0b00111111 = 0
    }
}
