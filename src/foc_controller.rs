use std::f32;
use std::f32::consts as math_consts;
use modulator::foc_modulator as foc;
use platform_traits::pwm_inverter as pwm;
use platform_traits::current_sensor as cs;
use platform_traits::rotor_position_sensor as encoder;
use platform_traits::blocking_delay as delay;
use foc_types::types as t;
use pid_controller::pid as pid;
use foc_math_utils::math_utils as utils;

#[derive(PartialEq)]
pub enum AlignedState {
    NotAligned,
    Aligned
}

#[derive(PartialEq)]
pub enum FocResult {
    FocNotAligned,
    FocInternalFault,
}

pub struct FocController <'a, E : encoder::RotorPositionSensor,
                           C : cs::CurrentSensor,
                           P : pwm::PWMInverter,
                           D : delay::BlockingDelay > {
    /// Commands:
    iq : t::Amperes,
    id : t::Amperes,
    
    iq_controller : pid::PidController,
    id_controller : pid::PidController, 

    ///Limits:
    supply_voltage : t::Volts,
    motor_pole_pairs : t::PolePairs,

    ///States:
    aligned : AlignedState,

    ///Timestamp:
    last_t : f32,

    ///Drivers:
    position_sensor : &'a mut  E,
    phase_current_sensor: &'a C,
    inverter :  &'a mut P,
    delay : &'a D,
}

impl <'a,E,C,P,D> FocController <'a, E,C,P,D>   
where
    E : encoder::RotorPositionSensor,
    C : cs::CurrentSensor,
    P : pwm::PWMInverter,
    D : delay::BlockingDelay

{
    pub fn new(encoder_driver : &'a mut E,  
               current_sensor_driver : & 'a C, 
               inverter_driver : & 'a mut P,
               delay_driver : & 'a D,
               voltage_supply : t::Volts,
               pole_pairs : t::PolePairs) -> Self {
        
        let i_settings  = pid::PidControlSettings{
            p_gain: 50.0,
            i_gain: 0.05,
            d_gain: 0.0,
         
            out_max: voltage_supply.0, 
            out_min: -voltage_supply.0,
        
            max_integrator_value: 40.0,      
        };

        FocController  {
            id : t::Amperes(0.0),
            iq : t::Amperes(0.0),
            supply_voltage : voltage_supply,
            position_sensor : encoder_driver,
            phase_current_sensor : current_sensor_driver,
            inverter : inverter_driver,
            delay : delay_driver,
            aligned : AlignedState::NotAligned,
            iq_controller : pid::PidController::new(&i_settings),
            id_controller : pid::PidController::new(&i_settings),
            last_t : 0.0,
            motor_pole_pairs : pole_pairs,
        }

    }
    
    pub fn initialize_foc(&mut self, initial_time : f32) -> Result<(), FocResult>{
        
        let align_command_q = t::VoltageQ(self.supply_voltage.0 * 0.1);
        let align_command_d = t::VoltageD(0.0);


        let zeroq_voltage = t::VoltageQ(0.0);
        let zerod_voltage = t::VoltageD(0.0);
        let mut phase_voltages = foc::modulate_dq_voltages(zeroq_voltage, 
            zerod_voltage, 
            t::RotorAngleRadians(0.0));
        self.inverter.set_inverter_voltages(phase_voltages, self.supply_voltage);
        self.delay.blocking_delay_ms(1000);


        for i in 0 .. 5 {
            let align_angle = math_consts::FRAC_PI_2 - ((2.0 * math_consts::PI) * i as f32 ) / 6.0;
            let phase_voltages = foc::modulate_dq_voltages(align_command_q, 
                align_command_d, 
            utils::convert_to_electric_angle(
                t::RotorAngleRadians(align_angle), 
                self.motor_pole_pairs));
            self.inverter.set_inverter_voltages(phase_voltages, self.supply_voltage);
            self.delay.blocking_delay_ms(250);
        } 

        for i in 5 .. 0 {
            let align_angle = math_consts::FRAC_PI_2 - ((2.0 * math_consts::PI) * i as f32 ) / 6.0;
            let phase_voltages = foc::modulate_dq_voltages(align_command_q, 
                align_command_d, 
            utils::convert_to_electric_angle(
                t::RotorAngleRadians(align_angle), 
                self.motor_pole_pairs));

            self.inverter.set_inverter_voltages(phase_voltages, self.supply_voltage);
            self.delay.blocking_delay_ms(150);
        }

        self.delay.blocking_delay_ms(2000);
        self.position_sensor.set_count_to_zero();
        self.delay.blocking_delay_ms(100);

        phase_voltages = foc::modulate_dq_voltages(zeroq_voltage, 
            zerod_voltage, 
            t::RotorAngleRadians(0.0));
        self.inverter.set_inverter_voltages(phase_voltages, self.supply_voltage);
        self.delay.blocking_delay_ms(100);

        self.aligned = AlignedState::Aligned;
        self.last_t = initial_time;
        Ok(())
    }

    pub fn set_target_current(&mut self, current : t::Amperes) -> Result<(), FocResult> {
        if self.aligned != AlignedState::Aligned {
            Err(FocResult::FocNotAligned)
        } else {
            self.iq = current;
            Ok(())
        }
    }

    pub fn get_rotor_position_ticks(&mut self) -> t::ShaftTicks {
        self.position_sensor.read_counts()
    }

    pub fn get_rotor_sensor_cpr(&mut self) -> t::ShaftTicks {
        self.position_sensor.get_cpr()
    }

    pub fn execute_foc_loop(&mut self, now : f32) -> Result<(), FocResult>{

        if self.aligned != AlignedState::Aligned {
            Err(FocResult::FocNotAligned)
        } else {
            let ia = self.phase_current_sensor.read_phase_current(cs::Phases::PhaseA);
            let ib = self.phase_current_sensor.read_phase_current(cs::Phases::PhaseB);
            let ic = self.phase_current_sensor.read_phase_current(cs::Phases::PhaseC);
            let phase_currents = t::PhaseCurrents(ia.0, ib.0, ic.0);
            let rotor_angle = self.position_sensor.read_rotor_position();

            let dq_state = foc::get_dq_currents(phase_currents, 
                                    utils::convert_to_electric_angle(
                                        rotor_angle, 
                                        self.motor_pole_pairs)
                                    );

            let vq_command = self.iq_controller.update(self.iq.0, dq_state.1, now - self.last_t);            
            let vd_command = self.id_controller.update(self.id.0, dq_state.0, now - self.last_t);
            
            self.last_t = now;

            let phase_voltages = foc::modulate_dq_voltages(t::VoltageQ(vq_command), 
                                                        t::VoltageD(vd_command), 
                                                        utils::convert_to_electric_angle(
                                                            rotor_angle, 
                                                            self.motor_pole_pairs));
            
            self.inverter.set_inverter_voltages(phase_voltages, self.supply_voltage);
            Ok(())
        }
    }
}
