//! Simple treblecross solver.

use bit_vec::BitVec;

/// A game of treblecross represented as a list of binary values.
pub struct Game {
    pub state: BitVec,
}

impl Game {
    /// Create a new game of treblecross with the given size.
    #[must_use]
    pub fn new(size: usize) -> Self {
        Self {
            state: BitVec::from_elem(size, false),
        }
    }

    /// Get the size of the board.
    #[must_use]
    pub fn size(&self) -> usize {
        self.state.len()
    }

    /// Get the number of cells that are filled in the game, aka the amount of moves made in the game.
    #[must_use]
    pub fn amount_played(&self) -> usize {
        self.state.iter().filter(|&x| x).count()
    }

    /// Check if a player can play in this cell (if it is not filled)
    #[must_use]
    pub fn can_play(&self, x: usize) -> bool {
        !self.state[x]
    }

    /// The game is over when there are 3 or more consecutive 1s in the state.
    #[must_use]
    pub fn game_over(&self) -> bool {
        let mut consecutive = 0;
        for val in &self.state {
            if val {
                consecutive += 1;
            } else {
                consecutive = 0;
            }
            if consecutive >= 3 {
                return true;
            }
        }
        false
    }

    /// True if the move makes 3 or more consecutive 1s in the state.
    #[must_use]
    pub fn is_winning_move(&self, x: usize) -> bool {
        let mut consecutive = 0;
        for i in 0..self.size() {
            if i == x || self.state[i] {
                consecutive += 1;
            } else {
                consecutive = 0;
            }
            if consecutive >= 3 {
                return true;
            }
        }
        false
    }

    /// Play a move in the game.
    pub fn play(&mut self, x: usize) {
        self.state.set(x, true);
    }
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

fn negamax(game: &Game, mut alpha: isize, beta: isize) -> isize {
    for x in 0..game.size() {
        if game.can_play(x) && game.is_winning_move(x) {
            return (game.size() as isize + 1 - game.amount_played() as isize) / 2;
        }
    }

    for x in 0..game.size() {
        if game.can_play(x) {
            let mut new_game = game.clone();
            new_game.play(x);

            let score = -negamax(&new_game, -beta, -alpha);

            if score >= beta - 1 {
                return score;
            }

            if score > alpha {
                alpha = score;
            }
        }
    }

    alpha
}

/// Solves a treblecross game using the negamax formula.
/// The game is over when there are 3 filled cells (1s) in a row.
pub fn solve(game: &Game) -> impl Iterator<Item = Option<isize>> + '_ {
    game.state.iter().enumerate().map(move |(x, _)| -> Option<isize> {
        if !game.can_play(x) {
            return None;
        }
        let mut new_game = game.clone();
        new_game.play(x);
        Some(negamax(&new_game, -(game.size() as isize) / 2, game.size() as isize) / 2)
    })
}

#[must_use]
pub fn solve_and_collect(game: &Game) -> Vec<Option<isize>> {
    solve(game).collect()
}

#[cfg(test)]
mod tests {

    use crate::{solve_and_collect, Game};

    #[test]
    fn winning_move() {
        let mut game = Game::new(5);

        game.play(0);
        game.play(1);

        assert!(game.is_winning_move(2));

        let mut game = Game::new(5);

        game.play(1);
        game.play(2);

        assert!(game.is_winning_move(3));
        assert!(game.is_winning_move(0));
    }

    #[test]
    fn length() {
        let game = Game::new(5);

        assert_eq!(game.size(), 5);
    }

    #[test]
    fn none_if_played() {
        let mut game = Game::new(5);

        game.play(2);

        assert!(!game.can_play(2));
        assert_eq!(solve_and_collect(&game)[2], None);
    }

    #[test]
    fn piece_set() {
        let mut game = Game::new(5);
        game.play(0);

        assert!(game.state[0]);
    }

    #[test]
    fn game_over() {
        let mut game = Game::new(5);
        game.play(0);
        game.play(1);
        game.play(2);

        assert!(game.game_over());
    }
}
