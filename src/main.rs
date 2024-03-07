use std::env;


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

            let command: &str = arguments.get(2).ok_or("invalid account argument!")?;
            
            match command {
                "all" => println!("account all coming soon!"),
                "new" => println!("account new coming soon!"),
                "view" => println!("account view coming soon!"),
                _ => println!("invalid account command!")
            }

        },
        
        _ => println!("invalid command!")
    }

    Ok(())

}
