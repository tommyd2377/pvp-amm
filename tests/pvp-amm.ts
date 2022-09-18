import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PvpAmm } from "../target/types/pvp_amm";

describe("pvp-amm", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PvpAmm as Program<PvpAmm>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
