fn main() {
    another_function();

    let mut n: u32 = 9;

    parameter(n);

    println!("{}", n);

    n = parameter_return(n);

    println!("{}", n);

    {
        let m = 3;

        println!("Bracket {}", m);
    }
}

fn another_function() {
    println!("Another")
}

fn parameter(mut n: u32) {
    n = n + 1;
    println!("Inside Param Function {}", n);
}

fn parameter_return(mut n: u32) -> u32 {
    n = n + 1;
    println!("Inside Param Return Function {}", n);
    n
}
