use indexmap::IndexMap;
use std::hash::Hash;
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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

    pub fn is_valid_coordinate(&self, r: usize, c: usize) -> bool {
        r < SIZE && c < SIZE
    }

    pub fn place_stone(&mut self, r: usize, c: usize, player: Player) -> bool {
        if self.is_valid_coordinate(r, c) && self.points[r][c] == Point::Empty {
            self.points[r][c] = Point::Stone(player);
            true
        } else {
            false
        }
    }

    pub fn remove_stone(&mut self, r: usize, c: usize) {
        if self.is_valid_coordinate(r, c) {
            self.points[r][c] = Point::Empty;
        }
    }

    pub fn get_adjacent_points(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut adjacent = Vec::new();

        if r > 0 {
            adjacent.push((r - 1, c));
        }
        if r + 1 < SIZE {
            adjacent.push((r + 1, c));
        }
        if c > 0 {
            adjacent.push((r, c - 1));
        }
        if c + 1 < SIZE {
            adjacent.push((r, c + 1));
        }

        adjacent
    }

    pub fn get_group(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut group = Vec::new();
        let mut visited = std::collections::HashSet::new();

        if let Some(point) = self.get_point(r, c) {
            if point != Point::Empty {
                self.flood_fill_group(r, c, point, &mut group, &mut visited);
            }
        }

        group
    }

    fn flood_fill_group(
        &self,
        r: usize,
        c: usize,
        target_point: Point,
        group: &mut Vec<(usize, usize)>,
        visited: &mut std::collections::HashSet<(usize, usize)>,
    ) {
        if visited.contains(&(r, c)) {
            return;
        }

        if let Some(current_point) = self.get_point(r, c) {
            if current_point == target_point {
                visited.insert((r, c));
                group.push((r, c));

                for (adj_r, adj_c) in self.get_adjacent_points(r, c) {
                    self.flood_fill_group(adj_r, adj_c, target_point, group, visited);
                }
            }
        }
    }

    pub fn count_liberties(&self, group: &[(usize, usize)]) -> usize {
        let mut liberties = std::collections::HashSet::new();

        for &(r, c) in group {
            for (adj_r, adj_c) in self.get_adjacent_points(r, c) {
                if self.get_point(adj_r, adj_c) == Some(Point::Empty) {
                    liberties.insert((adj_r, adj_c));
                }
            }
        }

        liberties.len()
    }
}

#[derive(Clone, Debug)]
pub struct BadukClassical<const SIZE: usize> {
    pub turn: Player,
    pub board: Board<SIZE>,
    pub captures: (u32, u32), // (black, white)
    pub ko_point: Option<(usize, usize)>,
    pub consecutive_passes: u8,
    pub position_history: Vec<[[Point; SIZE]; SIZE]>,
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
impl<const SIZE: usize> BadukClassical<SIZE> {
    pub fn new() -> Self {
        Self {
            turn: Player::Black,
            board: Board::new(),
            captures: (0, 0),
            ko_point: None,
            consecutive_passes: 0,
            position_history: vec![[[Point::Empty; SIZE]; SIZE]],
        }
    }

    pub fn remove_captured_stones(&mut self, opponent: Player) -> u32 {
        let mut captured_count = 0;
        let mut stones_to_remove = Vec::new();

        for r in 0..SIZE {
            for c in 0..SIZE {
                if self.board.get_point(r, c) == Some(Point::Stone(opponent)) {
                    let group = self.board.get_group(r, c);
                    if !group.is_empty() && self.board.count_liberties(&group) == 0 {
                        stones_to_remove.extend(group);
                    }
                }
            }
        }

        // Remove duplicates
        stones_to_remove.sort_unstable();
        stones_to_remove.dedup();

        for (r, c) in stones_to_remove {
            self.board.remove_stone(r, c);
            captured_count += 1;
        }

        captured_count
    }

