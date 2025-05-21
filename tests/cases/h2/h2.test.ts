import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hydrogen } from "../../../target/types/hydrogen";
import { assert } from "chai";
import { PublicKey, SystemProgram } from "@solana/web3.js";

describe("hydrogen", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Hydrogen as Program<Hydrogen>;

  const producerKeypair = anchor.web3.Keypair.generate();
  let producerPda: PublicKey;
  let h2CanisterPda: PublicKey;
  let bumpProducer: number;
  let bumpH2Canister: number;

  before(async () => {
    [producerPda, bumpProducer] = PublicKey.findProgramAddressSync(
      [Buffer.from("producer"), producerKeypair.publicKey.toBuffer()],
      program.programId
    );
    [h2CanisterPda, bumpH2Canister] = PublicKey.findProgramAddressSync(
      [Buffer.from("h2_canister"), producerKeypair.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Initialize Producer", async () => {
    await program.methods
      .initializeProducer(new anchor.BN(1), "Test Producer")
      .accounts({
        producer: producerPda,
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([])
      .rpc();

    const producerAccount = await program.account.producer.fetch(producerPda);
    assert.equal(producerAccount.name, "Test Producer");
  });

  // it("Mint H2 NFT", async () => {
  //   await program.methods
  //     .mintH2Nft(new anchor.BN(5)) // 5 tons of hydrogen
  //     .accounts({
  //       h2Canister: h2CanisterPda,
  //       producer: producerPda,
  //     })
  //     .rpc();

  //   const h2CanisterAccount = await program.account.h2Canister.fetch(
  //     h2CanisterPda
  //   );
  //   assert.equal(h2CanisterAccount.totalAmount.toNumber(), 5);
  // });
});
