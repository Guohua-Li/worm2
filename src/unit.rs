use egui::{
    Vec2,
    vec2,
    Color32,
    Painter,
};

use crate::consts;

pub struct Unit {
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Default for Unit {
    fn default() -> Self {
        Self {
            position: vec2(130.0-consts::EQLENGTH, 200.0),
            velocity: vec2(0.5, 0.0),
        }
    }
}

impl Unit {
    pub fn tick(&mut self, pos_lead: Vec2) {
        let delta = pos_lead - self.position;
        let angle = delta.angle();
        let deform = vec2(
            delta.x - consts::EQLENGTH * angle.cos(),
            delta.y - consts::EQLENGTH * angle.sin()
        );
        self.velocity = consts::KS * deform + 0.1 * self.velocity;
        self.position += self.velocity;
    }

    pub fn paint(&self, painter: &Painter) {
        painter.circle( self.position.to_pos2(),  consts::RADIUS, Color32::DARK_GREEN, consts::WHITE_STROKE );
    }
}