    pub fn would_be_suicide(&self, r: usize, c: usize, player: Player) -> bool {
        let mut temp_board = self.board.clone();

        if !temp_board.place_stone(r, c, player) {
            return true;
        }

        // Check if placing this stone would capture opponent stones
        let opponent = player.opponent();
        let mut would_capture = false;

        for (adj_r, adj_c) in temp_board.get_adjacent_points(r, c) {
            if temp_board.get_point(adj_r, adj_c) == Some(Point::Stone(opponent)) {
                let adj_group = temp_board.get_group(adj_r, adj_c);
                if temp_board.count_liberties(&adj_group) == 0 {
                    would_capture = true;
                    break;
                }
            }
        }

        // If it captures opponent stones, it's not suicide
        if would_capture {
            return false;
        }

        // Check if our own group has liberties
        let our_group = temp_board.get_group(r, c);
        temp_board.count_liberties(&our_group) == 0
    }

    pub fn would_repeat_position(&self, r: usize, c: usize, player: Player) -> bool {
        let mut temp_board = self.board.clone();

        if !temp_board.place_stone(r, c, player) {
            return true;
        }

        // Simulate captures
        let opponent = player.opponent();
        for adj_r in 0..SIZE {
            for adj_c in 0..SIZE {
                if temp_board.get_point(adj_r, adj_c) == Some(Point::Stone(opponent)) {
                    let group = temp_board.get_group(adj_r, adj_c);
                    if temp_board.count_liberties(&group) == 0 {
                        for &(gr, gc) in &group {
                            temp_board.remove_stone(gr, gc);
                        }
                    }
                }
            }
        }

        // Check against position history
        self.position_history.contains(&temp_board.points)
    }

    pub fn is_game_over(&self) -> bool {
        self.consecutive_passes >= 2
    }

