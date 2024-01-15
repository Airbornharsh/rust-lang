fn main() {
    let i: u32 = 1;

    println!("{}", i);

    let mut tuple: (u32, &str, char) = (89, "Harsh", 'A');

    tuple.0 = 90;

    println!("{}\n{}\n{}", tuple.0, tuple.1, tuple.2);
}
