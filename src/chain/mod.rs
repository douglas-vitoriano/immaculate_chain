
use crate::blocks::block::Block;

#[derive(Debug)]
pub struct Chain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Chain {
	pub fn new(difficulty: usize) -> Self {
		let genesis_block = Block::new(0, String::from("0"), String::from("Genesis Block"), difficulty);
		Chain {
			blocks: vec![genesis_block],
			difficulty,
		}
	}

	pub fn add_block(&mut self, data: String) {
		let previous_block = self.blocks.last().unwrap();
		let new_block = Block::new(
			previous_block.index + 1,
			previous_block.hash.clone(),
			data,
			self.difficulty,
		);
		self.blocks.push(new_block);
	}
}
