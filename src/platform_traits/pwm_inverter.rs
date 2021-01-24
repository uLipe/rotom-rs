use foc_types::types as t;

pub trait PWMInverter {
    fn get_supply_voltage(&self) -> t::Volts;
    fn set_inverter_voltages(&mut self, phases_voltage : t::PhaseVoltage,
                              supply_voltage : t::Volts);
}