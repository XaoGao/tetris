use rand::prelude::{ SliceRandom, ThreadRng };

use self::piece::{ Piece, Kind as PieceKind };

mod piece;

pub struct Engine {
    board: Board,
    bag: Vec<PieceKind>,
    rng: ThreadRng
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: Board::blank(),
            bag: Vec::new(),
            rng: ThreadRng::default()
        }
    }

    fn refill_bag(&mut self) {
        // fill all pices in bag
        self.bag.shuffle(&mut self.rng)
    }
}

struct Board([bool; Self::SIZE]);

impl Board {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    fn blank() -> Self {
        Self([false; Self::SIZE])
    }
}