#[derive(Debug)]
pub struct ChessPosition(u8, u8);

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self(rank as u8, file as u8)),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.pos.0 == other.pos.0
            || self.pos.1 == other.pos.1
            || ((self.pos.0 as i8 - other.pos.0 as i8).abs()
                == (self.pos.1 as i8 - other.pos.1 as i8).abs())
    }
}
