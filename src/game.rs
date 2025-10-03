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
// Players and equipment
//
//     Rule 1.[7] Players: Go is a game between two players, called Black and White.
//     Rule 2.[8] Board: Go is played on a plain grid of 19 horizontal and 19 vertical lines, called a board.
//         Definition. ("Intersection") A point on the board where a horizontal line meets a vertical line is called an intersection.
//     Rule 3.[9][10] Stones: Go is played with playing tokens known as stones. Each player has at their disposal an adequate supply (usually 180) of stones of the same color.
//
// Positions
//
//     Rule 4.[11][12] Positions: At any time in the game, each intersection on the board is in one and only one of the following three states: 1) empty; 2) occupied by a black stone; or 3) occupied by a white stone. A position consists of an indication of the state of each intersection.
//         Definition. ("Adjacent") Two intersections are said to be adjacent if they are connected by a horizontal or vertical line with no other intersections between them.[13] Note that intersections which are one away from each other diagonally, i.e., intersections that are connected by one horizontal and one vertical line, are not considered adjacent.
//         Definition.[14] ("Connected") In a given position, two placed stones of the same color (or two empty intersections) are said to be connected if it is possible to draw a path from one intersection to the other by passing through only adjacent intersections of the same state (empty, occupied by white, or occupied by black).
//         Definition. ("Liberty") In a given position, a liberty of a placed stone is an empty intersection adjacent to that stone or adjacent to a stone which is connected to that stone.[13]
//
// Play
//
//     Rule 5.[15] Initial position: At the beginning of the game, the board is empty.
//     Rule 6.[16] Turns: Black moves first. The players alternate thereafter.
//     Rule 7.[13] Moving: When it is their turn, a player may either pass (by announcing "pass" and performing no action) or play. A play consists of the following steps (performed in the prescribed order):
//         Step 1. (Playing a stone) Placing a stone of their color on an empty intersection (chosen subject to Rule 8 and, if it is in effect, to Optional Rule 7A). It can never be moved to another intersection after being played.
//         Step 2. (Capture) Removing from the board any stones of their opponent's color that have no liberties.
//         Step 3. (Self-capture) Removing from the board any stones of their own color that have no liberties.
//     Optional Rule 7A.[17] Prohibition of suicide: A play is illegal if one or more stones of that player's color would be removed in Step 3 of that play.
//     Rule 8.[18] Prohibition of repetition: A play is illegal if it would have the effect (after all steps of the play have been completed) of creating a position that has occurred previously in the game.
//
// End
//
//     Rule 9.[19] End: The game ends when both players have passed consecutively. The final position is the position on the board at the time the players pass consecutively.
//         Definition.[20][21] ("Territory") In the final position, an empty intersection is said to belong to a player's territory if all stones adjacent to it or to an empty intersection connected to it are of that player's color.
//         Definition.[22] ("Area") In the final position, an intersection is said to belong to a player's area if either: 1) it belongs to that player's territory; or 2) it is occupied by a stone of that player's color.
//         Definition.[23] ("Score") A player's score is the number of intersections in their area in the final position.
//     Rule 10.[24] Winner: If one player has a higher score than the other, then that player wins. Otherwise, the game is a draw.
impl<const SIZE: usize> GameNode<SIZE> {
    pub fn legal_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let player = self.turn;

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
