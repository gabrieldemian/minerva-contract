import { PublicKey } from "@solana/web3.js";
import { TextEncoder } from "text-encoding";
import idl from "./target/idl/minerva.json";
import { getProvider } from "@project-serum/anchor";
import { ec } from 'elliptic'

export const DEVNET_WALLET = getProvider().wallet.publicKey;

export const getUserPDA = async (
  seed: string,
  authority: PublicKey = DEVNET_WALLET
) => {
  const [PDA] = await PublicKey.findProgramAddress(
    [new TextEncoder().encode(seed), authority.toBuffer()],
    new PublicKey(idl.metadata.address)
  );
  return PDA;
};

export const elliptic = new ec('curve25519')
