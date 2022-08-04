use bit_vec::BitVec;

pub struct Game {
    pub state: BitVec,
}

impl Game {
    #[must_use]
    pub fn new(size: usize) -> Self {
        Self {
            state: BitVec::from_elem(size, false),
        }
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.state.len()
    }

    #[must_use]
    pub fn amount_played(&self) -> usize {
        self.state.iter().filter(|&x| x).count()
    }

    #[must_use]
    pub fn can_play(&self, x: usize) -> bool {
        !self.state[x]
    }

    /// The game is over when there are 3 or more consecutive 1s in the state.
    #[must_use]
    pub fn game_over(&self) -> bool {
        let mut consecutive = 0;
        for i in 0..self.size() {
            if self.state[i] {
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
/// Solves a treblecross game using the negamax formula.
/// The game is over when there are 3 filled cells (1s) in a row.
#[must_use]
pub fn solve(game: &Game) -> impl Iterator<Item = f32> + '_ {
    game.state.iter().enumerate().map(|(x, _)| -> f32 {
        let x = x as usize;

        if game.can_play(x) && game.is_winning_move(x) {
            return ((game.size() + 1) as f32 - game.amount_played() as f32) / 2f32;
        }

        if game.can_play(x) {
            let mut new_game = game.clone();
            new_game.play(x);
            return -(&solve(&new_game).reduce(f32::max).unwrap() as &f32);
        }

        -(game.size() as f32)
    })
}

#[must_use]
pub fn solve_and_collect(game: &Game) -> Vec<f32> {
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
    fn negative_length_if_played() {
        let mut game = Game::new(5);

        game.play(2);

        assert!(!game.can_play(2));
        assert_eq!(solve_and_collect(&game)[2], -5f32);
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
