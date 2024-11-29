mod chain;
mod blocks;
mod wallet;

use chain::Chain;
use crate::wallet::Wallet;

fn main() {
	let wallet = Wallet::new(); 
	wallet.display_info(); 

	let message = "Message to sign"; 
	let signature = wallet.sign_message(message);

	println!("Signed message: {:?}", signature); 

	let mut immaculate = Chain::new(4);

	for i in 1..=10 {
		immaculate.add_block(format!("Block #{}", i));
	}

	for block in &immaculate.blocks {
		println!("{:?}", block);
	}
}