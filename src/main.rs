use mplot;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut x_rng: Vec<f64> = Vec::with_capacity(30);
    let mut y_rng: Vec<f64> = Vec::with_capacity(30);
    for _i in 0..30 {
        x_rng.push(rng.gen());
        y_rng.push(rng.gen());
    }
    let x = vec![1.,2., 3.];
    let y = vec![1., 4., 9.];
    let y_2 = vec![1., 2., 3.];
    let y_3 = vec![1., 8., 27.];

    let env = mplot::Env::new();
    let plt = mplot::PyPlot::new(&env);
    
    plt.scatter(&x_rng, &y_rng);

    // plt.plot(&x, &y);
    // plt.plot(&x, &y_2);
    // plt.plot(&x, &y_3);
    plt.show().unwrap();


}

