use crate::direction::Direction;

pub struct Snake {
    pub direction: Direction,
    pub body: Vec<(usize, usize)>,
}

impl Snake {
    pub fn new() -> Snake {
        return Snake {
            direction: Direction::RIGHT,
            body: vec![],
        };
    }

    pub fn walk(&mut self, direction: Direction) -> bool {
        let (x, y) = self.body[0];

        let next_position = match direction {
            Direction::UP => (x, y - 1),
            Direction::DOWN => (x, y + 1),
            Direction::LEFT => (x - 1, y),
            Direction::RIGHT => (x + 1, y),
        };

        if self.body.contains(&next_position) {
            return false;
        }

        self.body.insert(0, next_position);
        return true;
    }
}
