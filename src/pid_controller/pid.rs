#![allow(dead_code)]


#[derive(Debug, Clone)]
pub struct PidControlSettings {

    pub p_gain: f32,
    pub i_gain: f32,
    pub d_gain: f32,

    pub out_max: f32,
    pub out_min: f32,

    pub max_integrator_value: f32,
}

#[derive(Debug, Clone)]
pub struct PidController {
    error: f32,
    
    p_gain: f32,
    i_gain: f32,
    d_gain: f32,

    out_max: f32,
    out_min: f32,

    previous_error : f32,
    accumulated_error : f32,
    max_integrator_value: f32,
}

impl PidController {
    pub fn new(settings : &PidControlSettings) -> Self {
        PidController{
            error: 0.0,
            previous_error: 0.0,
            accumulated_error: 0.0,

            p_gain: settings.p_gain,
            i_gain: settings.i_gain,
            d_gain: settings.d_gain,

            out_max: settings.out_max,
            out_min: settings.out_min,

            max_integrator_value: settings.max_integrator_value,
        }
    }

    fn constrain(&mut self, low : f32, value : f32, high : f32) -> f32 {
        if value < low {
            low
        } else if value > high {
            high
        } else {
            value
        }
    }

    pub fn reset(&mut self) {
        self.error = 0.0;
        self.accumulated_error = 0.0;
        self.previous_error = 0.0;
    }
    
    pub fn change_settings(&mut self, new_settings: &PidControlSettings) {
        self.p_gain = new_settings.p_gain;
        self.i_gain = new_settings.i_gain;
        self.d_gain = new_settings.d_gain;

        self.out_max = new_settings.out_max;
        self.out_min = new_settings.out_min;

        self.max_integrator_value = new_settings.max_integrator_value;
    }

    pub fn update(&mut self, target : f32, measured : f32, dt : f32) -> f32 {
        self.error = target - measured;
        
        let proportional_error = self.error * self.p_gain;
        let derivative_error = ((self.error - self.previous_error) / dt) * self.d_gain;
        self.accumulated_error = self.accumulated_error + self.i_gain * 0.5 * dt * (self.error + self.previous_error);
        self.accumulated_error = self.constrain(-self.max_integrator_value, self.accumulated_error, self.max_integrator_value);

        self.previous_error = self.error;

        self.constrain(self.out_min, proportional_error + derivative_error + self.accumulated_error , self.out_max)
    }
}