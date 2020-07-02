extern crate mplot;

fn main() {
    let ans: i32 = mplot::builtins_sum(vec![1,2, 3, 5, 9]).unwrap();
    println!("builtins ans: {}", ans);
    let np_ans: i32 = mplot::numpy_sum(vec![1,2, 3, 5, 9]).unwrap();
    println!("np ans: {}", np_ans);
}

