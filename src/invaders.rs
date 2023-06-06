use rusty_time::Timer;
use std::cmp::max;
use std::time::Duration;

use crate::{frame::Drawable, NUM_COLS, NUM_ROWS};

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if (x > 1)
                    && (x < NUM_COLS - 2)
                    && (y > 0)
                    && (y < 9)
                    && (x % 2 == 0)
                    && (y % 2 == 0)
                {
                    // build an army.
                    army.push(Invader { x, y });
                }
            }
        }

        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }

    // This function will be longest function in the project probably.
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);

        // what the hell is the move_timer thing? I must investigate about that.
        if self.move_timer.ready {
            // TODO Why are we resetting timer? Think about that.
            self.move_timer.reset();

            // if invaders reaches to a side (left or right) then move down them.
            // Every moving downward we will increase their speed.
            let mut downwards = false;

            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);

                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);

                // This means we reached our right side
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }

            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);

                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    // TODO What the hell is going on here? Think about that.
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
        }

        false
    }

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }

    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        // find the invader index number which has same x and y coordinates
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y))
        {
            // Remove that index from army.
            self.army.remove(idx);

            // For playing a sound we must return a boolean.
            true
        } else {
            false
        }
    }
}

impl Default for Invaders {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32())
                > 0.5
            {
                "x"
            } else {
                "+"
            };
        }
    }
}
