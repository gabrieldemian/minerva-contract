import {
  Program,
  workspace,
  Provider,
  setProvider,
} from "@project-serum/anchor";
import AES from 'crypto-js/aes'
import { enc, mode, lib } from 'crypto-js'
import { Keypair, SystemProgram } from "@solana/web3.js";
import { Minerva } from "../target/types/minerva";
import { DEVNET_WALLET, getUserPDA, elliptic } from "../utils";
import { expect } from "chai";

describe("minerva tests", () => {
  setProvider(Provider.env());

  /* generating diffie helmann keys */
  const aliceKeypair = elliptic.genKeyPair()
  const bobKeypair = elliptic.genKeyPair()

  const aliceDiffiePublic = aliceKeypair.getPublic().encode("hex", true)
  const bobDiffiePublic = bobKeypair.getPublic().encode("hex", true)

  const sharedSecret = aliceKeypair.derive(bobKeypair.getPublic()).toString("hex")

  /* generating blockchain wallets */
  const alice = DEVNET_WALLET;
  const bob = Keypair.generate();

  const program = workspace.Minerva as Program<Minerva>;

  it("can register both users and update the account", async () => {
    const aliceAccountPDA = await getUserPDA("user-account");
    const bobAccountPDA = await getUserPDA("user-account", bob.publicKey);

    const airdropTx = await program.provider.connection.requestAirdrop(
      bob.publicKey,
      1000000000
    );

    await program.provider.connection.confirmTransaction(airdropTx);

    await program.rpc.register(aliceDiffiePublic, {
      accounts: {
        authority: alice,
        userAccount: aliceAccountPDA,
        systemProgram: SystemProgram.programId,
      },
    });

    await program.rpc.register(bobDiffiePublic, {
      accounts: {
        authority: bob.publicKey,
        userAccount: bobAccountPDA,
        systemProgram: SystemProgram.programId,
      },
      signers: [bob]
    });

    const users = await program.account.userAccount.all();

    console.log("users: ", users);
    expect(users.length).to.equal(2);
  });

  it("can encrypt emails, send the emails, and decrypt it", async () => {
    const mailA = Keypair.generate();

    let cipher = AES.encrypt("simplesmente intankavel o bostil", sharedSecret, { mode: mode.CTR })

    await program.rpc.sendEmail(
      "very important subject", // subject
      cipher.ciphertext.toString(), // body of email
      alice, // from
      bob.publicKey, // to
      cipher.salt.toString(), // salt
      cipher.iv.toString(), // iv
      {
        accounts: {
          authority: alice,
          mail: mailA.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [mailA],
      }
    );

    const emails = await program.account.mail.all();

    const email = emails[0].account

    const plaintext = AES.decrypt(
      {
        ciphertext: enc.Hex.parse(email.body),
        iv: enc.Hex.parse(email.iv),
        salt: enc.Hex.parse(email.salt)
      } as lib.CipherParams,
      sharedSecret,
      { mode: mode.CTR }
    )

    console.log("\n");
    console.log("emails: ", emails);
    console.log("\n");
    console.log('plaintext: ', plaintext.toString(enc.Utf8))
    console.log("shared_secret: ", sharedSecret);
    console.log("cyphertext: ", emails[0].account.body);
    console.log("\n");

    expect(plaintext.toString(enc.Utf8)).to.equal('simplesmente intankavel o bostil');
  });

});
