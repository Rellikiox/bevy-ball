use bevy::{prelude::*, window::PrimaryWindow};

use super::components::*;
use crate::game::common::utils;

pub fn confine_window_bound(
    mut query: Query<(&mut Transform, &WindowBound)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (mut transform, window_bound) in query.iter_mut() {
        let x_min = 0.0 + window_bound.radius;
        let y_min = 0.0 + window_bound.radius;
        let x_max = window.width() - window_bound.radius;
        let y_max = window.height() - window_bound.radius;

        transform.translation.x = utils::clamp(transform.translation.x, x_min, x_max);
        transform.translation.y = utils::clamp(transform.translation.y, y_min, y_max);
    }
}
