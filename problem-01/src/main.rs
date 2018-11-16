fn main() {
    let numbers = 1..1000;
    let mut sum = 0;

    for number in numbers {
        if number % 3 == 0 || number % 5 == 0 {
            sum += number;
        }
    }

    println!("{:?}", sum);
}
