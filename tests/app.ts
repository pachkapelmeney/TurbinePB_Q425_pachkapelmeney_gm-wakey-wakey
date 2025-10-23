import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { App } from "../target/types/app";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.getProvider();
  const program = anchor.workspace.app as Program<App>;
  let counterPda: PublicKey;

  before(async () => {
    // Derive PDA exactly as in the program seeds
    counterPda = PublicKey.findProgramAddressSync(
      [Buffer.from("counter"), provider.wallet.publicKey.toBuffer()],
      program.programId
    )[0];

    // Initialize once if not exists
    const info = await provider.connection.getAccountInfo(counterPda);
    if (!info) {
      await program.methods
        .initialize()
        .accounts({
          authority: provider.wallet.publicKey,
          counter: counterPda,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
    }
  });

  it("Is initialized!", async () => {
    const account = await program.account.counter.fetch(counterPda);
    assert.isTrue(account.authority.equals(provider.wallet.publicKey));
    assert.equal(account.count, 0);
  });

  it("Can increment!", async () => {
    const before = (await program.account.counter.fetch(counterPda)).count;
    await program.methods
      .increaseCounter()
      .accounts({
        authority: provider.wallet.publicKey,
        counter: counterPda,
      })
      .rpc();

    const after = (await program.account.counter.fetch(counterPda)).count;
    assert.equal(after, before + 1);
  });
});

// Use PublicKey.findProgramAddressSync([Buffer.from("anchor"), provider.wallet.publicKey.toBytes()], program.programId) to get the PDA.
