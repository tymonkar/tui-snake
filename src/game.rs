use rand::Rng;
use std::collections::VecDeque;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    pub screen_size: (u16, u16),
    pub snake: Snake,
    pub food_position: (u16, u16),
}

impl Game {
    pub fn new(mut screen_size: (u16, u16)) -> Self {
        if screen_size.0 % 2 == 1 {
            screen_size.0 -= 1;
        }
        let new_snake = Snake::new((screen_size.0 / 4) * 2, screen_size.1 / 2);
        let new_food = Self::generate_food(&new_snake, &screen_size);
        Self {
            screen_size: screen_size,
            snake: new_snake,
            food_position: new_food,
        }
    }
    pub fn update(&mut self) -> bool {
        let head = self.snake.body.front().unwrap();
        let mut new_head = (head.0 as i32, head.1 as i32);

        match self.snake.direction {
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
            Direction::Left => new_head.0 -= 2,
            Direction::Right => new_head.0 += 2,
        }

        let width = self.screen_size.0 as i32;
        let height = self.screen_size.1 as i32;

        new_head.0 = (new_head.0 + width) % width;
        new_head.1 = (new_head.1 + height) % height;

        let new_head = (new_head.0 as u16, new_head.1 as u16);

        if new_head == self.food_position {
            self.snake.length += 1;
            self.food_position = Self::generate_food(&self.snake, &self.screen_size);
        }

        if self.snake.body.contains(&new_head) {
            return false;
        }

        if self.snake.length == self.snake.body.len() {
            self.snake.body.pop_back();
        }
        self.snake.body.push_front(new_head);
        true
    }

    pub fn change_sreen_size(&mut self, screen_size: (u16, u16)) {
        let (mut cols, rows) = screen_size;
        if cols % 2 == 1 {
            cols -= 1;
        }
        self.screen_size = (cols, rows);

        if self.food_position.0 > screen_size.0 || self.food_position.1 > screen_size.1 {
            self.food_position = Self::generate_food(&self.snake, &self.screen_size);
        }
    }

    fn generate_food(snake: &Snake, screen_size: &(u16, u16)) -> (u16, u16) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..screen_size.0 / 2) * 2;
            let y = rng.gen_range(0..screen_size.1 / 2) * 2;
            let new_food = (x, y);
            if !snake.body.contains(&new_food) {
                return new_food;
            }
        }
    }
}

pub struct Snake {
    pub body: VecDeque<(u16, u16)>,
    length: usize,
    pub direction: Direction,
}

impl Snake {
    fn new(x: u16, y: u16) -> Self {
        Self {
            body: VecDeque::from([(x, y), (x - 1, y), (x - 2, y)]),
            length: 3,
            direction: Direction::Right,
        }
    }
    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}
