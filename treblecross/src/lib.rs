pub struct Game {
    pub state: Vec<u16>
}

impl Game {
    #[must_use]
    pub fn new(size: usize) -> Self {
        Self {
            state: vec![0; size]
        }
    }

    pub fn size(&self) -> usize {
        self.state.len()
    }

    pub fn amount_played(&self) -> usize {
        self.state.iter().filter(|&x| *x == 1).count()
    }

    pub fn can_play(&self, x: usize) -> bool {
        self.state[x] == 0
    }

    /// The game is over when there are 3 or more consecutive 1s in the state.
    pub fn game_over(&self) -> bool {
        let mut consecutive = 0;
        for i in 0..self.size() {
            if self.state[i] == 1 {
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

    pub fn is_winning_move(&self, x: usize) -> bool {
        let mut consecutive = 0;
        for i in 0..self.size() {
            if i == x || self.state[i] == 1 {
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
        self.state[x] = 1;
    }
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone()
        }
    }
}
/// Solves a treblecross game using the negamax formula.
/// The game is over when there are 3 filled cells (1s) in a row.
pub fn solve(game: &Game) -> impl Iterator<Item = i16> + '_ {
    game.state
        .iter()
        .enumerate()
        .map(|(_, &x)| -> i16 {
            
            let x = x as usize;

            if game.can_play(x) && game.is_winning_move(x) {
                return ((game.size() + 1) as i16 - game.amount_played() as i16) / 2;
            }

            if game.can_play(x) {
                let mut new_game = game.clone();
                new_game.play(x);
                return -(&solve(&new_game).max().unwrap() as &i16);
            }

            return -(game.size() as i16);
        })
}

pub fn solve_and_collect(game: &Game) -> Vec<i16> {
    solve(game).collect()
}

#[cfg(test)]
mod tests {

    use crate::Game;

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
    fn piece_set() {
        let mut game = Game::new(5);
        game.play(0);

        assert!(game.state[0] == 1);
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