use foc_types::types as t;

pub enum Phases {
    PhaseA,
    PhaseB,
    PhaseC
}

pub trait CurrentSensor {
    fn get_max_current(&self) -> t::Amperes;
    fn read_phase_current(&self, channel : Phases) -> t::Amperes;
}