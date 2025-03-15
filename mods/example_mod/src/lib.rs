mod_macros::create_mod!("../../wit/module.wit");

use std::cell::RefCell;

pub struct Main {
    position: RefCell<(f32, f32)>,
    reverse: RefCell<(bool, bool)>,
    ghosts: RefCell<Vec<(f32, f32)>>,
    since_last_ghost: RefCell<u32>,
    size: (f32, f32),
    ghost_amount: u32,
    ghost_interval: u32,
}

impl GuestMain for Main {
    fn new() -> Self {
        Main {
            position: RefCell::new((0.0, 0.0)),
            reverse: RefCell::new((false, false)),
            ghosts: RefCell::new(Vec::new()),
            since_last_ghost: RefCell::new(0),
            size: (80.0, 60.0),
            ghost_amount: 10,
            ghost_interval: 5,
        }
    }

    fn init(&self) {}

    fn update(&self, _: f32) {
        let mut position = self.position.borrow_mut();
        let mut reverse = self.reverse.borrow_mut();
        let mut since_last_ghost = self.since_last_ghost.borrow_mut();
        let ghost_amount = self.ghost_amount;
        let ghost_interval = self.ghost_interval;
        let size = self.size;
        let window_size = get_window_size();

        position.0 += if reverse.0 { -1.0 } else { 1.0 };
        position.1 += if reverse.1 { -1.0 } else { 1.0 };

        if position.0 > window_size.0 - size.0 {
            reverse.0 = true;
            position.0 = window_size.0 - size.0;
        } else if position.0 <= 0.0 {
            reverse.0 = false;
            position.0 = 0.0;
        }

        if position.1 > window_size.1 - size.1 {
            reverse.1 = true;
            position.1 = window_size.1 - size.1;
        } else if position.1 <= 0.0 {
            reverse.1 = false;
            position.1 = 0.0;
        }

        if *since_last_ghost >= ghost_interval {
            let mut ghosts = self.ghosts.borrow_mut();
            ghosts.push((position.0, position.1));
            if ghosts.len() > ghost_amount as usize {
                ghosts.remove(0);
            }
            *since_last_ghost = 0;
        } else {
            *since_last_ghost += 1;
        }
    }

    fn draw(&self) {
        let position = self.position.borrow();
        let ghosts = self.ghosts.borrow();
        let since_last_ghost = *self.since_last_ghost.borrow();
        let ghost_interval = self.ghost_interval;
        let ghost_amount = self.ghost_amount;
        let size = self.size;
        let window_size = get_window_size();

        color(
            position.0 / window_size.0,
            position.1 / window_size.1,
            1.0 - position.0 / window_size.0,
            1.0,
        );
        draw_rect(position.0, position.1, size.0, size.1);

        let ghosts_len = ghosts.len() as f32;
        for (i, ghost) in ghosts.iter().enumerate() {
            color(
                ghost.0 / window_size.0,
                ghost.1 / window_size.1,
                1.0 - ghost.0 / window_size.0,
                ((1.0 / ghosts_len) * i as f32)
                    - ((since_last_ghost as f32) / ghost_interval as f32 / ghost_amount as f32),
            );
            draw_rect(ghost.0, ghost.1, size.0, size.1);
        }
    }

    fn shutdown(&self) {}
}
