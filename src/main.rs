use std::{env, fs, io::{self, Read, Write}, net::TcpListener, path::Path};
use fides::dsa::ed25519;
use lispeum::environment::Environment;

use crate::{api::connection::handle_connection, lispeum::special::SpecialFunctions};
pub mod lispeum;
pub mod api;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let arguments: Vec<String> = env::args().collect();

    let command: &str = arguments.get(1).ok_or("invalid arguments!")?;

    match command {
        "code" => {
            let mut lspm_env: Environment = Environment::new();
            let spcl_fns: SpecialFunctions = SpecialFunctions::new();
            
            loop {
                print!("lispeum > ");
                io::stdout().flush()?;

                let mut code = String::new();
                io::stdin().read_line(&mut code)?;

                if code.trim() == ":q" {
                    break;
                }

                let lispeum_expr = lispeum::parse_lispeum_string(&code)?;

                match lispeum::evaluator::evaluate(lispeum_expr, &mut lspm_env, &spcl_fns) {
                    Ok(result) => println!("{}", result),
                    Err(e) => println!("error: {}", e),
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
        "api" => {
            let listener = TcpListener::bind("127.0.0.1:22274").expect("Could not bind to port");
            println!("Server listening on port 22274");

            for stream in listener.incoming() {
                let stream = stream.expect("Failed to accept connection");
                handle_connection(stream);
            }
        }
        "sync" => {
            println!("test chain sync started ...");
            loop { }
        },
        _ => println!("invalid command!")

    }

    Ok(())

}
