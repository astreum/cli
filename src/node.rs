use std::{error::Error, fs, io::Read, path::Path};


pub struct Node {
    pke_secret_key: [u8; 32],
    pke_public_key: [u8; 32]
}

impl Node {
    pub fn new() -> Result<Self, Box<dyn Error>> {

        let pke_path = Path::new("./pke.bin");

        let pke_secret_key = if pke_path.exists() {
            let mut pke_file = fs::File::open(pke_path)?;
            let mut pke_bytes = [0u8; 32];
            pke_file.read_exact(&mut pke_bytes)?;
            pke_bytes
        } else {
            let new_key = fides::pke::x25519::secret_key();
            fs::write(pke_path, new_key)?;
            new_key
        };

        let pke_public_key = fides::pke::x25519::public_key(&pke_secret_key);

        Ok(Node {
            pke_secret_key,
            pke_public_key
        })

    }

    pub fn start(self) -> Result<Self, Box<dyn Error>> {
        loop {
            
        }
    }
}