pub trait BlockingDelay {
    fn blocking_delay_ms(&self, amount: u32);
    fn blocking_delay_us(&self, amount: u32); 
}