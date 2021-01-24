#![allow(dead_code)]

use foc_types::types as t;
use std::f32::consts as consts;

const B : f32  = 4.0 / consts::PI;
const C : f32  = -4.0 / (consts::PI * consts::PI);
const P : f32 = 0.225;
const _2PI : f32 = 2.0 * consts::PI;

pub const SQRT3 : f32 = 1.73205080757;
pub const FRAC_2_SQRT3 : f32 = 2.0 / SQRT3;

#[inline(always)]
fn angle_normalize(mut angle : f32) -> f32 {
    angle = angle %  _2PI;
    while angle <= -consts::PI {
         angle += _2PI;
    }
    while angle > consts::PI {
        angle -= _2PI;
    }
    angle
}
 
#[inline(always)]
pub fn convert_to_electric_angle (mech_angle : t::RotorAngleRadians, 
                                  pole_pairs : t::PolePairs) -> t::RotorAngleRadians {
    t::RotorAngleRadians(mech_angle.0 * pole_pairs.0)
}

#[inline(always)]
pub fn fast_sin(mut theta : f32) -> f32 {
    theta = angle_normalize(theta);
    let theta_y = B * theta + C * theta * match theta < 0.0 {
        true => -theta,
        false => theta 
    };

    let result = P *( theta_y * (match theta_y < 0.0 {
        true => -theta_y,
        false => theta_y,
    }) -theta_y) + theta_y;

    result
}

#[inline(always)]
pub fn fast_cos(mut theta : f32) -> f32 {
    theta = angle_normalize(theta);
    theta = match theta < 0.0 {
        true => -theta,
        false => theta
    };

    theta += consts::FRAC_PI_2;
    fast_sin(theta) 
}