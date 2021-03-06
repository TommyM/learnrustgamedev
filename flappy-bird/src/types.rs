use ggez::{audio, graphics};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct DeltaTime {
    pub delta: f32,
}

#[derive(Clone, Debug)]
pub struct Images {
    pub background: graphics::Image,
    pub bird: graphics::Image,
    pub ground: graphics::Image,
    pub pipe: graphics::Image,
}

#[derive(Debug)]
pub struct Sounds {
    pub explosion: audio::Source,
    pub hurt: audio::Source,
    pub jump: audio::Source,
    pub score: audio::Source,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum FontType {
    Retro,
    Flappy,
}

pub type Fonts = HashMap<FontType, graphics::Font>;

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Alignment {
    Left,
    Right,
    Centered,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PipeSide {
    Top,
    Bottom,
}
