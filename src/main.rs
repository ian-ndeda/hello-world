// Prints 'Hello, {input from the cli}!' to the terminal
fn main() {
    println!("Please enter who you wanna holla at: ");
    
    let mut hold = String::new();
    std::io::stdin().read_line(&mut hold).unwrap();
    
    let name = hold.trim();

    println!("Hello, {}!", name);
}
