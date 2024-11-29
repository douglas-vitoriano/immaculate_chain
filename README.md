# Immaculate - Mini Blockchain

Immaculate is a simple blockchain implementation written in Rust, focused on learning and practicing concepts of cryptography and decentralized networks. This mini blockchain uses the `secp256k1` library for public/private key generation, message signing, and block mining through Proof-of-Work (PoW).

## Features

- **Wallet Generation**: Create a digital wallet with private key, public key, and address.
- **Message Signing**: Allows signing messages using the private key of the wallet.
- **Block Mining**: The blockchain mines blocks using Proof-of-Work.
- **Block Structure**: Each block contains information such as index, timestamp, data, previous hash, block hash, and nonce.

## How to Run Locally

### Prerequisites

Before running the project, you need to have the following installed:

- **Rust** (recommended to install the latest version)  
  If you don't have Rust installed yet, follow the official guide:  
  https://www.rust-lang.org/tools/install

### Steps to Start

1. **Clone the repository**:

   ```bash
   git clone https://github.com/your-username/immaculate.git
   cd immaculate
	 ```
2. **Build the project**:

- If it's your first time compiling the project, run the following command to compile the dependencies:
  ```bash
  cargo build --release
  ```
3. **Run the application**:

- To run the project, simply execute:
  ```bash
  cargo run
  ```
This will generate a wallet with a private key, public key, and address, sign a message, and mine some blocks.

## Code Structure
- `src/wallet.rs`: Implementation of wallet generation and message signing.
- `src/chain.rs`: Blockchain implementation, with block miners and data structure.
- `src/blocks.rs`: Definition of the block model and block data structure.
- `src/main.rs`: The entry point for running the application, generating the wallet, signing a message, and mining blocks.


## License
This project is licensed under the MIT License. See the [LICENSE](https://github.com/douglas-vitoriano/immaculate_chain/blob/main/LICENSE) file for details.
