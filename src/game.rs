use std::{collections::BTreeMap, hash::Hash};

use bevy::utils::hashbrown::HashMap;
use indexmap::IndexMap;
use thiserror::Error;

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
pub struct BadukMove {
    pub player: Player,
    pub coordinates: (usize, usize),
}

#[derive(Error, Debug)]
enum MoveError {
    #[error("Illegal Move")]
    IllegalMove,
    #[error("Unexpected Missing Move in Game Tree")]
    MissingMove,
}

trait StatelessGame: Sized + Clone {
    type Move: Hash + Eq + Copy;
    fn list_all_legal_moves(&self) -> Vec<Self::Move>;
    fn is_legal(&self, game_move: &Self::Move) -> bool;
    fn generate_next_board(&self, game_move: &Self::Move) -> Result<Self, MoveError>;
}

pub struct BadukClassical<const SIZE: usize> {
    pub turn: Player,
    pub board: Board<SIZE>,
    pub captures: (u32, u32), // (black, white)
    pub ko_point: Option<(usize, usize)>,
}

pub enum SupportedGames {
    BadukClassic(BadukClassical<19>),
    BadukBeginner(BadukClassical<13>),
    BadukNewbie(BadukClassical<9>),
}

#[derive(Clone)]
pub struct GameNode<G: StatelessGame> {
    pub game: G,
    // Maybe its a good idea to swap this out for a hashmap, but I think since most of these are
    // going to only contain 5-10 elements it might be faster as a btree, and it also ensures that
    // they can be displayed and saved in a consistent order.
    //
    // Also the game G gets saved twice, once as the key in the btree and another as the
    // GameNode.game.
    pub children: IndexMap<G::Move, GameNode<G>>,
}
impl<G: StatelessGame> GameNode<G> {
    fn new(game: G) -> Self {
        GameNode {
            game,
            children: IndexMap::new(),
        }
    }
    fn traverse_downward(&mut self, mv: &G::Move) -> Result<&mut Self, MoveError> {
        self.children.get_mut(mv).ok_or(MoveError::MissingMove)
    }

    fn make_move(&mut self, mv: G::Move) -> Result<&mut Self, MoveError> {
        if !self.game.is_legal(&mv) {
            return Err(MoveError::IllegalMove);
        }
        let generate_board = || {
            let current_board = &self.game;
            let next_board = current_board.generate_next_board(&mv).unwrap();
            GameNode::new(next_board)
        };
        let next_node_ref = self.children.entry(mv).or_insert_with(generate_board);
        Ok(next_node_ref)
    }
}

pub struct GamePointer<G: StatelessGame> {
    move_list: Vec<G::Move>,
}
impl<G: StatelessGame> GamePointer<G> {
    fn traverse_tree_downward<'a>(
        &self,
        game: &'a mut GameTree<G>,
    ) -> Result<&'a mut GameNode<G>, MoveError> {
        let mut node = &mut game.root;
        for mv in &self.move_list {
            let next = node.traverse_downward(mv)?;
            node = next;
        }
        Ok(node)
    }
    fn make_move<'a>(
        &mut self,
        game: &'a mut GameTree<G>,
        mv: G::Move,
    ) -> Result<&'a mut GameNode<G>, MoveError> {
        let active_board = self.traverse_tree_downward(game)?;
        let next_board = active_board.make_move(mv)?;
        self.move_list.push(mv);
        Ok(next_board)
    }
    fn undo_move(&mut self) {
        let _ = self.move_list.pop();
    }
}

pub struct GameTree<Game: StatelessGame> {
    root: GameNode<Game>,
}
