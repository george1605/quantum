use num_complex::Complex;
use rand::Rng;
use std::f64::consts::PI;
use std::f64::consts::FRAC_1_SQRT_2;

const SQRT_2: f64 = 1.4142135623730951;

#[derive(Debug, Copy, Clone)]
struct Qubit
{
    alpha: Complex<f64>,
    beta: Complex<f64>
}

fn z_gate(q: &Qubit) -> Qubit {
    return Qubit {
        alpha: q.alpha,
        beta: Complex::new(-1.0, 0.0) * q.beta
    }
}

fn x_gate(q: &Qubit) -> Qubit {
    return Qubit {
        alpha: q.beta,
        beta: q.alpha
    }
}

fn h_gate(q: &Qubit) -> Qubit {
    return Qubit {
        alpha: (q.beta + q.alpha) / SQRT_2,
        beta: (q.alpha - q.beta) / SQRT_2
    }
}

fn t_gate(q: &Qubit) -> Qubit {
    return Qubit {
        alpha: q.alpha,
        beta: Complex::new(0.0, PI / 4.0).exp() * q.beta
    }
}

fn compute_probabilities(wave: Vec<Complex<f64>>) -> Vec<f64> {
    let mut prob = vec![0.0_f64; wave.len()];
    for i in 0..wave.len() {
        prob[i] = wave[i].norm_sqr();
    }
    return prob;
}

fn measure(q: &Qubit) -> bool {
    let p0 = q.alpha.norm_sqr();
    let mut rng = rand::thread_rng();
    let r: f64 = rng.r#gen();
    r >= p0
}

fn measure_complex(q: Complex<f64>) -> bool {
    let p0 = q.norm_sqr();
    let mut rng = rand::thread_rng();
    let r: f64 = rng.r#gen();
    r >= p0
}

fn apply_hadamard(wavefunction: &mut Vec<Complex<f64>>, target: usize, n_qubits: usize) {
    let size = wavefunction.len();
    let mut new_state = wavefunction.clone();

    for i in 0..size {
        if ((i >> target) & 1) == 0 {
            let j = i | (1 << target);
            let a = wavefunction[i];
            let b = wavefunction[j];

            new_state[i] = (a + b) * Complex::new(FRAC_1_SQRT_2, 0.0);
            new_state[j] = (a - b) * Complex::new(FRAC_1_SQRT_2, 0.0);
        }
    }

    *wavefunction = new_state;
}

fn apply_cnot(wavefunction: &mut Vec<Complex<f64>>, control: usize, target: usize) {
    let size = wavefunction.len();
    let mut new_state = wavefunction.clone();

    for i in 0..size {
        if ((i >> control) & 1) == 1 {
            let j = i ^ (1 << target); 
            new_state[j] = wavefunction[i];
        } else {
            new_state[i] = wavefunction[i];
        }
    }

    *wavefunction = new_state;
}

fn create_bell_pair(wavefunction: &mut Vec<Complex<f64>>, q1: usize, q2: usize) {
    wavefunction[0] = Complex::new(1.0, 0.0);
    apply_hadamard(wavefunction, q1, wavefunction.len());
    apply_cnot(wavefunction, q1, q2);
}

fn cz_gate(wavefunction: &mut Vec<Complex<f64>>, q1: usize, q2: usize) {
    let n = wavefunction.len();
    for i in 1..n {
        if ((i >> q1) == 1) && ((i >> q2) == 1) {
            wavefunction[i] *= Complex::new(-1.0,0.0);
        }
    }
}

fn wavefunction_collapse(wavefunction: &mut Vec<Complex<f64>>, q: usize) {
    let s = measure_complex(wavefunction[q]) as usize;
    
}

fn swap_qubits(wavefunction: &mut Vec<Complex<f64>>, q1: usize, q2: usize) {
    let n = wavefunction.len();
    let mut new_state = wavefunction.clone();

    for i in 0..n {
        let bit1 = (i >> q1) & 1;
        let bit2 = (i >> q2) & 1;
        let swapped_index = i ^ ((bit1 ^ bit2) << q1) ^ ((bit1 ^ bit2) << q2);
        new_state[swapped_index] = wavefunction[i];
    }

    *wavefunction = new_state;
}

// Quantum Fourier Transform
// Simulated: O(n^2 * 2^n) where n = number of qubits
fn qft(wavefunction: &mut Vec<Complex<f64>>, n_qubits: usize) {
    for i in 0..n_qubits {
        apply_hadamard(wavefunction, i, n_qubits);
        for j in (i + 1)..n_qubits {
            let angle = f64::from(2) * PI / f64::from(1 << (j - i + 1) as u32);
            let phase = Complex::new(0.0, angle).exp();
            for k in 0..wavefunction.len() {
                if ((k >> i) & 1) == 1 && ((k >> j) & 1) == 1 {
                    wavefunction[k] *= phase;
                }
            }
        }
    }

    for i in 0..n_qubits / 2 {
        swap_qubits(wavefunction, i, n_qubits - i - 1);
    }
}

fn main() {
    let mut n = 3; // can be changed later
    let mut wavefunction: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); 1 << n]; // 2^n states |000> |001> etc

    create_bell_pair(&mut wavefunction, 0, 1);
    for (i, amp) in wavefunction.iter().enumerate() {
        println!("|{:02b}⟩: {:?}", i, amp);
    }

    qft(&mut wavefunction, n);
    println!("\nAfter QFT:");
    for (i, amp) in wavefunction.iter().enumerate() {
        println!("|{:02b}⟩: {:?}", i, amp);
    }
}