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

fn make_sig_array(len: u32) -> VecDeque<f32> {
    let mut signal: Vec<f32> = Vec::with_capacity(len as usize);
    for _i in 0..len {
        signal.push(0.0);
    }
    let signal: VecDeque<f32> = VecDeque::from(signal);
    return signal
}

fn main() {    
    // Define some parameters for an oscillation
    let freq: f32 = 0.25; // [Hz]
    let ampl: f32 = 2.0;
    let phase: f32 = 0.0;
    let mut oscillator = Oscillator::new(
        freq, ampl, phase
    );

    // Define a data array
    let len: u64 = 10;
    let mut signal = make_sig_array(len as u32);
    println!("{:?}", signal);

    // Get a time loop going
    let dt: u64 = 100; // time resolution [ms]
    let dur = time::Duration::from_millis(dt); // Thread sleep variable
    let mut t: u64 = 0; // iteration variable
    let n_iterations: u64 = 40;
    
    loop {
        // Check time at beginning 
        let now = time::Instant::now();

        // Update the state of the oscillator
        let time: f32 = (t * dt) as f32;
        oscillator.set_state(time / 1000.0);
        
        // Compensate for computation time; then sleep dt
        thread::sleep(dur - now.elapsed());
        
        // Add the real part of the oscillating signal to
        // the signal vector at index t.
        // If signal vector is filled out, shift the array
        // before adding new value
        if t < len {
            signal[t as usize] = oscillator.sig.0;
        } else {
            signal.rotate_left(1);
            signal[(len - 1) as usize] = oscillator.sig.0;
        }

        println!("Sampled signal: {:?}", signal);
        
        if t >= n_iterations {
            break;
        }
        t += 1;
    }
}