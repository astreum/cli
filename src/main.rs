use std::{env, fs, io::Read, path::Path};
use fides::dsa::ed25519;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let arguments: Vec<String> = env::args().collect();

    let command: &str = arguments.get(1).ok_or("invalid arguments!")?;

    match command {
        "code" => {
            loop {
                print!("lisp> ");  // Prompt for input
                io::stdout().flush()?;  // Ensure the prompt is displayed immediately

                let mut code = String::new();
                io::stdin().read_line(&mut code)?;  // Read a line of input

                if code.trim() == ":quit" {  // Check for a quit command
                    break;
                }

                match evaluate_lisp(&code) {
                    Ok(result) => println!("Result: {}", result),
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
        "account" => {

            let key_path = Path::new("./key.bin");

            let secret_key = if key_path.exists() {
                let mut file = fs::File::open(key_path)?;
                let mut key_bytes = [0u8; 32];
                file.read_exact(&mut key_bytes)?;
                key_bytes
            } else {
                let new_key = ed25519::secret_key();
                fs::write(key_path, new_key)?;
                new_key
            };

            let public_key = ed25519::public_key(&secret_key)?;

            print!("0x");
            for byte in public_key {
                print!("{:X}", byte);
            }

        },
        
        _ => println!("invalid command!")
    }

    Ok(())

}
