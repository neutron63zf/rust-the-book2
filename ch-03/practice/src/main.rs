fn main() {
    println!("27c is {}f", degree_c2f(27.0));
    println!("100f is {}c", degree_f2c(100.0));
    println!("5th fibonacci is {}", fibonacci(5));
    twelve_days_of_christmas();
}

fn degree_c2f(c: f64) -> f64 {
    c * (9.0 / 5.0) + 32.0
}

fn degree_f2c(f: f64) -> f64 {
    (5.0 / 9.0) * (f - 32.0)
}

fn fibonacci(n: i32) -> i32 {
    if n <= 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn twelve_days_of_christmas() {
    fn print_head(day: usize) {
        let head1 = "on the ";
        let head2 = " day of christmas,";
        let head3 = "my true love sent to me";
        let counts = [
            "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
            "tenth", "eleventh", "twelfth",
        ];
        println!("{}{}{}", head1, counts[day - 1], head2);
        println!("{}", head3);
    }
    let presents = [
        "a partridge in a pear tree",
        "two turtle doves, and",
        "three french hens,",
        "four calling birds,",
        "five golden rings!",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine pipers piping,",
        "ten drummers drumming,",
        "eleven lords a-leaping,",
        "twelve ladies dancing,",
    ];
    let mut day: usize = 1;
    while day <= 12 {
        print_head(day);
        let mut presents_count = day;
        while presents_count >= 1 {
            println!("{}", presents[presents_count - 1]);
            presents_count -= 1;
        }
        println!("");
        day += 1;
    }
}
