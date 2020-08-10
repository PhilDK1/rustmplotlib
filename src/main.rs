use mplot;

fn main() {
    let x = vec![1.,2., 3.];
    let y = vec![1., 4., 9.];
    let y_2 = vec![1., 2., 3.];
    let y_3 = vec![1., 8., 27.];

    let env = mplot::Env::new();
    let plt = mplot::PyPlot::new(&env);
    
    plt.plot(&x, &y);
    plt.plot(&x, &y_2);
    plt.plot(&x, &y_3);
    plt.show().unwrap();


}

