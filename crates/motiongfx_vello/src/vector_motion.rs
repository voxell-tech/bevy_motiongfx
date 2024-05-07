use bevy::prelude::*;

use crate::prelude::{Fill, Stroke};

#[derive(Clone)]
pub struct FillStrokeMotion<T> {
    pub id: Entity,
    pub vector: T,
    pub fill: Fill,
    pub stroke: Stroke,
    pub transform: Transform,
}

#[derive(Clone)]
pub struct FillMotion<T> {
    pub id: Entity,
    pub vector: T,
    pub fill: Fill,
    pub transform: Transform,
}

#[derive(Clone)]
pub struct StrokeMotion<T> {
    pub id: Entity,
    pub vector: T,
    pub stroke: Stroke,
    pub transform: Transform,
}
