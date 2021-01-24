use foc_types::types as t;
use foc_math_utils::math_utils as utils;

pub fn clarke_transform(currents : t::PhaseCurrents) ->  t::AlphaBetaFrame{
    let mut ab = t::AlphaBetaFrame(0.0,0.0);

    ab.0 = 0.66666666666 * currents.0 - 0.33333333333 * (currents.1 - currents.2);
    ab.1 = utils::FRAC_2_SQRT3 * (currents.1 - currents.2);
    ab
}

pub fn park_transform(ab_frame: t::AlphaBetaFrame, rotor_angle : t::RotorAngleRadians) -> t::DqFrame {
    let mut dq = t::DqFrame(0.0, 0.0);
    
    dq.0 = ab_frame.0 * utils::fast_cos(rotor_angle.0) +
           ab_frame.1 * utils::fast_sin(rotor_angle.0);
    
    dq.1 = ab_frame.1 * utils::fast_cos(rotor_angle.0) -
           ab_frame.0 * utils::fast_sin(rotor_angle.0);

    dq    
}

pub fn inverse_park_transform(dq_frame: t::DqFrame, rotor_angle : t::RotorAngleRadians) -> t::AlphaBetaFrame {
    let mut ab = t::AlphaBetaFrame(0.0,0.0);
    ab.0 = dq_frame.0 * utils::fast_cos(rotor_angle.0) -
           dq_frame.1 * utils::fast_sin(rotor_angle.0);
    
    ab.1 = dq_frame.1 * utils::fast_cos(rotor_angle.0) +
           dq_frame.0 * utils::fast_sin(rotor_angle.0);

    ab
}

pub fn inverse_clarke_transform(ab_frame: t::AlphaBetaFrame) -> t::PhaseVoltage {
    let mut p = t::PhaseVoltage(0.0,0.0,0.0);
    
    p.0 = ab_frame.0;
    p.1 = (-ab_frame.0 + utils::SQRT3 * ab_frame.1) * 0.5;
    p.2 = (-ab_frame.0 - utils::SQRT3 * ab_frame.1) * 0.5;
    p
}