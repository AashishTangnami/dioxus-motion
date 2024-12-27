use crate::{motion::Motion, prelude::AnimationMode, use_motion, UseMotion};

#[derive(Clone, Copy)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub rotate: f32,
    pub opacity: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale: 1.0,
            rotate: 0.0,
            opacity: 1.0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct TransformMotion {
    x: UseMotion,
    y: UseMotion,
    scale: UseMotion,
    rotate: UseMotion,
    opacity: UseMotion,
}
impl TransformMotion {
    fn new(initial: Transform, target: Transform, config: AnimationMode) -> Self {
        TransformMotion {
            x: use_motion(Motion::new(initial.x).to(target.x).mode(config)),
            y: use_motion(Motion::new(initial.y).to(target.y).mode(config)),
            scale: use_motion(Motion::new(initial.scale).to(target.scale).mode(config)),
            rotate: use_motion(Motion::new(initial.rotate).to(target.rotate).mode(config)),
            opacity: use_motion(Motion::new(initial.opacity).to(target.opacity).mode(config)),
        }
    }

    pub fn x(&self) -> f32 {
        self.x.value()
    }
    pub fn y(&self) -> f32 {
        self.y.value()
    }
    pub fn scale(&self) -> f32 {
        self.scale.value()
    }
    pub fn rotate(&self) -> f32 {
        self.rotate.value()
    }
    pub fn opacity(&self) -> f32 {
        self.opacity.value()
    }

    pub fn style(&self) -> String {
        format!(
            "transform: translate({}px, {}px) scale({}) rotate({}deg); opacity: {}",
            self.x(),
            self.y(),
            self.scale(),
            self.rotate(),
            self.opacity()
        )
    }

    pub fn start(&mut self) {
        self.x.start();
        self.y.start();
        self.scale.start();
        self.rotate.start();
        self.opacity.start();
    }

    pub fn stop(&mut self) {
        self.x.stop();
        self.y.stop();
        self.scale.stop();
        self.rotate.stop();
        self.opacity.stop();
    }

    pub fn resume(&mut self) {
        self.x.resume();
        self.y.resume();
        self.scale.resume();
        self.rotate.resume();
        self.opacity.resume();
    }

    pub fn reset(&mut self) {
        self.x.reset();
        self.y.reset();
        self.scale.reset();
        self.rotate.reset();
        self.opacity.reset();
    }

    pub fn reverse(&mut self) {
        self.x.reverse();
        self.y.reverse();
        self.scale.reverse();
        self.rotate.reverse();
        self.opacity.reverse();
    }
}

pub fn use_transform_animation(
    initial: Transform,
    target: Transform,
    config: AnimationMode,
) -> TransformMotion {
    TransformMotion::new(initial, target, config)
}
