#[derive(Debug, Copy, Clone)]
pub struct VoltageQ(pub f32);
#[derive(Debug, Copy, Clone)]
pub struct VoltageD(pub f32);
#[derive(Debug, Copy, Clone)] 
pub struct VoltageLimit(pub f32);
#[derive(Debug, Copy, Clone)]
pub struct PhaseVoltage(pub f32, pub f32, pub f32);
#[derive(Debug, Copy, Clone)]
pub struct PhaseCurrents(pub f32, pub f32, pub f32);
#[derive(Debug, Copy, Clone)]
pub struct DqFrame (pub f32, pub f32);
#[derive(Debug, Copy, Clone)]
pub struct AlphaBetaFrame(pub f32, pub f32);
#[derive(Debug, Copy, Clone)]
pub struct Amperes (pub f32);
#[derive(Debug, Copy, Clone)]
pub struct Volts(pub f32);
#[derive(Debug, Copy, Clone)]
pub struct ShaftTicks(pub f32);
#[derive(Debug, Copy, Clone)]
pub struct RotorAngleRadians(pub f32);
#[derive(Debug, Copy, Clone)]
pub struct PolePairs(pub f32);
