mod util;
use util::*;

use rand::random;

const ITERATIONS: usize = 500;

const DATA_SIZE: usize = 20;

fn main() {
    let data = read_data("./data.csv");

    let mut alpha: f64 = random();
    let mut beta: f64 = random();

    // What is the appropriate name for this?
    let mut factor: f64 = 0.000001;
    let mut prev_loss = f64::INFINITY;

    for _ in 0..ITERATIONS {
        let a = alpha;
        let b = beta;
        let f = |x: f64| a*x + b;

        let data = random_slice(&data, DATA_SIZE);

        let len = data.len() as f64;

        let dloss_a =
            data.iter().map(|(x,y)|
                -2.0*x*y + 2.0*b*x + 2.0*a*x*x
                ).sum::<f64>() / len;

        let dloss_b =
            data.iter().map(|(x,y)|
                -2.0*y + 2.0*a*x + 2.0*b
                ).sum::<f64>() / len;

        // minus, since we'd like to descent
        alpha += -dloss_a*factor;
        beta  += -dloss_b*factor;
        // println!("{},{}", dloss_a, dloss_b);

        let loss: f64 = data.iter().map(|(x,y)| (y - f(*x)).powi(2)).sum::<f64>() / len;

        // decrease factor, if the loss goes up
        if prev_loss < loss {
            factor *= 0.9;
        } else {
            factor *= 1.05;
        }

        prev_loss = loss;

        println!("average loss: {}", loss);
    }

    println!("\nalpha: {}\nbeta: {}", alpha, beta);
}

fn random_slice<T>(data: &[T], len: usize) -> &[T] {
    let start = (data.len()-len) as f64 * random::<f64>();
    let start = start as usize;
    let end = start+len;

    &data[start..end]
}
