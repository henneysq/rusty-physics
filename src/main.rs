use std::{thread, time, f64::consts::PI, collections::VecDeque};
use plotters::prelude::*;

struct Oscillator {
    // An oscillator oscillating with a frequency,
    // amplitude, and phase
    freq: f64, // [Hz]
    phase: f64, // [rad]
    amplitude: f64,
    sig: (f64, f64),

}

impl Oscillator {
    fn new(freq: f64, amplitude: f64, phase: f64) -> Oscillator {
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

    fn set_state(&mut self, t: f64) {
        // Sets a new state of the oscillator.
        // t: time in seconds
        let osc: f64 = t * self.freq * (2.0 * PI) + self.phase;
        self.sig.0 = self.amplitude * osc.sin();
        self.sig.1 = self.amplitude * osc.cos();
    }
}

fn make_sig_array(len: u32) -> VecDeque<f64> {
    let mut signal: Vec<f64> = Vec::with_capacity(len as usize);
    for _i in 0..len {
        signal.push(0.0);
    }
    let signal: VecDeque<f64> = VecDeque::from(signal);
    return signal
}

fn plot_signal(time: VecDeque<f64>, sig: VecDeque<f64>) {
    let time_sig: Vec<(f64, f64)> = time.iter().cloned().zip(sig.iter().cloned()).collect();

    let root_area = BitMapBackend::new("img/test.png", (1200, 800)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 80.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 80.0)
        .set_label_area_size(LabelAreaPosition::Right, 80.0)
        .set_label_area_size(LabelAreaPosition::Top, 80.0)
        .caption("Signal", ("sans-serif", 40.0))
        .build_cartesian_2d(time[0]..time[time.len() - 1], -4.0..4.0)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        time_sig.iter().map(|point| Circle::new(*point, 4.0_f64, &BLUE)),
    ).unwrap();
    
}

fn main() {    
    // Define some parameters for an oscillation
    let freq: f64 = 0.25; // [Hz]
    let ampl: f64 = 4.0;
    let phase: f64 = 0.0;
    let mut oscillator = Oscillator::new(
        freq, ampl, phase
    );

    // Define a data array
    let len: u64 = 50;
    let mut signal = make_sig_array(len as u32);
    println!("{:?}", signal);

    // Get a time loop going
    let dt: u64 = 100; // time resolution [ms]
    let dur = time::Duration::from_millis(dt); // Thread sleep variable
    let mut t: u64 = 0; // iteration variable
    let mut t_vec: VecDeque<f64> = make_sig_array(len as u32); // Prepare time array
    let n_iterations: u64 = 50;
    
    loop {
        // Check time at beginning 
        //let now = time::Instant::now();

        // Update the state of the oscillator
        let time: f64 = (t * dt) as f64;
        oscillator.set_state(time / 1000.0);
        
        // Compensate for computation time; then sleep dt
        //thread::sleep(dur - now.elapsed());
        
        // Add the real part of the oscillating signal to
        // the signal vector at index t.
        // If signal vector is filled out, shift the array
        // before adding new value
        if t < len {
            signal[t as usize] = oscillator.sig.0;
            t_vec[t as usize] = time;
        } else {
            signal.rotate_left(1);
            signal[(len - 1) as usize] = oscillator.sig.0;
            t_vec.rotate_left(1);
            t_vec[(len - 1) as usize] = time;
        }

        println!("Sampled signal: {:?}", signal);
        println!("Time vector: {:?}", t_vec);
        
        if t >= n_iterations {
            break;
        }
        t += 1;
    }

    plot_signal(t_vec, signal);

}