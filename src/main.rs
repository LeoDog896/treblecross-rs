use console::{Style, Term, Key};
use dialoguer::{theme::ColorfulTheme, Input};
use std::cmp::{max, min};
use anyhow::Result;
use treblecross::{solve, Game};

fn print_game(game: &Game, position: usize) {
    {
        let state = game.state.clone();
        let mut board = String::new();
        for i in 0..game.size() {

            let position = position;

            let style = match position {
                _ if position == i => Style::new().red(),
                _ => Style::new().white(),
            };

            board.push_str(&format!(
                "{: >4} ",
                style.apply_to(match state.get(i as usize) {
                    Some(0) | None => ".",
                    Some(1) => "X",
                    _ => unreachable!(),
                })
            ));
        }
        println!("{}", board);
    }

    {
        let solved = solve(game);
        let mut board = String::new();
        for i in 0..game.size() {
            board.push_str(&format!("{: >4} ", solved[i as usize]));
        }

        println!("{}", board);
    }
}

fn main() {
    main_err().unwrap();
}

fn main_err() -> Result<()> {
    let mut cursor_position: usize = 0;

    let stdout = Term::stdout();

    let length: usize = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Length of board")
        .default(5)
        .interact_text()
        .unwrap();

    let mut game = Game::new(length);

    loop {

        let game_over = game.game_over();
        
        if game_over {
            println!("Game over!");
            break;
        }

        stdout.clear_screen()?;

        print_game(&game, cursor_position);

        if let Ok(key) = stdout.read_key() {
            match key {
                Key::Char('a') | Key::ArrowLeft => cursor_position = max(0, cursor_position - 1),
                Key::Char('d') | Key::ArrowRight => cursor_position = min(length - 1, cursor_position + 1),
                Key::Enter => {
                    if game.can_play(cursor_position) {
                        game.play(cursor_position);
                    }
                }
                _ => continue
            }
        }
    }

    Ok(())
}
