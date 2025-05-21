import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import {
  getAssociatedTokenAddress,
  getAccount,
  createMint,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { assert } from "chai";

import { Marketplace } from "../../../target/types/marketplace";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { Hydrogen } from "../../../target/types/hydrogen";
import { Oracle } from "../../../target/types/oracle";

const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

const realUsdcMint = new PublicKey(
  "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU" // devnet 4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU
);

describe("h2u + market merged", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const hydrogen = anchor.workspace.Hydrogen as Program<Hydrogen>;
  const marketplace = anchor.workspace.Marketplace as Program<Marketplace>;
  const oracle = anchor.workspace.Oracle as Program<Oracle>;

  const producerAuthority = Keypair.generate();
  const signer = provider.wallet as anchor.Wallet;

  let producerPda: PublicKey;
  let eacPda: PublicKey;
  let h2Pda: PublicKey;
  let configPda: PublicKey;
  let signerPda: PublicKey;
  let configBump: number;
  let h2Mint: PublicKey;
  let producerH2Ata: PublicKey;
  let transferManagerPda: PublicKey;
  let transferManagerAta: PublicKey;
  let listingPda: PublicKey;

  before(async () => {
    console.log("🔄 Airdropping 4 SOL to producer authority...");
    await provider.connection.requestAirdrop(producerAuthority.publicKey, 4e9);
    await new Promise((resolve) => setTimeout(resolve, 1000));

    [producerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("producer"), producerAuthority.publicKey.toBuffer()],
      hydrogen.programId
    );

    [eacPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("eac"), producerPda.toBuffer(), Buffer.from("chlen")],
      hydrogen.programId
    );

    [h2Pda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("h2_canister"),
        producerAuthority.publicKey.toBuffer(),
        Buffer.from("chlen"),
      ],
      hydrogen.programId
    );

    [configPda, configBump] = await PublicKey.findProgramAddress(
      [Buffer.from("config")],
      marketplace.programId
    );

    [signerPda] = await PublicKey.findProgramAddress(
      [Buffer.from("signer")],
      marketplace.programId
    );

    console.log("📌 producerPda:", producerPda.toBase58());
    console.log("📌 eacPda:", eacPda.toBase58());
    console.log("📌 h2Pda:", h2Pda.toBase58());
    console.log("📌 configPda:", configPda.toBase58());
    console.log("📌 signerPda:", signerPda.toBase58());
  });

  it("Initializes Producer", async () => {
    const tx = await hydrogen.methods
      .initializeProducer(new anchor.BN(1), "Test Producer")
      .accounts({
        producer: producerPda,
        authority: producerAuthority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([producerAuthority])
      .rpc();

    console.log("✅ initializeProducer tx signature:", tx);

    const producerAccount = await hydrogen.account.producer.fetch(producerPda);
    assert.equal(producerAccount.name, "Test Producer");
  });

  it("Initializes EAC", async () => {
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

    const tx = await hydrogen.methods
      .initializeEacStorage(
        "chlen",
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

    console.log("✅ initializeEAC tx:", tx);
  });

  it("Initializes H2 Canister", async () => {
    h2Mint = await createMint(
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
        h2Mint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    producerH2Ata = await getAssociatedTokenAddress(
      h2Mint,
      producerAuthority.publicKey
    );
    console.log();

    const tx = await hydrogen.methods
      .initializeH2Canister(
        "chlen",
        "HydrogenCredit",
        "H2U",
        "https://example.com/metadata.json"
      )
      .accounts({
        h2Canister: h2Pda,
        tokenMint: h2Mint,
        metadataAccount: metadataPda,
        producerAta: producerH2Ata,
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

    console.log("✅ initializeH2Canister tx:", tx);
  });

  it("Register Produce with 900 kWh", async () => {
    // 1. Fetch account states
    const eacAccountBefore = await hydrogen.account.eac.fetch(eacPda);
    const h2CanisterBefore = await hydrogen.account.h2Canister.fetch(h2Pda);

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
    const tx = await hydrogen.methods
      .producerRegisterBatch("chlen", new anchor.BN(900))
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
    const eacAccountAfter = await hydrogen.account.eac.fetch(eacPda);
    const h2CanisterAfter = await hydrogen.account.h2Canister.fetch(h2Pda);
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

  it("Initializes Market Config", async () => {
    const [configPda, configBump] = await PublicKey.findProgramAddress(
      [Buffer.from("config")],
      marketplace.programId
    );

    const [transferManagerPda, transferManagerBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("transfer_manager")],
        marketplace.programId
      );

    console.log("📌 Config PDA:", configPda.toBase58());
    console.log("📌 Transfer Manager PDA:", transferManagerPda.toBase58());
    console.log(
      "🔢 Bumps → config:",
      configBump,
      "| transfer_manager:",
      transferManagerBump
    );

    const tx = await marketplace.methods
      .initializeConfig()
      .accounts({
        config: configPda,
        authority: signer.publicKey,
        transferManager: transferManagerPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("✅ initializeConfig tx signature:", tx);

    const config = await marketplace.account.marketConfig.fetch(configPda);

    console.log("📄 Loaded Config Account:");
    console.log(" - authority:", config.authority.toBase58());
    console.log(" - transferManager:", config.transferManager.toBase58());
    console.log(" - transferManagerBump:", config.transferManagerBump);

    assert.ok(config.authority.equals(signer.publicKey));
    assert.ok(config.transferManager.equals(transferManagerPda));
    assert.equal(config.transferManagerBump, transferManagerBump);
  });

  it("Lists H2 to the marketplace (to transfer manager ATA)", async () => {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("listing"), h2Pda.toBuffer()],
      marketplace.programId
    );
    listingPda = pda;

    [transferManagerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("transfer_manager")],
      marketplace.programId
    );

    const transferManagerAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer.payer,
      h2Mint,
      transferManagerPda,
      true // allow owner to be a PDA
    );

    console.log("📦 Producer H2 ATA:", producerH2Ata.toBase58());
    console.log(
      "🏦 Transfer Manager ATA (Buffer):",
      transferManagerAta.address.toBase58()
    );

    const tx = await marketplace.methods
      .listTokens(new anchor.BN(10), new anchor.BN(1))
      .accounts({
        listing: listingPda,
        producerAuthority: producerAuthority.publicKey, // ✅ match your Rust context
        producer: producerPda,
        h2Canister: h2Pda,
        producerAta: producerH2Ata,
        transferManagerAta: transferManagerAta.address,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([producerAuthority]) // ✅ must match producer_authority
      .rpc();
    console.log("✅ listH2 tx signature:", tx);

    const balance = await getAccount(
      provider.connection,
      transferManagerAta.address
    );
    const listing = await marketplace.account.listing.fetch(listingPda);

    console.log("📄 Listing created:");
    console.log(" - price:", listing.price.toString());
    console.log(" - producer:", listing.producer.toBase58());
    console.log(
      " - transfer manager ATA:",
      listing.transferManagerAta.toBase58()
    );

    assert.equal(Number(balance.amount), 10000000000);
    assert.ok(listing.transferManagerAta.equals(transferManagerAta.address));
    assert.ok(listing.producer.equals(producerPda));
  });

  it("Allows a buyer to purchase H2 at dynamic price and transfers SOL to producer", async () => {
    const buyer = anchor.web3.Keypair.generate();
    await provider.connection.requestAirdrop(buyer.publicKey, 15e9);
    await new Promise((r) => setTimeout(r, 1000));

    // Create a mock USDC mint
    const mockUsdcMint = await createMint(
      provider.connection,
      signer.payer,
      signer.publicKey, // Mint authority
      null,
      6 // 6 decimals, like USDC
    );

    const buyerUsdcAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer.payer,
      mockUsdcMint,
      buyer.publicKey
    );

    const producerUsdcAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer.payer,
      mockUsdcMint,
      producerAuthority.publicKey
    );

    // Step 3: Mint mock USDC to the buyer's ATA
    await mintTo(
      provider.connection,
      signer.payer,
      mockUsdcMint,
      buyerUsdcAta.address,
      signer.publicKey, // Mint authority
      100 * 10 ** 6 // 100 USDC in smallest units
    );

    const listingAccount = await marketplace.account.listing.fetch(listingPda);
    const h2Mint = (await hydrogen.account.h2Canister.fetch(h2Pda)).tokenMint;

    const [transferManagerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("transfer_manager")],
      marketplace.programId
    );

    const transferManagerAta = await getAssociatedTokenAddress(
      h2Mint,
      transferManagerPda,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const buyerAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer.payer,
      h2Mint,
      buyer.publicKey,
      true
    );

    const buyerSolBefore = await provider.connection.getBalance(
      buyer.publicKey
    );
    const producerSolBefore = await provider.connection.getBalance(
      producerAuthority.publicKey
    );

    // Step 4: Record balances before the transaction
    const buyerUsdcBefore = await getAccount(
      provider.connection,
      buyerUsdcAta.address
    );
    const producerUsdcBefore = await getAccount(
      provider.connection,
      producerUsdcAta.address
    );

    // Buyer defines their own price (should be ≥ listing.price) and amount
    const purchaseAmount = 1; // H2 tokens
    const pricePerToken = listingAccount.price.toNumber() + 1; // Slightly above listing

    const totalPayment = pricePerToken * purchaseAmount;
    const totalPaymentInSmallestUnits = totalPayment * 10 ** 6; // 2_000_000

    const tx = await marketplace.methods
      .sellTokens(new anchor.BN(purchaseAmount), new anchor.BN(pricePerToken))
      .accounts({
        config: configPda,
        listing: listingPda,
        buyer: buyer.publicKey,
        transferManager: transferManagerPda,
        transferManagerAta,
        buyerAta: buyerAta.address,
        producer: producerAuthority.publicKey,
        buyerUsdcAta: buyerUsdcAta.address,
        producerUsdcAta: producerUsdcAta.address,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([buyer])
      .rpc();

    console.log("✅ sellH2 tx signature:", tx);

    const buyerAtaAcc = await getAccount(provider.connection, buyerAta.address);
    const buyerSolAfter = await provider.connection.getBalance(buyer.publicKey);
    const producerSolAfter = await provider.connection.getBalance(
      producerAuthority.publicKey
    );

    console.log("💧 H2 tokens in buyer ATA:", Number(buyerAtaAcc.amount));
    console.log("💰 Buyer SOL diff:", buyerSolBefore - buyerSolAfter);
    console.log("💰 Producer SOL diff:", producerSolAfter - producerSolBefore);

    const buyerUsdcAfter = await getAccount(
      provider.connection,
      buyerUsdcAta.address
    );
    const producerUsdcAfter = await getAccount(
      provider.connection,
      producerUsdcAta.address
    );

    console.log("💧 H2 tokens in buyer ATA:", Number(buyerAtaAcc.amount));
    console.log("💵 Buyer USDC balance:", Number(buyerUsdcAfter.amount));
    console.log("💵 Producer USDC balance:", Number(producerUsdcAfter.amount));

    assert.equal(Number(buyerAtaAcc.amount), 1e9 * purchaseAmount);

    //assert.equal(buyerSolBefore - buyerSolAfter, totalPayment);
    //assert.equal(producerSolAfter - producerSolBefore, totalPayment);

    // Assert balance changes
    assert.equal(
      Number(buyerUsdcBefore.amount) - Number(buyerUsdcAfter.amount),
      totalPaymentInSmallestUnits,
      "Buyer's USDC balance should decrease by the total payment"
    );
    assert.equal(
      Number(producerUsdcAfter.amount) - Number(producerUsdcBefore.amount),
      totalPaymentInSmallestUnits,
      "Producer's USDC balance should increase by the total payment"
    );
  });

  it("Initializes config and updates price", async () => {
    const [oracleConfigPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("oracle_config")],
      oracle.programId
    );
    const [pricePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("oracle_price")],
      oracle.programId
    );

    const admin = provider.wallet.publicKey;

    // Initialize config
    await oracle.methods
      .initConfig()
      .accounts({
        oracleConfig: oracleConfigPda,
        oraclePrice: pricePda,
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([provider.wallet.payer])
      .rpc();

    // Update price
    await oracle.methods
      .updatePrice(new anchor.BN(150), new anchor.BN(300))
      .accounts({
        oracleConfig: oracleConfigPda,
        oraclePrice: pricePda,
        admin,
      })
      .signers([provider.wallet.payer])
      .rpc();

    const price = await oracle.account.oraclePrice.fetch(pricePda);

    console.log("✅ Oracle price updated:", {
      min: price.minPricePerKg.toNumber(),
      max: price.maxPricePerKg.toNumber(),
      updated: price.lastUpdated.toString(),
    });

    assert.equal(price.minPricePerKg.toNumber(), 150);
    assert.equal(price.maxPricePerKg.toNumber(), 300);
  });
  // Optional test for config authority update
  // const newAuthority = anchor.web3.Keypair.generate();
  // it("Updates config authority", async () => {
  //   await marketplace.methods
  //     .updateConfig(newAuthority.publicKey)
  //     .accounts({
  //       config: configPda,
  //       authority: signer.publicKey,
  //     })
  //     .rpc();
  //
  //   const config = await marketplace.account.marketConfig.fetch(configPda);
  //   assert.ok(config.authority.equals(newAuthority.publicKey));
  // });
});
