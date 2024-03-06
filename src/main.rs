use std::{env, fs, io::Read, path::Path};
use fides::dsa::ed25519;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let arguments: Vec<String> = env::args().collect();

    let topic: &str = arguments.get(1).ok_or("invalid arguments!")?;

    match topic {
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
        "block" => {
            let command: &str = arguments.get(2).ok_or("invalid block argument!")?;
            
            match command {
                "view" => println!("block view coming soon!"),
                _ => println!("invalid block command!")
            }
        },
        "chain" => {
            let command: &str = arguments.get(2).ok_or("invalid chain argument!")?;
            
            match command {
                "sync" => println!("chain sync coming soon!"),
                "view" => println!("chain view coming soon!"),
                _ => println!("invalid chain command!")
            }
        },
        "shell" => {
            println!("shell coming soon!")
        },
        "tx" => {
            let command: &str = arguments.get(2).ok_or("invalid transaction argument!")?;
            
            match command {
                "new" => println!("transaction new coming soon!"),
                "view" => println!("transaction view coming soon!"),
                _ => println!("invalid transaction command!")
            }
        },
        _ => println!("invalid topic!")
    }

    Ok(())

}
