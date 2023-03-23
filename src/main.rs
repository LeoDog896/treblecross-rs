use console::{Key, Style, Term};
use dialoguer::{theme::ColorfulTheme, Input};
use lib_treblecross::{solve_and_collect, Game};
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    cmp::min,
    io::Write, time::Duration,
};

fn show_calculating() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    pb.set_message("Calculating...");
    pb
}

fn print_game(game: &Game, position: usize, term: &mut Term, solved: &Vec<f32>) -> std::io::Result<()> {
    {
        for i in 0..game.size() {
            let position = position;

            let style = match position {
                _ if position == i => Style::new().red(),
                _ => Style::new().white(),
            };

            term.write_all(
                format!(
                    "{: >4} ",
                    style.apply_to(if game.can_play(i) { "." } else { "X" })
                )
                .as_bytes(),
            )?;
        }
    }

    term.write_line("")?;

    for num in solved {
        term.write_all(format!("{: >4} ", num).as_bytes())?;
    }

    term.write_line("")?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut cursor_position: usize = 0;

    let mut stdout = Term::stdout();

    let length = Input::<usize>::with_theme(&ColorfulTheme::default())
        .with_prompt("Board Length")
        .default(5)
        .interact_text()
        .unwrap();

    let mut pb = show_calculating();

    let mut game = Game::new(length);
    let mut solved = solve_and_collect(&game);

    loop {
        pb.finish();

        if game.game_over() {
            assert!(game.amount_played() != 0); // can't win if you haven't played yet
            stdout.write_line(&format!("Game finished! Player {} won!", if game.amount_played() % 2 == 0 { "2" } else { "1" }))?;
            break;
        }

        stdout.clear_screen()?;

        print_game(&game, cursor_position, &mut stdout, &solved)?;

        if let Ok(key) = stdout.read_key() {
            match key {
                Key::Char('a') | Key::ArrowLeft => {
                    cursor_position = ((cursor_position as isize) - 1).try_into().unwrap_or(0);
                }
                Key::Char('d') | Key::ArrowRight => {
                    cursor_position = min(length - 1, cursor_position + 1);
                }
                Key::Enter => {
                    if game.can_play(cursor_position) {
                        pb = show_calculating();
                        game.play(cursor_position);
                        solved = solve_and_collect(&game);
                    }
                }
                _ => continue,
            }
        }
    }

    Ok(())
}
