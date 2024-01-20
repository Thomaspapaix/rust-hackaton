fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn slice_sum(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for i in slice {
        sum += i;
    }
    return sum;
}

fn print_modulo(number: i32, modulo: i32) {
    if number % modulo == 0 {
        println!("0");
    } else {
        println!("{} is not divisible by {}", number, modulo);
    }    
}

fn main() {
    println!("{}", add(1, 2));
    println!("{}", slice_sum(&[1, 2, 3]));
    print_modulo(10, 2);
}
  