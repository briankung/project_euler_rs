fn main() {
    // "Stupid is as stupid does, sir!"
    // - A wise man

    let mut current_fibs: Vec<i32> = vec![1, 2];

    // Let's count up past 4_000_000
    let max_size = 4_000_000 as i32;

    loop {
        let next_fib = next_fib(&current_fibs);

        if next_fib > max_size {
            break;
        }
        current_fibs.push(next_fib)
    }

    // Okay, next to select the even numbers and sum them:

    let sum = current_fibs.iter().filter(|&num| num % 2 == 0).sum::<i32>();

    println!("{:?}", sum);
}

fn next_fib(fibs: &Vec<i32>) -> i32 {
    let last = fibs.last().unwrap();
    let second_to_last = fibs.get(fibs.len() - 2).unwrap();
    last + second_to_last
}
