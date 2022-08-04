use console::{Key, Style, Term};
use dialoguer::{theme::ColorfulTheme, Input};
use lib_treblecross::{solve_and_collect, Game};
use std::{
    cmp::{max, min},
    io::Write,
};

fn print_game(game: &Game, position: usize, term: &mut Term) -> std::io::Result<()> {
    {
        let state = game.state.clone();
        for i in 0..game.size() {
            let position = position;

            let style = match position {
                _ if position == i => Style::new().red(),
                _ => Style::new().white(),
            };

            term.write_all(
                format!(
                    "{: >4} ",
                    style.apply_to(match state.get(i) {
                        Some(false) | None => ".",
                        Some(true) => "X",
                    })
                )
                .as_bytes(),
            )?;
        }
        term.write_line("")?;
    }

    {
        let solved = solve_and_collect(game);
        for i in 0..game.size() {
            term.write_all(format!("{: >4} ", solved[i as usize]).as_bytes())?;
        }

        term.write_line("")?;
    }

    Ok(())
}

fn main() {
    main_err().unwrap();
}

fn main_err() -> std::io::Result<()> {
    let mut cursor_position: usize = 0;

    let mut stdout = Term::stdout();

    let length = Input::<usize>::with_theme(&ColorfulTheme::default())
        .with_prompt("Board Length")
        .default(5)
        .interact_text()
        .unwrap();

    let mut game = Game::new(length);

    loop {
        let game_over = game.game_over();

        if game_over {
            stdout.write_line("Game over!")?;
            break;
        }

        stdout.clear_screen()?;

        print_game(&game, cursor_position, &mut stdout)?;

        if let Ok(key) = stdout.read_key() {
            match key {
                Key::Char('a') | Key::ArrowLeft => cursor_position = max(0, cursor_position - 1),
                Key::Char('d') | Key::ArrowRight => {
                    cursor_position = min(length - 1, cursor_position + 1);
                }
                Key::Enter => {
                    if game.can_play(cursor_position) {
                        game.play(cursor_position);
                    }
                }
                _ => continue,
            }
        }
    }

    Ok(())
}
