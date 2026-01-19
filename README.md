# Hello Anchor

A simple Solana program built with the Anchor framework. This is a local Anchor version of the [Solana Playground Quick Start](https://solana.com/ko/docs/intro/quick-start) example.

## Environment

- Solana CLI: 3.0.13
- Anchor CLI: 0.32.1
- Rust: 1.89.0
- Node.js: 24.10.0

## Getting Started

### 1. Build the program

```bash
anchor build
```

### 2. Sync program ID (first time only)

```bash
anchor keys sync
```

### 3. Run tests (localnet)

```bash
anchor test
```

### 4. Deploy to devnet

```bash
solana config set --url devnet
solana airdrop 2
anchor deploy
```
