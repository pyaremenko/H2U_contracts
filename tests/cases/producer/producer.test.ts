import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";
import { Hydrogen } from "../../../target/types/hydrogen";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createMint } from "@solana/spl-token";
import { getAssociatedTokenAddress, getAccount } from "@solana/spl-token";

const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

describe("h2u_contracts", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Hydrogen as Program<Hydrogen>;
  const programId = program.programId;
  const producerAuthority = anchor.web3.Keypair.generate();

  let producerPda: PublicKey;
  let eacPda: PublicKey;
  let h2Pda: PublicKey;

  before(async () => {
    console.log("🔄 Airdropping 4 SOL to producer authority...");
    await provider.connection.requestAirdrop(producerAuthority.publicKey, 4e9);
    await new Promise((resolve) => setTimeout(resolve, 1000));

    [producerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("producer"), producerAuthority.publicKey.toBuffer()],
      programId
    );

    [eacPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("eac"), producerPda.toBuffer()],
      programId
    );

    [h2Pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("h2_canister"), producerAuthority.publicKey.toBuffer()],
      programId
    );

    console.log("📌 producerPda:", producerPda.toBase58());
    console.log("📌 eacPda:", eacPda.toBase58());
    console.log("📌 h2Pda:", h2Pda.toBase58());
  });

  it("Initialize Producer", async () => {
    const tx = await program.methods
      .initializeProducer(new anchor.BN(1), "Test Producer")
      .accounts({
        producer: producerPda,
        authority: producerAuthority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([producerAuthority])
      .rpc();

    console.log("✅ initializeProducer tx signature:", tx);

    const producerAccount = await program.account.producer.fetch(producerPda);
    console.log("📄 Producer Account:", {
      ...producerAccount,
      id: producerAccount.id.toNumber(),
      authority: producerAccount.authority.toBase58(),
    });

    assert.equal(producerAccount.name, "Test Producer");
  });

  it("Update Producer Name", async () => {
    const tx = await program.methods
      .updateProducerData("Updated Producer Name")
      .accounts({
        producer: producerPda,
        authority: producerAuthority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([producerAuthority])
      .rpc();

    console.log("✅ updateProducerData tx signature:", tx);

    const producerAccount = await program.account.producer.fetch(producerPda);
    console.log("📄 Updated Producer Account:", {
      ...producerAccount,
      id: producerAccount.id.toNumber(),
      authority: producerAccount.authority.toBase58(),
    });

    assert.equal(producerAccount.name, "Updated Producer Name");
  });

  it("Initializes EAC storage and creates metadata", async () => {
    const tokenMint = await createMint(
      provider.connection,
      producerAuthority,
      producerAuthority.publicKey,
      producerAuthority.publicKey,
      9
    );

    const [metadataPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        tokenMint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    const [producerAta] = PublicKey.findProgramAddressSync(
      [
        producerAuthority.publicKey.toBuffer(),
        TOKEN_PROGRAM_ID.toBuffer(),
        tokenMint.toBuffer(),
      ],
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const [eacPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("eac"), producerPda.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .initializeEacStorage(
        "EAC Certificate",
        "EAC",
        "https://example.com/metadata.json",
        new anchor.BN(1000)
      )
      .accounts({
        eac: eacPda,
        tokenMint,
        metadataAccount: metadataPda,
        producerAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        producer: producerPda,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        signer: producerAuthority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([producerAuthority])
      .rpc();

    console.log("✅ initEacStorage tx:", tx);

    const eacAccount = await program.account.eac.fetch(eacPda);
    console.log("📄 EAC Account:", {
      certificateCapacityKwts: eacAccount.certificateCapacityKwts.toNumber(),
      availableKwts: eacAccount.availableKwts.toNumber(),
      burnedKwts: eacAccount.burnedKwts.toNumber(),
      producerPubkey: eacAccount.producerPubkey.toBase58(),
      tokenMint: eacAccount.tokenMint.toBase58(),
    });
  });

  it("Initialize H2 Canister", async () => {
    const tokenMint = await createMint(
      provider.connection,
      producerAuthority,
      producerAuthority.publicKey,
      producerAuthority.publicKey,
      9
    );

    const [metadataPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        tokenMint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    const producerAta = await getAssociatedTokenAddress(
      tokenMint,
      producerAuthority.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const tx = await program.methods
      .initializeH2Canister(
        "HydrogenCredit",
        "H2U",
        "https://example.com/metadata.json"
      )
      .accounts({
        h2Canister: h2Pda,
        tokenMint,
        metadataAccount: metadataPda,
        producerAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        producer: producerPda,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        signer: producerAuthority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([producerAuthority])
      .rpc();

    console.log("✅ initializeH2Canister tx signature:", tx);

    const h2Account = await program.account.h2Canister.fetch(h2Pda);
    console.log("📄 H2 Canister Account:", {
      totalAmount: h2Account.totalAmount.toNumber(),
      availableHydrogen: h2Account.availableHydrogen.toNumber(),
      producerPubkey: h2Account.producerPubkey.toBase58(),
      tokenMint: h2Account.tokenMint.toBase58(),
    });
  });

  it("Register Produce with 900 kWh", async () => {
    // 1. Fetch account states
    const eacAccountBefore = await program.account.eac.fetch(eacPda);
    const h2CanisterBefore = await program.account.h2Canister.fetch(h2Pda);

    // 2. Derive token mints from on-chain state
    const eacMint = eacAccountBefore.tokenMint;
    const h2Mint = h2CanisterBefore.tokenMint;

    // 3. Derive ATAs
    const producerEacAta = await getAssociatedTokenAddress(
      eacMint,
      producerAuthority.publicKey
    );
    const producerH2Ata = await getAssociatedTokenAddress(
      h2Mint,
      producerAuthority.publicKey
    );

    // 4. Read balances before
    const eacAtaBefore = await getAccount(provider.connection, producerEacAta);
    const h2AtaBefore = await getAccount(provider.connection, producerH2Ata);

    console.log("🔍 Balances BEFORE:");
    console.log("🔥 EAC ATA:", Number(eacAtaBefore.amount));
    console.log("💧 H2 ATA:", Number(h2AtaBefore.amount));

    // 5. Call instruction
    const tx = await program.methods
      .producerRegisterBatch(new anchor.BN(900))
      .accounts({
        producer: producerPda,
        h2Canister: h2Pda,
        eac: eacPda,
        h2Mint,
        eacMint,
        producerH2Ata,
        producerEacAta,
        authority: producerAuthority.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([producerAuthority])
      .rpc();

    console.log("✅ TX Signature:", tx);

    // 6. Fetch states after
    const eacAccountAfter = await program.account.eac.fetch(eacPda);
    const h2CanisterAfter = await program.account.h2Canister.fetch(h2Pda);
    const eacAtaAfter = await getAccount(provider.connection, producerEacAta);
    const h2AtaAfter = await getAccount(provider.connection, producerH2Ata);

    console.log("📄 EAC Account After Register:", {
      certificateCapacityKwts:
        eacAccountAfter.certificateCapacityKwts.toNumber(),
      availableKwts: eacAccountAfter.availableKwts.toNumber(),
      burnedKwts: eacAccountAfter.burnedKwts.toNumber(),
    });

    console.log("📄 H2 Canister Account After Register:", {
      totalAmount: h2CanisterAfter.totalAmount.toNumber(),
      availableHydrogen: h2CanisterAfter.availableHydrogen.toNumber(),
    });

    console.log("📦 Balances AFTER:");
    console.log("🔥 EAC ATA:", Number(eacAtaAfter.amount));
    console.log("💧 H2 ATA:", Number(h2AtaAfter.amount));
  });

  // it("Add 1000 kWh to EAC", async () => {
  //   const tx = await program.methods
  //     .addKilowattsToEac(new anchor.BN(1000))
  //     .accounts({
  //       eac: eacPda,
  //       authority: producerAuthority.publicKey,
  //     })
  //     .signers([producerAuthority])
  //     .rpc();

  //   console.log("✅ addKilowattsToEac tx signature:", tx);

  //   const eacAccount = await program.account.eac.fetch(eacPda);
  //   console.log("📄 EAC Account After Add:", eacAccount);

  //   const h2Account = await program.account.h2Canister.fetch(h2Pda);
  //   console.log("📄 H2 Canister Account:", h2Account);
  //   // assert.equal(h2Account.totalAmount.toNumber(), 0);
  // });

  // it("Fail to Register Produce with 101 kWh", async () => {
  //   try {
  //     await program.methods
  //       .producerRegisterBatch(new anchor.BN(101))
  //       .accounts({
  //         producer: producerPda,
  //         h2Canister: h2Pda,
  //         eac: eacPda,
  //         authority: producerAuthority.publicKey,
  //       })
  //       .signers([producerAuthority])
  //       .rpc();
  //     assert.fail("❌ Should have thrown due to insufficient kWh");
  //   } catch (err) {
  //     console.log("✅ Expected error:", err);

  //     const eacAccount = await program.account.eac.fetch(eacPda);
  //     const h2Account = await program.account.h2Canister.fetch(h2Pda);
  //     console.log("📄 EAC Account After Failed Register:", eacAccount);
  //     console.log("📄 H2 Account After Failed Register:", h2Account);

  //     // assert.equal(eacAccount.availableAmount.toNumber(), 86);
  //     // assert.equal(eacAccount.burnedAmount.toNumber(), 914);
  //     // assert.equal(h2Account.availableAmount.toNumber(), 15233);
  //     // assert.equal(h2Account.totalAmount.toNumber(), 15233);
  //   }
  // });
});
