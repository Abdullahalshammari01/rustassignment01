fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [11, 2, 30, 4, 54, 6, 77, 8, 9, 17];

    for &num in numbers.iter() {
        let even_odd = if is_even(num) { "even" } else { "odd" };
        let fizz_buzz = match (num % 3, num % 5) {
            (0, 0) => "FizzBuzz",
            (0, _) => "Fizz",
            (_, 0) => "Buzz",
            _ => "",
        };

        println!("{}: {} {}", num, even_odd, fizz_buzz);
    }

    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("\nSum of the numbers: {}", sum);

    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("\nLargest number: {}", largest);

}
