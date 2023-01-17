use std::ops::Range;

use rand::{rngs::ThreadRng, Rng};
use sdl2_sys::{SDL_Delay, SDL_GetTicks, SDL_Point};

use crate::{renderer::Renderer, snake::Snake};

pub struct Game {
    snake: Snake,
    food: SDL_Point,
    rng: ThreadRng,
    width_range: Range<i32>,
    height_range: Range<i32>,
    score: usize,
}

impl Game {
    pub fn new(grid_width: usize, grid_height: usize) -> Self {
        let mut game = Self {
            snake: Snake::new(grid_width, grid_height),
            food: SDL_Point { x: 0, y: 0 },
            rng: ThreadRng::default(),
            score: 0,
            width_range: 0..grid_width as i32 - 1,
            height_range: 0..grid_height as i32 - 1,
        };

        game.place_food();

        game
    }

    pub fn run(&mut self, renderer: &mut Renderer, target_frame_duration: u32) {
        unsafe {
            let mut title_timestamp = SDL_GetTicks();
            let mut frame_start: u32;
            let mut frame_end: u32;
            let mut frame_duration: u32;
            let mut frame_count = 0;
            let mut running = true;

            while running {
                frame_start = SDL_GetTicks();

                self.snake.handle_input(&mut running);
                self.update();
                renderer.render(&self.snake, self.food);

                frame_end = SDL_GetTicks();

                // Calculates time between cycles
                frame_count += 1;
                frame_duration = frame_end - frame_start;

                // Uupdate the window title after every second.
                if frame_end - title_timestamp >= 1000 {
                    renderer.update_window_title(self.score, frame_count);
                    frame_count = 0;
                    title_timestamp = frame_end;
                }

                if frame_duration < target_frame_duration {
                    SDL_Delay(target_frame_duration - frame_duration);
                }
            }
        }
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn get_size(&self) -> isize {
        self.snake.size
    }

    fn place_food(&mut self) {
        let (mut x, mut y): (i32, i32);
        loop {
            x = self.rng.gen_range(self.width_range.clone());
            y = self.rng.gen_range(self.height_range.clone());

            // Check that the location is not occupied by a snake item before placing
            // food.
            if !self.snake.snake_cell(x, y) {
                self.food.x = x;
                self.food.y = y;
                return;
            }
        }
    }

    fn update(&mut self) {
        if !self.snake.alive {
            return;
        }

        self.snake.update();

        let new_x = self.snake.head_x as i32;
        let new_y = self.snake.head_y as i32;

        // Check if there's food over here
        if self.food.x == new_x && self.food.y == new_y {
            self.score += 1;
            self.place_food();

            // Grow snake and increase speed.
            self.snake.grow_body();
            self.snake.speed += 0.02;
        }
    }
}
