use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
	pub index: u64,
	pub timestamp: u64,
	pub previous_hash: String,
	pub data: String,
	pub hash: String,
	pub nonce: u64,
}

impl Block {
	pub fn new(index: u64, previous_hash: String, data: String, difficulty: usize) -> Self {
		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("Error timestamp")
			.as_secs();

		let mut block = Block {
			index,
			timestamp,
			previous_hash,
			data,
			hash: String::new(),
			nonce: 0,
		};

		block.mine(difficulty);
		block
	}

	pub fn calculate_hash(&self) -> String {
		let data = format!(
			"{}{}{}{}{}",
			self.index, self.timestamp, self.previous_hash, self.data, self.nonce
		);
		let mut hasher = Sha256::new();
		hasher.update(data);
		format!("{:x}", hasher.finalize())
	}

	pub fn mine(&mut self, difficulty: usize) {
		let target = "0".repeat(difficulty);
		while !self.hash.starts_with(&target) {
			self.nonce += 1;
			self.hash = self.calculate_hash();
		}
		println!("Block {} mined with hash: {}", self.index, self.hash);
	}
}
