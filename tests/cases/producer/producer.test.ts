import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { getAccount, saveAccount } from "../../../utils/account.utils";
import { assert } from "chai";

import { Hydrogen } from "../../../target/types/hydrogen";

describe("h2u_contracts", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Hydrogen as Program<Hydrogen>;
  const programId = program.programId;
  const producerAuthority = anchor.web3.Keypair.generate();
  let producerPda: PublicKey;
  let eacPda: PublicKey;
  let h2Pda: PublicKey;

  before(async () => {
    await provider.connection.requestAirdrop(producerAuthority.publicKey, 4e9); // 4 SOL for testing
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

    [h2Pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("h2_canister"), producerAuthority.publicKey.toBuffer()],
      programId
    );
  });

  // before(async () => {
  //   const cachedProducer = getAccount<string>("producerPda");
  //   const cachedEac = getAccount<string>("eacPda");

  //   if (cachedProducer && cachedEac) {
  //     producerPda = new PublicKey(cachedProducer);
  //     eacPda = new PublicKey(cachedEac);
  //   } else {
  //     [producerPda] = PublicKey.findProgramAddressSync(
  //       [Buffer.from("producer"), producerAuthority.publicKey.toBuffer()],
  //       programId
  //     );
  //     [eacPda] = PublicKey.findProgramAddressSync(
  //       [Buffer.from("eac"), producerAuthority.publicKey.toBuffer()],
  //       programId
  //     );
  //     saveAccount("producerPda", producerPda.toBase58());
  //     saveAccount("eacPda", eacPda.toBase58());
  //   }
  // });

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
    try {
      await program.methods
        .updateProducerData("Updated Producer Name")
        .accounts({
          producer: producerPda,
          authority: producerAuthority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([producerAuthority])
        .rpc();
    } catch (err) {
      console.error("Transaction failed:", err);
      if (err.logs) {
        console.error("Logs:\n", err.logs.join("\n"));
      }
      throw err; // rethrow to let the test fail
    }

    const producerAccount = await program.account.producer.fetch(producerPda);
    assert.equal(producerAccount.name, "Updated Producer Name");
  });

  it("Initialize EAC Certificate", async () => {
    console.log("producerPda = ", producerPda);
    await program.methods
      .initializeEacStorage()
      .accounts({
        producer: producerPda,
        eac: eacPda,
        signer: producerAuthority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([producerAuthority])
      .rpc();

    const eacAccount = await program.account.eac.fetch(eacPda);
    assert.equal(eacAccount.availableAmount.toNumber(), 0); // fresh initialized
  });

  it("Initialize H2 Canister", async () => {
    await program.methods
      .initializeH2Canister()
      .accounts({
        h2Canister: h2Pda,
        producer: producerPda,
        signer: producerAuthority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([producerAuthority])
      .rpc();

    const h2Account = await program.account.h2Canister.fetch(h2Pda);
    assert.equal(h2Account.availableAmount.toNumber(), 0);
  });

  it("Add 1000 kWh to EAC", async () => {
    await program.methods
      .addKilowattsToEac(new anchor.BN(1000))
      .accounts({
        eac: eacPda,
        authority: producerAuthority.publicKey,
      })
      .signers([producerAuthority])
      .rpc();

    const eacAccount = await program.account.eac.fetch(eacPda);
    assert.equal(eacAccount.availableAmount.toNumber(), 1000);
    assert.equal(eacAccount.burnedAmount.toNumber(), 0);

    const h2Account = await program.account.h2Canister.fetch(h2Pda);
    assert.equal(h2Account.availableAmount.toNumber(), 0); // no register produce yest
    assert.equal(h2Account.totalAmount.toNumber(), 0);
  });

  it("Register Produce with 900 kWh", async () => {
    await program.methods
      .producerRegisterBatch(new anchor.BN(914))
      .accounts({
        producer: producerPda,
        h2Canister: h2Pda,
        eac: eacPda,
        authority: producerAuthority.publicKey,
      })
      .signers([producerAuthority])
      .rpc();

    const eacAccount = await program.account.eac.fetch(eacPda);
    assert.equal(eacAccount.availableAmount.toNumber(), 86); // 1000 - 914
    assert.equal(eacAccount.burnedAmount.toNumber(), 914);

    const h2Account = await program.account.h2Canister.fetch(h2Pda);
    assert.equal(h2Account.availableAmount.toNumber(), 15233); // (914 * 1000) / 60
    assert.equal(h2Account.totalAmount.toNumber(), 15233);
  });

  it("Fail to Register Produce with 101 kWh", async () => {
    try {
      await program.methods
        .producerRegisterBatch(new anchor.BN(101))
        .accounts({
          producer: producerPda,
          h2Canister: h2Pda,
          eac: eacPda,
          authority: producerAuthority.publicKey,
        })
        .signers([producerAuthority])
        .rpc();
      assert.fail("Should have thrown an error due to insufficient kWh");
    } catch (err) {
      // Optional: check specific error if you want
      console.log("Expected error:", err);
      const eacAccount = await program.account.eac.fetch(eacPda);
      assert.equal(eacAccount.availableAmount.toNumber(), 86); // Unchanged
      assert.equal(eacAccount.burnedAmount.toNumber(), 914); // Unchanged

      const h2Account = await program.account.h2Canister.fetch(h2Pda);
      assert.equal(h2Account.availableAmount.toNumber(), 15233); // Unchanged
      assert.equal(h2Account.totalAmount.toNumber(), 15233); // Unchanged
      //assert.include(err.error.errorMessage, "NotEnoughElectricity");
    }
  });
});
