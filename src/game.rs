#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Point {
    Empty,
    Stone(Player),
}

#[derive(Clone, Debug)]
pub struct Board<const SIZE: usize> {
    points: [[Point; SIZE]; SIZE],
}

impl<const SIZE: usize> Board<SIZE> {
    pub fn new() -> Self {
        Self {
            points: [[Point::Empty; SIZE]; SIZE],
        }
    }

    pub fn get_point(&self, r: usize, c: usize) -> Option<Point> {
        if r < SIZE && c < SIZE {
            Some(self.points[r][c])
        } else {
            None
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

impl<const SIZE: usize> GameNode<SIZE> {
    pub fn legal_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let player = self.next_player();

        for r in 0..SIZE {
            for c in 0..SIZE {
                if self.board.get_point(r, c) == Some(Point::Empty) {
                    if self.is_legal((r, c), player) {
                        moves.push((r, c));
                    }
                }
            }
        }
        moves
    }

    fn is_legal(&self, coords: (usize, usize), player: Player) -> bool {
        if self.ko_point == Some(coords) {
            return false;
        }

        // A full suicide check is needed here.
        // This involves:
        // 1. Simulating the move.
        // 2. Checking the liberties of the new stone's group.
        // 3. If the group has no liberties, checking if the move captures any opponent stones.
        // A move is suicide if the group has 0 liberties AND it captures no opponent stones.
        
        // For now, we'll just prevent placing a stone on another stone.
        self.board.get_point(coords.0, coords.1) == Some(Point::Empty)
    }

    fn next_player(&self) -> Player {
        if let Some(move_info) = &self.move_info {
            move_info.player.opponent()
        } else {
            Player::Black // Black moves first
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub player: Player,
    pub coordinates: (usize, usize),
}

pub type GameTree<const SIZE: usize> = GameNode<SIZE>;
