use num_complex::Complex;

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

fn main() {
    let x = 10;
    println!("{}", x);
}