    pub fn make_move(&mut self, mv: BadukMove) -> Result<(), MoveError> {
        match mv {
            BadukMove::Pass => {
                self.consecutive_passes += 1;
                self.turn = self.turn.opponent();
                self.ko_point = None;
                Ok(())
            }
            BadukMove::Play {
                coordinates: (r, c),
            } => {
                if !self.is_legal_move(r, c) {
                    return Err(MoveError::IllegalMove);
                }

                // Save current position to history
                self.position_history.push(self.board.points);

                // Place the stone
                self.board.place_stone(r, c, self.turn);

                // Reset consecutive passes
                self.consecutive_passes = 0;

                // Capture opponent stones
                let opponent = self.turn.opponent();
                let captured = self.remove_captured_stones(opponent);

                // Update capture count
                match self.turn {
                    Player::Black => self.captures.0 += captured,
                    Player::White => self.captures.1 += captured,
                }

                // Handle ko detection (simple ko - single stone recapture)
                self.ko_point = if captured == 1 {
                    // Check if this was a single stone capture that could create ko
                    let our_group = self.board.get_group(r, c);
                    if our_group.len() == 1 && self.board.count_liberties(&our_group) == 1 {
                        // Find the liberty (potential ko point)
                        self.board
                            .get_adjacent_points(r, c)
                            .into_iter()
                            .find(|&(adj_r, adj_c)| {
                                self.board.get_point(adj_r, adj_c) == Some(Point::Empty)
                            })
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Switch turns
                self.turn = self.turn.opponent();

                Ok(())
            }
        }
    }

    fn is_legal_move(&self, r: usize, c: usize) -> bool {
        // Check if position is empty
        if self.board.get_point(r, c) != Some(Point::Empty) {
            return false;
        }

        // Check ko rule
        if self.ko_point == Some((r, c)) {
            return false;
        }

        // Check suicide rule
        if self.would_be_suicide(r, c, self.turn) {
            return false;
        }

        // Check position repetition
        if self.would_repeat_position(r, c, self.turn) {
            return false;
        }

        true
    }

    pub fn calculate_territory(
        &self,
    ) -> (
        Vec<(usize, usize)>,
        Vec<(usize, usize)>,
        Vec<(usize, usize)>,
    ) {
        let mut visited = std::collections::HashSet::new();
        let mut black_territory = Vec::new();
        let mut white_territory = Vec::new();
        let mut neutral_territory = Vec::new();

        for r in 0..SIZE {
            for c in 0..SIZE {
                if self.board.get_point(r, c) == Some(Point::Empty) && !visited.contains(&(r, c)) {
                    let mut empty_group = Vec::new();
                    let mut bordering_stones = std::collections::HashSet::new();

                    self.flood_fill_territory(
                        r,
                        c,
                        &mut empty_group,
                        &mut bordering_stones,
                        &mut visited,
                    );

                    // Determine territory ownership based on bordering stones
                    let black_borders = bordering_stones.contains(&Player::Black);
                    let white_borders = bordering_stones.contains(&Player::White);

                    match (black_borders, white_borders) {
                        (true, false) => black_territory.extend(empty_group),
                        (false, true) => white_territory.extend(empty_group),
                        _ => neutral_territory.extend(empty_group), // Both or neither
                    }
                }
            }
        }

        (black_territory, white_territory, neutral_territory)
    }

    fn flood_fill_territory(
        &self,
        r: usize,
        c: usize,
        empty_group: &mut Vec<(usize, usize)>,
        bordering_stones: &mut std::collections::HashSet<Player>,
        visited: &mut std::collections::HashSet<(usize, usize)>,
    ) {
        if visited.contains(&(r, c)) {
            return;
        }

        match self.board.get_point(r, c) {
            Some(Point::Empty) => {
                visited.insert((r, c));
                empty_group.push((r, c));

                for (adj_r, adj_c) in self.board.get_adjacent_points(r, c) {
                    self.flood_fill_territory(adj_r, adj_c, empty_group, bordering_stones, visited);
                }
            }
            Some(Point::Stone(player)) => {
                bordering_stones.insert(player);
            }
            None => {}
        }
    }

    pub fn calculate_score(&self) -> (f32, f32) {
        let (black_territory, white_territory, _) = self.calculate_territory();

        // Count stones on board
        let mut black_stones = 0;
        let mut white_stones = 0;

        for r in 0..SIZE {
            for c in 0..SIZE {
                match self.board.get_point(r, c) {
                    Some(Point::Stone(Player::Black)) => black_stones += 1,
                    Some(Point::Stone(Player::White)) => white_stones += 1,
                    _ => {}
                }
            }
        }

        let black_score = black_stones as f32 + black_territory.len() as f32;
        let white_score = white_stones as f32 + white_territory.len() as f32 + 6.5; // 6.5 komi

        (black_score, white_score)
    }

    pub fn get_winner(&self) -> Option<Player> {
        if !self.is_game_over() {
            return None;
        }

        let (black_score, white_score) = self.calculate_score();

        if black_score > white_score {
            Some(Player::Black)
        } else if white_score > black_score {
            Some(Player::White)
        } else {
            None // Draw
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum BadukMove {
    Play { coordinates: (usize, usize) },
    Pass,
}

#[derive(Error, Debug)]
pub enum MoveError {
    #[error("Illegal Move")]
    IllegalMove,
    #[error("Unexpected Missing Move in Game Tree")]
    MissingMove,
}

pub trait StatelessGame: Sized + Clone {
    type Move: Hash + Eq + Copy;
    fn list_all_legal_moves(&self) -> Vec<Self::Move>;
    fn is_legal(&self, game_move: &Self::Move) -> bool;
    fn generate_next_board(&self, game_move: &Self::Move) -> Result<Self, MoveError>;
}

impl<const SIZE: usize> StatelessGame for BadukClassical<SIZE> {
    type Move = BadukMove;

    fn list_all_legal_moves(&self) -> Vec<Self::Move> {
        let mut moves = Vec::new();

        // Always allow pass
        moves.push(BadukMove::Pass);

        // Add all legal stone placements
        for r in 0..SIZE {
            for c in 0..SIZE {
                if self.is_legal_move(r, c) {
                    moves.push(BadukMove::Play {
                        coordinates: (r, c),
                    });
                }
            }
        }

        moves
    }

    fn is_legal(&self, game_move: &Self::Move) -> bool {
        match game_move {
            BadukMove::Pass => true,
            BadukMove::Play {
                coordinates: (r, c),
            } => self.is_legal_move(*r, *c),
        }
    }

    fn generate_next_board(&self, game_move: &Self::Move) -> Result<Self, MoveError> {
        let mut next_game = self.clone();
        next_game.make_move(*game_move)?;
        Ok(next_game)
    }
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
