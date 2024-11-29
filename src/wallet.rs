extern crate secp256k1;
use secp256k1::{Secp256k1, PublicKey, Message, SecretKey};
use secp256k1::ecdsa::Signature;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct Wallet {
	pub private_key: SecretKey,
	pub address: String,
	public_key: PublicKey,  
}

impl Wallet {
	pub fn new() -> Wallet {
		let secp = Secp256k1::new();
		let mut rng = OsRng;
		let mut bytes = [0u8; 32];
		
		rng.fill_bytes(&mut bytes);

		let private_key = SecretKey::from_slice(&bytes).expect("Invalid secret key slice");
		let public_key = PublicKey::from_secret_key(&secp, &private_key);

		let public_key_bytes = public_key.serialize();
		let hash = Sha256::digest(&public_key_bytes);
		let address = format!("{:x}", hash);

		Wallet {
			private_key,
			public_key,
			address,
		}
	}

    pub fn sign_message(&self, message: &str) -> Signature {
			let secp = Secp256k1::new();
			let msg = Message::from_slice(&Sha256::digest(message.as_bytes())).unwrap();
			secp.sign_ecdsa(&msg, &self.private_key)
    }

    pub fn display_info(&self) {
			println!("Private Key: {:?}", self.private_key);
			println!("Public Key: {:?}", self.public_key);
			println!("Address: {}", self.address);
    }
}
