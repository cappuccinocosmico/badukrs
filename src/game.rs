#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    Black,
    White,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Point {
    Empty,
    Stone(Player),
}

pub struct Board<const SIZE: usize> {
    points: [[Point; SIZE]; SIZE],
}

impl<const SIZE: usize> Board<SIZE> {
    pub fn new() -> Self {
        Self {
            points: [[Point::Empty; SIZE]; SIZE],
        }
    }
}

pub struct GameNode<const SIZE: usize> {
    pub board: Board<SIZE>,
    pub captures: (u32, u32), // (black, white)
    pub ko_point: Option<(usize, usize)>,
    pub move_info: Option<Move>, // None for the root node
    pub children: Vec<GameNode<SIZE>>,
}

pub struct Move {
    pub player: Player,
    pub coordinates: (usize, usize),
}

pub type GameTree<const SIZE: usize> = GameNode<SIZE>;
