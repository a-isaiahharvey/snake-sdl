use std::alloc::{alloc, Layout};

use sdl2_sys::{SDL_EventType, SDL_KeyCode, SDL_Point, SDL_PollEvent, SDL_fmod};

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub direction: Direction,
    pub speed: f64,
    pub size: isize,
    pub alive: bool,
    pub head_x: f64,
    pub head_y: f64,
    pub body: Vec<SDL_Point>,
    growing: bool,
    grid_width: usize,
    grid_height: usize,
}

impl Snake {
    pub fn new(grid_width: usize, grid_height: usize) -> Self {
        Self {
            direction: Direction::Up,
            speed: 0.1,
            size: 1,
            alive: true,
            head_x: grid_width as f64 / 2.,
            head_y: grid_height as f64 / 2.,
            body: Default::default(),
            growing: false,
            grid_width,
            grid_height,
        }
    }

    pub fn update(&mut self) {
        let prev_cell = SDL_Point {
            x: self.head_x as i32,
            y: self.head_y as i32,
        };

        self.update_head();

        let current_cell = SDL_Point {
            x: self.head_x as i32,
            y: self.head_y as i32,
        };

        if current_cell.x != prev_cell.x || current_cell.y != prev_cell.y {
            self.update_body(&current_cell, &prev_cell);
        }
    }

    pub fn grow_body(&mut self) {
        self.growing = true;
    }

    pub fn snake_cell(&self, x: i32, y: i32) -> bool {
        if x == self.head_x as i32 && y == self.head_y as i32 {
            return true;
        }

        for item in &self.body {
            if x == item.x && y == item.y {
                return true;
            }
        }

        false
    }

    pub fn handle_input(&mut self, running: &mut bool) {
        unsafe {
            let layout = Layout::new::<sdl2_sys::SDL_Event>();
            let event = alloc(layout) as *mut sdl2_sys::SDL_Event;

            while SDL_PollEvent(event) != 0 {
                if (*event).type_ == SDL_EventType::SDL_QUIT as u32 {
                    *running = false;
                } else if (*event).type_ == SDL_EventType::SDL_KEYDOWN as u32 {
                    match (*event).key.keysym.sym {
                        k if SDL_KeyCode::SDLK_UP as i32 == k => {
                            self.change_direction(Direction::Up, Direction::Down)
                        }

                        k if SDL_KeyCode::SDLK_DOWN as i32 == k => {
                            self.change_direction(Direction::Down, Direction::Up)
                        }

                        k if SDL_KeyCode::SDLK_LEFT as i32 == k => {
                            self.change_direction(Direction::Left, Direction::Right)
                        }

                        k if SDL_KeyCode::SDLK_RIGHT as i32 == k => {
                            self.change_direction(Direction::Right, Direction::Left)
                        }

                        _ => continue,
                    }
                }
            }
        }
    }

    fn change_direction(&mut self, input: Direction, opposite: Direction) {
        if self.direction != opposite || self.size == 1 {
            self.direction = input
        }
    }

    fn update_head(&mut self) {
        match self.direction {
            Direction::Up => self.head_y -= self.speed,
            Direction::Down => self.head_y += self.speed,
            Direction::Left => self.head_x -= self.speed,
            Direction::Right => self.head_x += self.speed,
        };

        unsafe {
            self.head_x = SDL_fmod(self.head_x + self.grid_width as f64, self.grid_width as f64);
            self.head_y = SDL_fmod(
                self.head_y + self.grid_height as f64,
                self.grid_height as f64,
            )
        }
    }

    fn update_body(&mut self, current_cell: &SDL_Point, prev_cell: &SDL_Point) {
        // Add previous head location to vector
        self.body.push(*prev_cell);

        if !self.growing {
            // Remove the tail from the vector.
            self.body.remove(0);
        } else {
            self.growing = false;
            self.size += 1;
        }

        // Check if the snake has died.
        for item in &self.body {
            if current_cell.x == item.x && current_cell.y == item.y {
                self.alive = false;
            }
        }
    }
}
