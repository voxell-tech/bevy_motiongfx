use bevy::prelude::*;

use crate::prelude::{Fill, Stroke};

#[macro_export]
macro_rules! build_vector {
    (
        $commands:expr,
        vector = $vector:expr,
        fill = $fill:expr,
        stroke = $stroke:expr,
        transform = $transform:expr
    ) => {
        $crate::vector_motion::FillStrokeMotion {
            id: $crate::AddVelloHandleCommandExtension::add_vello_handle(&mut $commands.spawn((
                $vector.clone(),
                $fill.clone(),
                $stroke.clone(),
                $transform,
            )))
            .id(),
            vector: $vector,
            fill: $fill,
            stroke: $stroke,
            transform: $transform,
        }
    };
    (
        $commands:expr,
        vector = $vector:expr,
        fill = $fill:expr,
        transform = $transform:expr
    ) => {
        $crate::vector_motion::FillMotion {
            id: $crate::AddVelloHandleCommandExtension::add_vello_handle(&mut $commands.spawn((
                $vector.clone(),
                $fill.clone(),
                $transform,
            )))
            .id(),
            vector: $vector,
            fill: $fill,
            transform: $transform,
        }
    };
    (
        $commands:expr,
        vector = $vector:expr,
        stroke = $stroke:expr,
        transform = $transform:expr
    ) => {
        $crate::vector_motion::StrokeMotion {
            id: $crate::AddVelloHandleCommandExtension::add_vello_handle(&mut $commands.spawn((
                $vector.clone(),
                $stroke.clone(),
                $transform,
            )))
            .id(),
            vector: $vector,
            stroke: $stroke,
            transform: $transform,
        }
    };
}
pub use build_vector;

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

#[derive(Clone)]
pub struct FillStrokeMotion<T> {
    pub id: Entity,
    pub vector: T,
    pub fill: Fill,
    pub stroke: Stroke,
    pub transform: Transform,
}
