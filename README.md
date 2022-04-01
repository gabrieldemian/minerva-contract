# About

This is the blockchain contract of a project of mine, named "Minerva". An end-to-end encrypted email service powered by Solana blockchain. <br />

I did this project mostly to learn how to implement end-to-end encryption.

# Encryption

The frontend is responsible for encrypting and decrypting data, while the blockchain is only used to store public information about the encryption, key exchange, and the email itself. <br />

I used AES-256 bit with Counter Mode.

# Key Exchange

I used diffie-helmann algorithm for the key exchange, each user get a public an private key at the registration moment. Only the public key is stored on the blockchain.

# Run the project

First of all, open `/Anchor.toml` and make sure that it is pointing to your wallet and the cluster that you want to run on. <br />

Run the following commands on your terminal, at the root of the project:

```bash
yarn
anchor build
anchor test
```

# Architecture

...
├─ src <br />
│  ├─ context.rs -> structs used on my instructions arguments <br />
│  ├─ error.rs -> error structs <br />
│  ├─ lib.rs -> my entrypoint, processor, and register modules <br />
│  ├─ state.rs -> state structs <br />
│  ├─ utils.rs -> helpers functions <br />
...
