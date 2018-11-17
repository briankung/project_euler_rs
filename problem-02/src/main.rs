fn main() {
    // "Stupid is as stupid does, sir!"
    // - A wise man

    let mut current_fibs = vec![1, 2];

    // Let's count up past 4_000_000
    let max_size = 4_000_000;

    loop {
        let last = current_fibs[current_fibs.len()-1];
        let second_to_last = current_fibs[current_fibs.len()-2];
        let next_fib = last + second_to_last;

        if next_fib > max_size {
            break;
        }

        current_fibs.push(next_fib)
    }

    // Okay, next to select the even numbers and sum them:

    let sum: i32 = current_fibs.iter().filter(|&num| num % 2 == 0).sum();

    println!("{}", sum);
}
