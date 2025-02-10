use colored::Colorize;
use getch_rs::{Getch, Key};
use snake::direction::Direction;
use snake::map::Map;
use snake::snake::Snake;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread, time};

fn main() {
    println!("Hello, world!");
    let mut snake = Snake::new();
    let mut map = Map::new(15, 40);
    map.add_snake(&mut snake);
    map.add_apple(snake.body.clone());

    let stdin_channel = spawn_stdin_channel();

    loop {
        print_header();
        map.render();
        println!("Score: {}", snake.body.len());

        let direction = match stdin_channel.try_recv() {
            Ok(Key::Up | Key::Char('w')) => Direction::UP,
            Ok(Key::Down | Key::Char('s')) => Direction::DOWN,
            Ok(Key::Left | Key::Char('a')) => Direction::LEFT,
            Ok(Key::Right | Key::Char('d')) => Direction::RIGHT,
            Ok(Key::Char('q')) | Ok(Key::Char('Q')) => break,
            Err(TryRecvError::Empty) => snake.direction,
            Err(_) => break,
            _ => continue,
        };

        if direction == snake.direction.opposite() {
            continue;
        }

        // Check map wall hit
        let (x, y) = snake.body[0];
        let out_of_bounds = match direction {
            Direction::UP => y == 0,
            Direction::DOWN => y == map.height - 1,
            Direction::LEFT => x == 0,
            Direction::RIGHT => x == map.width - 1,
        };
        if out_of_bounds {
            println!("{}", "Game over!".red());
            break;
        }

        snake.direction = direction;

        // Check snake body hit
        if !snake.walk(direction) {
            println!("{}", "Game over!".red());
            break;
        }

        let eat = snake.body[0] == map.apple;
        map.update_snake(&mut snake, eat);
        sleep(100);
    }
}

fn spawn_stdin_channel() -> Receiver<Key> {
    let (tx, rx) = mpsc::channel::<Key>();
    thread::spawn(move || loop {
        let g = Getch::new();
        tx.send(g.getch().unwrap()).unwrap();
    });
    rx
}

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}

fn print_header() {
    std::process::Command::new("clear").status().unwrap(); // Clear screen
    println!("{}", " ____              _".yellow());
    println!("{}", "/ ___| _ __   __ _| | _____".yellow());
    println!("{}", "\\___ \\| '_ \\ / _` | |/ / _ \\".yellow());
    println!("{}", " ___) | | | | (_| |   <  __/".yellow());
    println!("{}", "|____/|_| |_|\\__,_|_|\\_\\___|".yellow());
    println!(
        "\n - Use {} or the {} keys to move",
        "WASD".blue(),
        "arrow".blue()
    );
    println!(" - Press {} to quit", "q".red());
}
