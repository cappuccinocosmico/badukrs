use crate::game::{MoveError, StatelessGame};
use rand::Rng;

pub struct RandomBot<G: StatelessGame> {
    _phantom: std::marker::PhantomData<G>,
}

pub trait GameBot {
    type Game: StatelessGame;
    fn new() -> Self;
    fn select_move(
        &self,
        game: &Self::Game,
    ) -> Result<<Self::Game as StatelessGame>::Move, MoveError>;
}

impl<G: StatelessGame> GameBot for RandomBot<G> {
    type Game = G;
    fn new() -> Self {
        RandomBot {
            _phantom: std::marker::PhantomData,
        }
    }

    fn select_move(&self, game: &G) -> Result<G::Move, MoveError> {
        let legal_moves = game.list_all_legal_moves();

        if legal_moves.is_empty() {
            return Err(MoveError::IllegalMove);
        }

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..legal_moves.len());

        Ok(legal_moves[random_index])
    }
}

