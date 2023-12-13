use std::io;

pub fn banner() -> (bool, String, String) {
    loop {
        println!("Chose to [e]crypt/[d]crypt:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        let encrypt =  match input.chars().next().unwrap() {
            'e' => true,
            'd' => false,
            _ => {
                println!("Invalid choice");
                continue;
            }
        };

        println!("Enter the text to encrypt/decrypt:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("Enter the key to encrypt/decrypt:");
        let mut key = String::new();
        io::stdin()
            .read_line(&mut key)
            .expect("Failed to read line");

        return (encrypt, input, key);
    }
}


