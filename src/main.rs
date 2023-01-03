use std::{thread, time, f32::consts::PI, collections::VecDeque};

struct Oscillator {
    // An oscillator oscillating with a frequency,
    // amplitude, and phase
    freq: f32, // [Hz]
    phase: f32, // [rad]
    amplitude: f32,
    sig: (f32, f32),

}

impl Oscillator {
    fn new(freq: f32, amplitude: f32, phase: f32) -> Oscillator {
        // Create new oscillator
        //
        // freq: oscillation frequency [Hz]
        // phase: initial oscillation phase [radians]
        Oscillator { 
            freq, 
            amplitude, 
            phase, 
            sig: (0.0, 0.0) }
    }

    fn set_state(&mut self, t: f32) {
        // Sets a new state of the oscillator.
        // t: time in seconds
        let osc: f32 = t * self.freq * (2.0 * PI) + self.phase;
        self.sig.0 = self.amplitude * osc.sin();
        self.sig.1 = self.amplitude * osc.cos();
    }
}

fn main() {    
    // Define some parameters for an oscillation
    let freq: f32 = 0.25; // [Hz]
    let ampl: f32 = 2.0;
    let phase: f32 = 0.0;
    let mut oscillator = Oscillator::new(
        freq, ampl, phase
    );

    // Get a time loop going
    let dt: u64 = 100; // ms
    let dur = time::Duration::from_millis(dt);
    let mut t: u64 = 0;
    
    loop {
        // Check time at beginning 
        let now = time::Instant::now();

        // Update the state of the oscillator
        let time: f32 = (t * dt) as f32;
        oscillator.set_state(time / 1000.0);

        let deb_amplitude: f32 = (oscillator.sig.0.powf(2.0) + 
            oscillator.sig.1.powf(2.0)).sqrt();
        
        // println!("{} seconds passed", time / 1000.0);
        println!("Complex signal: {}, {}, - Amplitude: {}", 
            oscillator.sig.0, oscillator.sig.1, deb_amplitude);
        
        // Compensate for computation time; then sleep
        thread::sleep(dur - now.elapsed());
        
        if t >= 10 {
            break;
        }
        t += 1;
    }
}