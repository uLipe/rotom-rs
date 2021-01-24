
# This is a WIP project! 
# Rust, experimental, minimal vector PMSM motor controller
This is a crate that implements the platform independent part
of a vector controller for PMSM motors. Also known as FOC controller
here you find the top-level API to initialize and control the motors.

## Features:
* Platform independent, implment the I/O for your target using the traits;
* Closed-loop PID based currrent control;
* Very simple API, construct, initialize and invoke the controller periodically;
* Scalable, build Velocity and Position controls on top of this crate;
* Fast Math, suited for mainstrean 32bit microcontrollers;
* Implement the rotor_position sensor outside of the controller with sensor of your choce, just providing the functions needed by the trait.

## Limitations:
* Floating point, best suited for processors that have an FPU at least;
* No Sensorless control support;
* It needs phase-currrent sensor of each motor windings;
* Uses Sine-PWWM instead of Space-Vector PWM; 
* Please keep in mind this is a just-for-fun project
## Support:
If you would like some kind of help try reaching me here: ryukokki.felipe@gmail.com
