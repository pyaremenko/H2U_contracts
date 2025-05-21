import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";
import { Marketplace } from "../../../target/types/marketplace";

describe("market config update", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Marketplace as Program<Marketplace>;

  const signer = provider.wallet as anchor.Wallet;

  let configPda: PublicKey;
  let signerPda: PublicKey;
  let configBump: number;

  const newAuthority = anchor.web3.Keypair.generate();

  it("Initializes config", async () => {
    [configPda, configBump] = await PublicKey.findProgramAddress(
      [Buffer.from("config")],
      program.programId
    );

    [signerPda] = await PublicKey.findProgramAddress(
      [Buffer.from("signer")],
      program.programId
    );

    await program.methods
      .initializeConfig()
      .accounts({
        config: configPda,
        payer: signer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const configAccount = await program.account.marketConfig.fetch(configPda);

    assert.ok(configAccount.authority.equals(signer.publicKey));
    assert.equal(configAccount.signerBump, configBump);
  });

  it("Updates config authority", async () => {
    await program.methods
      .updateConfiguration()
      .accounts({
        config: configPda,
        authority: signer.publicKey,
      })
      .rpc();

    const configAccount = await program.account.marketConfig.fetch(configPda);
  });
});
