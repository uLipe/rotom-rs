use dq_transform::clarke_park as dq;
use foc_types::types as t; 

pub fn modulate_dq_voltages(vq : t::VoltageQ, 
                        vd : t::VoltageD,
                        rotor_position: t::RotorAngleRadians ) -> t::PhaseVoltage {

    let cmd = t::DqFrame(vd.0, vq.0);
    let ab_frame = dq::inverse_park_transform(cmd, rotor_position);
    dq::inverse_clarke_transform(ab_frame)
}

pub fn get_dq_currents(currents : t::PhaseCurrents, 
                      rotor_position: t::RotorAngleRadians) -> t::DqFrame {
    let ab_frame = dq::clarke_transform(currents);
    dq::park_transform(ab_frame, rotor_position)
}