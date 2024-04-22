use bevy::prelude::*;
use bevy_vello_renderer::vello::kurbo;

use super::VelloVector;

/// Vello Bézier path component.
#[derive(Component, Default, Clone)]
pub struct VelloBezPath {
    pub path: kurbo::BezPath,
    pub trace: f32,
}

impl VelloBezPath {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_path(mut self, path: kurbo::BezPath) -> Self {
        self.path = path;
        self
    }

    pub fn with_trace(mut self, trace: f32) -> Self {
        self.trace = trace;
        self
    }
}

impl VelloVector for VelloBezPath {
    fn shape(&self) -> impl kurbo::Shape {
        let pathels = self.path.elements();
        // TODO(perf): Prevent from creating a new BezPath for each animation change.
        let mut path = kurbo::BezPath::new();

        let pathel_count = pathels.len();
        let trace_raw = self.trace * pathel_count as f32;

        let mut most_recent_initial = kurbo::Point::new(0.0, 0.0);
        let mut most_recent_point = kurbo::Point::new(0.0, 0.0);

        for (path_index, pathel) in pathels.iter().enumerate() {
            let mut interp_value = trace_raw - path_index as f32;

            // if interp_value <= 0.0 {
            // pathels[path_index] = kurbo::PathEl::MoveTo(kurbo::Point::default());
            // } else {
            if interp_value > 0.0 {
                // Clamp value within 1.0
                interp_value = f32::min(interp_value, 1.0);

                match pathel {
                    kurbo::PathEl::MoveTo(p) => {
                        path.push(kurbo::PathEl::MoveTo(*p));

                        most_recent_initial = *p;
                        most_recent_point = *p;
                    }
                    kurbo::PathEl::LineTo(p) => {
                        path.push(interp_pathel(most_recent_point, *pathel, interp_value));

                        most_recent_point = *p;
                    }
                    kurbo::PathEl::QuadTo(_, p) => {
                        path.push(interp_pathel(most_recent_point, *pathel, interp_value));

                        most_recent_point = *p;
                    }
                    kurbo::PathEl::CurveTo(.., p) => {
                        path.push(interp_pathel(most_recent_point, *pathel, interp_value));

                        most_recent_point = *p;
                    }
                    kurbo::PathEl::ClosePath => {
                        if interp_value == 1.0 {
                            path.push(kurbo::PathEl::ClosePath);
                        } else {
                            path.push(interp_pathel(
                                most_recent_point,
                                kurbo::PathEl::MoveTo(most_recent_initial),
                                interp_value,
                            ));
                        }
                    }
                }
            }
        }

        path
    }
}

fn interp_pathel(p0: kurbo::Point, pathel: kurbo::PathEl, t: f32) -> kurbo::PathEl {
    if t == 1.0 {
        return pathel;
    }

    match pathel {
        kurbo::PathEl::MoveTo(p1) => kurbo::PathEl::MoveTo(kurbo::Point::lerp(p0, p1, t as f64)),
        kurbo::PathEl::LineTo(p1) => kurbo::PathEl::LineTo(kurbo::Point::lerp(p0, p1, t as f64)),
        kurbo::PathEl::QuadTo(p1, p2) => {
            let t = t as f64;
            // Point between p0 and p1
            let x0 = kurbo::Point::lerp(p0, p1, t);
            // Point between p1 and p2
            let x1 = kurbo::Point::lerp(p1, p2, t);
            // Point on curve
            let end_p = kurbo::Point::lerp(x0, x1, t);

            kurbo::PathEl::QuadTo(x0, end_p)
        }
        kurbo::PathEl::CurveTo(p1, p2, p3) => {
            let t = t as f64;
            // Point between p0 and p1
            let x0 = kurbo::Point::lerp(p0, p1, t);
            // Point between p1 and p2
            let x1 = kurbo::Point::lerp(p1, p2, t);
            // Point between p2 and p3
            let x2 = kurbo::Point::lerp(p2, p3, t);
            // Point between x0 and x1
            let y0 = kurbo::Point::lerp(x0, x1, t);
            // Point between x1 and x2
            let y1 = kurbo::Point::lerp(x1, x2, t);
            // Point on curve
            let end_p = kurbo::Point::lerp(y0, y1, t);

            kurbo::PathEl::CurveTo(x0, y0, end_p)
        }
        kurbo::PathEl::ClosePath => kurbo::PathEl::ClosePath,
    }
}
