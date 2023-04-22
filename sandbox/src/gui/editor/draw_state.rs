use bevy::prelude::{Resource, Vec2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawTransition {
    Start(Vec2),
    Draw(Vec2),
    End,
}

#[derive(Resource, Default, Debug, Clone, PartialEq)]
pub struct CanvasDrawState {
    pub prev: Option<Vec2>,
    pub current: Option<Vec2>,
}

impl CanvasDrawState {
    #[allow(unused)]
    pub fn started(&self) -> bool {
        self.current.is_some()
    }

    #[allow(unused)]
    pub fn idle(&self) -> bool {
        !self.started()
    }

    pub fn transition(&mut self, draw_event: DrawTransition) -> Option<CanvasDrawState> {
        match draw_event {
            DrawTransition::Start(v) => {
                self.current = Some(v);

                None
            }
            DrawTransition::Draw(v) => {
                self.prev = self.current;
                self.current = Some(v);
                None
            }
            DrawTransition::End => {
                let result = self.clone();
                self.prev = None;
                self.current = None;
                Some(result)
            }
        }
    }

    pub fn start(&mut self, pos: Vec2) {
        self.transition(DrawTransition::Start(pos));
    }

    pub fn draw(&mut self, pos: Vec2) {
        self.transition(DrawTransition::Draw(pos));
    }

    pub fn end(&mut self) {
        self.transition(DrawTransition::End);
    }
}
