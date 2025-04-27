import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { H2uContracts } from "../target/types/h2u_contracts";

describe("h2u_contracts", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.H2uContracts as Program<H2uContracts>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
