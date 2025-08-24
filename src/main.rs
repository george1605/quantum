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

fn apply_cnot(wavefunction: &mut Vec<Complex<f64>>, control: usize, target: usize, n_qubits: usize) {
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
    apply_cnot(wavefunction, q1, q2, wavefunction.len());
}

fn main() {
    let mut n = 3; // can be changed later
    let mut wavefunction: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); 1 << n]; // 2^n states |000> |001> etc

    create_bell_pair(&mut wavefunction, 0, 1);
    for (i, amp) in wavefunction.iter().enumerate() {
        println!("|{:02b}⟩: {:?}", i, amp);
    }
}