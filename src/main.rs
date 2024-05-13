// Prints 'Hello, {input from the cli}!' to the terminal
fn main() {
    let mut hold = String::new();
    println!("Please enter who you wanna holla at: ");
    std::io::stdin().read_line(&mut hold).unwrap();
    
    let name = hold.trim();

    println!("Hello, {}!", name);
}
