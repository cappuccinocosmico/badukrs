### Baduk Data Structure Ideas (Revised)

This document outlines some ideas for the data structures needed to represent a game of Baduk (Go), incorporating your feedback.

#### 1. `Board<const SIZE: usize>`

The game board will be generic to support various sizes.

*   **Representation:** A 2D array `[[Point; SIZE]; SIZE]`. Using a const generic `SIZE` will allow for compile-time defined board sizes (e.g., 9, 13, 19).
*   **`Point` enum:** Remains the same.

```rust
// In a new `src/game.rs` or similar

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

    // Methods to place a stone, get a stone, etc.
}
```

#### 2. Game Tree and `GameNode`

A `GameState` is best represented as a node within the game tree. The tree itself represents the entire game, with all its variations.

*   **`GameNode`:** Represents a single position (a game state) in the game.
    *   `board`: The `Board` state at this position.
    *   `captures`: Number of captured stones.
    *   `ko_point`: Current ko restriction.
    *   `move_info`: Information about the move that led to this node (e.g., player, coordinates).
    *   `children`: A `Vec<GameNode>` for variations.

This structure is recursive and forms the game tree.

```rust
// In a new `src/game.rs` or similar

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

// The game tree is essentially the root node
pub type GameTree<const SIZE: usize> = GameNode<SIZE>;
```

#### 3. SGF (Smart Game Format)

You're right to point out the naming. SGF stands for "Smart Game Format", and it is the widely-used standard for storing Go game records.

The `GameTree` structure described above can be serialized to and from the SGF format. An SGF file represents one or more game trees.

*   **Serialization:** Traversing our `GameTree` and writing out the data in SGF syntax.
*   **Deserialization:** Parsing an SGF file to build our `GameTree` structure.

A dedicated `src/sgf.rs` module would handle this serialization and deserialization.

**Next Steps:**

I can start by creating a `src/game.rs` file and implementing the generic `Board` and the `GameNode` structures.

Let me know how you'd like to proceed.
