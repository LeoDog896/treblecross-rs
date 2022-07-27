# treblecross-solver

Easily solve any treblecross puzzle.

## Usage

```rs
use lib_treblecross::{solve, solve_and_collect, Game};

fn main() {
    let mut game = Game::new(5);

    let result = solve_and_collect(&mut game); // [-1.5, -1.5, 2, -1.5, -1.5]

    game.play(0);
    game.play(1);

    let winning_move = game.is_winning_move(2); // true

    game.play(2);

    let game_over = game.game_over(); // true
}
```