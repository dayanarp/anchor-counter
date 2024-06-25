import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { BN } from "bn.js";
import { assert } from "chai";

describe("counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;
  const counterKp = new anchor.web3.Keypair();
  const initValue = new BN(0);

  // ---------------- CREATE ------------------///
  it("should create a counter", async () => {
    const tx = await program.methods.createCounter(initValue).accounts({
      counter: counterKp.publicKey,
      authority: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .signers([counterKp])
    .rpc()

    console.log("Your transaction signature", tx);
    const counter = await program.account.counter.fetch(counterKp.publicKey);
    console.log("counter count is:", counter.count.toNumber());
    assert(initValue.eq(counter.count));
  });
  // ---------------- UPDATE ------------------///
  it("should update a counter", async () => {

    const number = new BN(23);
    const tx = await program.methods.updateCounter(number).accounts({
      counter: counterKp.publicKey,
      authority: provider.wallet.publicKey,
    })
    .rpc()

    console.log("Your transaction signature", tx);
    const counter = await program.account.counter.fetch(counterKp.publicKey);
    console.log("counter count is:", counter.count.toNumber());
    assert(number.eq(counter.count));
  });

  // ---------------- INCREMENT ------------------///
  it("should increment a counter (+1)", async () => {

    let counter = await program.account.counter.fetch(counterKp.publicKey);
    const oldValue = counter.count;
    const tx = await program.methods.incrementCounter().accounts({
      counter: counterKp.publicKey,
      authority: provider.wallet.publicKey,
    })
    .rpc()

    console.log("Your transaction signature", tx);
    counter = await program.account.counter.fetch(counterKp.publicKey);
    console.log("counter count is:", counter.count.toNumber());
    assert(oldValue.lt(counter.count));
  });

  // ---------------- DECREMENT ------------------///
  it("should decrement a counter (-1)", async () => {

    let counter = await program.account.counter.fetch(counterKp.publicKey);
    const oldValue = counter.count;
    const tx = await program.methods.decrementCounter().accounts({
      counter: counterKp.publicKey,
      authority: provider.wallet.publicKey,
    })
    .rpc()

    console.log("Your transaction signature", tx);
    counter = await program.account.counter.fetch(counterKp.publicKey);
    console.log("counter count is:", counter.count.toNumber());
    assert(oldValue.gt(counter.count));
  });

  // ---------------- DELETE ------------------///
  it("should delete a counter", async () => {
    const tx = await program.methods.deleteCounter().accounts({
      counter: counterKp.publicKey,
      authority: provider.wallet.publicKey,
    })
    .rpc()

    console.log("Your transaction signature", tx);
    const counter = await program.account.counter.fetchNullable(counterKp.publicKey);
    assert.equal(counter,null);
  });

});
