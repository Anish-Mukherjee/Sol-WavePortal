import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Waveportal } from "../target/types/waveportal";

describe("waveportal", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Waveportal as Program<Waveportal>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
