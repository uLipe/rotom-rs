use foc_types::types as t;

pub trait RotorPositionSensor {
    fn set_count_to_zero(&mut self);
    fn get_cpr(&self) -> t::ShaftTicks;
    fn read_counts(&self) -> t::ShaftTicks;
    fn read_rotor_position(&self) -> t::RotorAngleRadians;
}