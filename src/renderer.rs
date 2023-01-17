use std::{
    ffi::{c_int, CStr, CString},
    ptr,
};

use log::{error, warn};
use sdl2_sys::{
    SDL_CreateRenderer, SDL_CreateWindow, SDL_DestroyWindow, SDL_GetError, SDL_Init, SDL_Point,
    SDL_Quit, SDL_Rect, SDL_RenderClear, SDL_RenderFillRect, SDL_RenderPresent, SDL_Renderer,
    SDL_RendererFlags, SDL_SetRenderDrawColor, SDL_SetWindowTitle, SDL_Window, SDL_WindowFlags,
    Uint32, SDL_INIT_VIDEO, SDL_WINDOWPOS_CENTERED_MASK,
};

use crate::snake::Snake;

#[derive(Debug)]
pub struct Renderer {
    sdl_window: *mut SDL_Window,
    sdl_renderer: *mut SDL_Renderer,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            sdl_window: ptr::null_mut(),
            sdl_renderer: ptr::null_mut(),
        }
    }
}

impl Renderer {
    const SCREEN_WIDTH: c_int = 640;
    const SCREEN_HEIGHT: c_int = 640;
    const GRID_WIDTH: c_int = 32;
    const GRID_HEIGHT: c_int = 32;

    pub fn new() -> Self {
        unsafe {
            let mut this = Self::default();

            // Initialize SDL
            if SDL_Init(SDL_INIT_VIDEO) < 0 {
                warn!("SDL could not initialize.");
                error!(
                    "SDL_Error: {}",
                    CStr::from_ptr(SDL_GetError())
                        .to_str()
                        .expect("Could not convert error to Rust string")
                );
            }

            // Create Window
            let title = CString::new("Snake Game").expect("Could not convert to C string");
            this.sdl_window = SDL_CreateWindow(
                title.as_ptr(),
                SDL_WINDOWPOS_CENTERED_MASK as i32,
                SDL_WINDOWPOS_CENTERED_MASK as i32,
                Self::SCREEN_WIDTH,
                Self::SCREEN_HEIGHT,
                SDL_WindowFlags::SDL_WINDOW_SHOWN as Uint32,
            );

            if this.sdl_window.is_null() {
                warn!("Window could not be created.");
                error!(
                    " SDL_Error: {}",
                    CStr::from_ptr(SDL_GetError())
                        .to_str()
                        .expect("Could not convert error to Rust string")
                );
            }

            // Create renderer
            this.sdl_renderer = SDL_CreateRenderer(
                this.sdl_window,
                -1,
                SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32,
            );

            if this.sdl_renderer.is_null() {
                warn!("Renderer could not be created.");
                error!(
                    "SDL_Error: {}",
                    CStr::from_ptr(SDL_GetError())
                        .to_str()
                        .expect("Could not convert error to Rust string")
                );
            }

            this
        }
    }

    pub fn render(&mut self, snake: &Snake, food: SDL_Point) {
        unsafe {
            let mut block = SDL_Rect {
                x: 0,
                y: 0,
                w: Self::SCREEN_WIDTH / Self::GRID_WIDTH,
                h: Self::SCREEN_HEIGHT / Self::GRID_HEIGHT,
            };

            // Clear Screen
            SDL_SetRenderDrawColor(self.sdl_renderer, 0x1E, 0x1E, 0x1E, 0xFF);
            SDL_RenderClear(self.sdl_renderer);

            // Render food
            SDL_SetRenderDrawColor(self.sdl_renderer, 0xC5, 0x11, 0x04, 0xFF);
            block.x = food.x * block.w;
            block.y = food.y * block.h;
            SDL_RenderFillRect(self.sdl_renderer, &block);

            // Render snake's body
            SDL_SetRenderDrawColor(self.sdl_renderer, 0x8A, 0xC8, 0x47, 0xFF);
            for point in &snake.body {
                block.x = point.x * block.w;
                block.y = point.y * block.h;
                SDL_RenderFillRect(self.sdl_renderer, &block);
            }

            // Render snake's head
            block.x = snake.head_x as i32 * block.w;
            block.y = snake.head_y as i32 * block.h;
            if snake.alive {
                SDL_SetRenderDrawColor(self.sdl_renderer, 0x52, 0xA7, 0x36, 0xFF);
            } else {
                SDL_SetRenderDrawColor(self.sdl_renderer, 0x8A, 0x03, 0x03, 0xFF);
            }
            SDL_RenderFillRect(self.sdl_renderer, &block);

            // Update Screen
            SDL_RenderPresent(self.sdl_renderer);
        }
    }

    pub fn update_window_title(&self, score: usize, fps: usize) {
        let title = CString::new(format!("Snake Score: {score} FPS: {fps}"))
            .expect("Could not convert to C string");
        unsafe { SDL_SetWindowTitle(self.sdl_window, title.as_ptr()) }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyWindow(self.sdl_window);
            SDL_Quit();
        }
    }
}
