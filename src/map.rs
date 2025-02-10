use crate::snake::Snake;
use colored::Colorize;
use rand::Rng;

pub struct Map {
    positions: Vec<Vec<&'static str>>,
    pub height: usize,
    pub width: usize,
    pub apple: (usize, usize),
}

impl Map {
    pub fn new(height: usize, width: usize) -> Map {
        return Map {
            height,
            width,
            apple: (width / 2 + 3, height / 2),
            positions: vec![vec![" "; width]; height],
        };
    }

    pub fn add_snake(&mut self, snake: &mut Snake) {
        let head = (self.width / 2, self.height / 2);
        snake.body.push(head);

        for &(x, y) in snake.body.iter() {
            self.positions[y][x] = "█"
        }
    }

    pub fn update_snake(&mut self, snake: &mut Snake, eat: bool) {
        let (x, y) = snake.body[0];

        self.positions[y][x] = "█";

        if !eat {
            let (x, y) = snake.body.pop().unwrap();
            self.positions[y][x] = " ";
        } else {
            self.add_apple(snake.body.clone());
        }
    }

    pub fn add_apple(&mut self, unavailable_positions: Vec<(usize, usize)>) {
        let mut x = rand::rng().random_range(0..self.width);
        let mut y = rand::rng().random_range(0..self.height);

        while unavailable_positions.contains(&(x, y)) {
            x = rand::rng().random_range(0..self.width);
            y = rand::rng().random_range(0..self.height);
        }

        self.apple = (x, y);
        self.positions[y][x] = "■";
    }

    pub fn render(&self) {
        print!("╔");
        for _ in 0..self.width {
            print!("═")
        }
        print!("╗");
        println!("");

        for row in self.positions.iter() {
            print!("║");
            for col in row {
                if col == &"■" {
                    print!("{}", col.red().bold());
                } else if col == &"█" {
                    print!("{}", col.green().bold());
                } else {
                    print!("{col}");
                }
            }
            print!("║");
            println!("");
        }

        print!("╚");
        for _ in 0..self.width {
            print!("═")
        }
        print!("╝");
        println!("");
    }
}
