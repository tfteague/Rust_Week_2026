fn main() {
    let vec: Vec<i32> = (0..5).collect();
    println!("{:?}", vec)
    numbers.iter().map(|x| x * 2)
    numbers.iter().filter(|&&x| x % 2 == 0)
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let even_squares: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).map(|x| x * x).collect();