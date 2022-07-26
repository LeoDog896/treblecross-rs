pub struct Game {
  pub state: Vec<u16>,
  pub size: u16
}

impl Game {
    #[must_use]
    pub fn new(size: u16) -> Self {
        Self {
            state: vec![0; size as usize],
            size
        }
    }

    pub fn amount_played(&self) -> u16 {
        self.state.iter().filter(|&x| *x == 1).count() as u16
    }

    pub fn can_play(&self, x: u16) -> bool {
        self.state[x as usize] == 0
    }

    /// The game is over when there are 3 or more consecutive 1s in the state.
    pub fn game_over(&self) -> bool {
        let mut consecutive = 0;
        for i in 0..self.size {
            if self.state[i as usize] == 1 {
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

    pub fn is_winning_move(&self, x: u16) -> bool {
        let mut consecutive = 0;
        for i in 0..self.size {
            if i == x || self.state[i as usize] == 1 {
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

    pub fn play(mut self, x: u16) -> Self {
        if self.can_play(x) {
            self.state[x as usize] = 1;
        }

        self
    }
    
    pub fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            size: self.size
        }
    }}
/// Solves a treblecross game using the negamax formula.
/// The game is over when there are 3 filled cells (1s) in a row.
pub fn solve(game: &Game) -> Vec<i16> {

    game.state.iter().enumerate().map(|(_, &x)| -> i16 {
        if game.can_play(x) && game.is_winning_move(x) {
            return (game.size as i16 + 1 - game.amount_played() as i16) / 2;
        }

        if game.can_play(x) {
            let new_game = game.clone().play(x);
            return -(solve(&new_game).iter().max().unwrap() as &i16)
        }

        return -(game.size as i16);
    }).collect::<Vec<i16>>()
}
