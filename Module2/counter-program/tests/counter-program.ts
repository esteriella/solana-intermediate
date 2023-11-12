import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterProgram } from "../target/types/counter_program";
import { Keypair, SystemProgram } from "@solana/web3.js";
import assert from "assert";

describe("counter-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  
  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.CounterProgram as Program<CounterProgram>;

  // Create a keypair for the counter account
  const counter = Keypair.generate();

  // Test case to create a Counter account
  it("Create Counter account!", async () => {
    // Call the create method of the program
    const tx = await program.methods.create()
    .accounts({
      counter: counter.publicKey,
      authority: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([counter])
    .rpc();
    console.log("");
    console.log("Counter Account Created!");
    console.log("Transaction signature", tx);
    console.log("");
  });

  // Test case to increment the Counter
  it("Increment Counter!", async () => {
    // Call the increment method of the program
    const tx = await program.methods.increment()
    .accounts({
      counter: counter.publicKey,
      authority: provider.wallet.publicKey
    })
    .rpc();
    const myCounterAccount = await program.account.counter.fetch(counter.publicKey);
    assert.ok(myCounterAccount.count.toString() === "1");
    console.log("");
    console.log('Increment count: ', myCounterAccount.count.toString());
    console.log("Transaction signature", tx);    
    console.log("");
  });

  // Test case to decrement the Counter
  it("Decrement Counter!", async () => {
    // Call the decrement method of the program
    const tx = await program.methods.decrement()
    .accounts({
      counter: counter.publicKey,
      authority: provider.wallet.publicKey
    })
    .rpc();
    const myCounterAccount = await program.account.counter.fetch(counter.publicKey);
    assert.ok(myCounterAccount.count.toString() === "0");
    console.log("");
    console.log('Decrement count: ', myCounterAccount.count.toString());
    console.log("Transaction signature", tx);    
    console.log("");
  });
});
