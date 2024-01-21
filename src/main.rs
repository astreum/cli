use std::env;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let arguments: Vec<String> = env::args().collect();

    let topic: &str = arguments.get(1).ok_or("invalid arguments!")?;

    match topic {
        "account" => {

            let command: &str = arguments.get(2).ok_or("invalid account argument!")?;
            
            match command {
                "all" => println!("account all coming soon!"),
                "new" => println!("account new coming soon!"),
                "view" => println!("account view coming soon!"),
                _ => println!("invalid account command!")
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
