import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram, Keypair } from "@solana/web3.js";
import { assert } from "chai";

import { Hydrogen } from "../../../target/types/hydrogen";

describe("h2u_contracts", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Hydrogen as Program<Hydrogen>;
  const programId = program.programId;
  const signer = provider.wallet;
  const producerAuthority = anchor.web3.Keypair.generate();
  //const producerKeypair = anchor.web3.Keypair.generate();
  let producerPda: PublicKey;
  let bumpProducer: number;
  let eacPda: PublicKey;
  let bumpEac: number;

  before(async () => {
    await provider.connection.requestAirdrop(producerAuthority.publicKey, 4e9); // 4 SOL for testing
    console.log("producer = ", producerAuthority);
    await new Promise((resolve) => setTimeout(resolve, 1000)); // wait

    [producerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("producer"), producerAuthority.publicKey.toBuffer()],
      programId
    );
    // console.log("Signer public key:", signer.publicKey.toBase58());
    // console.log("producerPda = ", producerPda);
    [eacPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("eac"), producerAuthority.publicKey.toBuffer()],
      programId
    );
  });

  it("Initialize Producer", async () => {
    await program.methods
      .initializeProducer(new anchor.BN(1), "Test Producer")
      .accounts({
        producer: producerPda,
        authority: producerAuthority.publicKey,
        systemProgram: SystemProgram.programId, // correct
      })
      .signers([producerAuthority])
      .rpc();

    const producerAccount = await program.account.producer.fetch(producerPda);
    assert.equal(producerAccount.name, "Test Producer");
  });

  it("Update Producer Name", async () => {
    await program.methods
      .updateProducerData("Updated Producer Name")
      .accounts({
        producer: producerPda,
      })
      .rpc();

    const producerAccount = await program.account.producer.fetch(producerPda);
    assert.equal(producerAccount.name, "Updated Producer Name");
  });

  it("Initialize EAC Certificate", async () => {
    await program.methods
      .initializeEacStorage()
      .accounts({
        producer: producerPda,
        eac: eacPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([])
      .rpc();

    const eacAccount = await program.account.eac.fetch(eacPda);
    assert.equal(eacAccount.availableAmount.toNumber(), 0); // fresh initialized
  });

  it("Add 1000 kWh to EAC", async () => {
    await program.methods
      .addKilowattsToEac(new anchor.BN(1000))
      .accounts({
        eac: eacPda,
      })
      .rpc();

    const eacAccount = await program.account.eac.fetch(eacPda);
    assert.equal(eacAccount.availableAmount.toNumber(), 1000);
  });

  it("Register Produce with 900 kWh", async () => {
    await program.methods
      .producerRegisterBatch(new anchor.BN(900))
      .accounts({
        producer: producerPda,
        eac: eacPda,
      })
      .rpc();

    const eacAccount = await program.account.eac.fetch(eacPda);
    assert.equal(eacAccount.availableAmount.toNumber(), 100); // 1000 - 900
  });

  it("Fail to Register Produce with 101 kWh", async () => {
    try {
      await program.methods
        .producerRegisterBatch(new anchor.BN(101))
        .accounts({
          producer: producerPda,
          eac: eacPda,
        })
        .rpc();
      assert.fail("Should have thrown an error due to insufficient kWh");
    } catch (err) {
      // Optional: check specific error if you want
      console.log("Expected error:", err.error.errorMessage);
      assert.include(err.error.errorMessage, "NotEnoughElectricity");
    }
  });
});
