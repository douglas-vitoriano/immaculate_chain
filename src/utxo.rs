#[derive(Debug, Clone)]
pub struct UTXO {
	pub id: String,
	pub amount: u64,
}

#[derive(Debug)]
pub struct UTXOSet {
	pub utxos: Vec<UTXO>,
}

impl UTXOSet {
	pub fn new() -> Self {
		UTXOSet {
			utxos: Vec::new(),
		}
	}

	pub fn add_utxo(&mut self, utxo: UTXO) {
		self.utxos.push(utxo);
	}

	pub fn remove_utxo(&mut self, utxo_id: &str) {
		self.utxos.retain(|utxo| utxo.id != utxo_id);
	}

	pub fn get_utxos(&self) -> &Vec<UTXO> {
		&self.utxos
	}

	pub fn get_utxo_by_id(&self, id: &str) -> Option<&UTXO> {
		self.utxos.iter().find(|&utxo| utxo.id == id)
	}
}
