import { PublicKey } from "@solana/web3.js";
import { TextEncoder } from "text-encoding";
import idl from "./target/idl/minerva.json";
import { getProvider } from "@project-serum/anchor";
import { ec } from 'elliptic'

export const DEVNET_WALLET = getProvider().wallet.publicKey;

export const seededPubkey = (pubkey: PublicKey, seed: any) => {
  return PublicKey.createWithSeed(
    pubkey,
    seed,
    new PublicKey(idl.metadata.address)
  );
};

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

export function hexStringToArrayBuffer(hexString: string) {
  hexString = hexString.replace(/^0x/, '');
  if (hexString.length % 2 != 0) {
    console.log('WARNING: expecting an even number of characters in the hexString');
  }
  var bad = hexString.match(/[G-Z\s]/i);
  if (bad) {
      console.log('WARNING: found non-hex characters', bad);
  }
  var pairs = hexString.match(/[\dA-F]{2}/gi);
  var integers = pairs.map(function(s) {
      return parseInt(s, 16);
  });
  var array = new Uint8Array(integers);
  return array.buffer;
}

export const elliptic = new ec('curve25519')
