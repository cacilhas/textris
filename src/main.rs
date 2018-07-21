extern crate termion;
extern crate textris;

use std::io::{stdout, Read, Write};
use std::thread;
use std::time::Duration;
use termion::async_stdin;
use termion::raw::IntoRawMode;
use textris::coord::Dir;
use textris::game::Game;

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide,
    ).unwrap();

    let mut game = Game::new();

    let interval = Duration::from_millis(32);
    let mut t = 0;
    loop {
        match stdin.next() {
            Some(Ok(key)) => match key {
                b'q' => break,
                b'h' => game.slide_piece(Dir::Left),
                b'l' => game.slide_piece(Dir::Right),
                b'j' => game.slide_piece(Dir::Down),
                b'd' => game.rotate_piece(false),
                b'f' => game.rotate_piece(true),
                _ => {}
            },
            _ => {}
        }

        if t % 20 == 0 {
            match game.tick() {
                Ok(_) => {},
                Err(_) => {
                    render_game_over(&mut stdout);
                    break;
                }
            }
        }

        render(&game, &mut stdout);
        stdout.flush().unwrap();

        thread::sleep(interval);
        t += 1;
    }

    stdout.flush().unwrap();

    // Wait any key inputs before exiting.
    loop {
        match stdin.next() {
            Some(_) => break,
            _ => {},
        }
        thread::sleep(interval);
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn render(g: &Game, w: &mut Write) {
    for (i, line) in g.field().lines_iter().enumerate() {
        write!(w, "{}", termion::cursor::Goto(1, (i as u16) + 1)).unwrap();
        for cell in line.iter() {
            match cell {
                Some(block) => write!(w, "{} ", block.chr),
                None => write!(w, "  "),
            }.unwrap();
        }
    }
}

fn render_game_over(w: &mut Write) {
    write!(w, "{}", termion::cursor::Goto(1, 1)).unwrap();
    write!(w, "====== GAME OVER ======").unwrap();
}
