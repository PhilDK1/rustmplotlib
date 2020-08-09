use mplot;

fn main() {
    // let ans: i32 = mplot::builtins_sum(vec![1,2, 3, 5, 9]).unwrap();
    // println!("builtins ans: {}", ans);
    // let np_ans: i32 = mplot::numpy_sum(vec![1,2, 3, 5, 9]).unwrap();
    // println!("np ans: {}", np_ans);
    // let vec: Vec<u32> = Vec::new();
    // let pyplt = mplot::PyPlot::new(&vec[..],"y".to_string());
    // let ans = pyplt.plot();
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

