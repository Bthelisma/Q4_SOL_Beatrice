import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";

describe("anchor_vault_q424", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorVaultQ424 as Program<AnchorVault>;

  it("initialize!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
  it("deposit", async () => {
    // Add your test here.
    const tx = await program.methods.deposit(new anchor.BN(0.2)).rpc();
    console.log("Your transaction signature", tx);
  });
  it("withdraw", async () => {
    // Add your test here.
    const tx = await program.methods.withdraw(new anchor.BN(0.1)).rpc();
    console.log("Your transaction signature", tx);
  });
});