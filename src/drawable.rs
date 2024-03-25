use crossterm::style::{ContentStyle, Stylize};

use crate::{
    canvas::Canvas,
    entities::{Bullet, Enemy, EntityStatus, Fuel, Player},
};

pub trait Drawable {
    fn draw(&self, sc: &mut Canvas);
}

impl Drawable for Enemy {
    fn draw(&self, sc: &mut Canvas) {
        match self.status {
            EntityStatus::Alive => {
                sc.draw_styled_char(self, 'E', ContentStyle::new().red().into());
            }
            EntityStatus::DeadBody => {
                sc.draw_char(self, 'X');
            }
            EntityStatus::Dead => {}
        };
    }
}

impl Drawable for Fuel {
    fn draw(&self, sc: &mut Canvas) {
        match self.status {
            EntityStatus::Alive => {
                sc.draw_styled_char(self, 'F', ContentStyle::new().green().into());
            }
            EntityStatus::DeadBody => {
                sc.draw_char(self, '$');
            }
            EntityStatus::Dead => {}
        };
    }
}

impl Drawable for Bullet {
    fn draw(&self, sc: &mut Canvas) {
        sc.draw_styled_char(self, '|', None)
            .draw_char((self.location.c, self.location.l - 1), '^');
    }
}

impl Drawable for Player {
    fn draw(&self, sc: &mut Canvas) {
        sc.draw_char(self, 'P');
    }
}
