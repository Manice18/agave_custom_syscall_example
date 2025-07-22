import { describe, test, before } from "node:test";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { InstructionType, createTransferInstruction } from "./instruction";

const connection = new Connection("http://localhost:8899", "confirmed");

describe("transfer-sol using local validator", () => {
  const payer = Keypair.generate(); // Will airdrop
  const test1Recipient = Keypair.generate();

  const PROGRAM_ID = new PublicKey(
    "6fmTHukAQ4rpW399cxwvRhYHWHHYXY4jt4fzpBT6boXu"
  );
  const transferAmount = 1 * LAMPORTS_PER_SOL;

  before(async () => {
    // Airdrop 5 SOL to payer
    const sig = await connection.requestAirdrop(
      payer.publicKey,
      5 * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(sig);
  });

  test("Transfer between accounts using the system program", async () => {
    await getBalances(
      payer.publicKey,
      test1Recipient.publicKey,
      "Before Transfer"
    );

    const ix = createTransferInstruction(
      payer.publicKey,
      test1Recipient.publicKey,
      PROGRAM_ID,
      InstructionType.CpiTransfer,
      transferAmount
    );

    const tx = new Transaction().add(ix);
    const signature = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log("signature", signature);

    await getBalances(
      payer.publicKey,
      test1Recipient.publicKey,
      "After Transfer"
    );
  });

  async function getBalances(pub1: PublicKey, pub2: PublicKey, label: string) {
    const b1 = await connection.getBalance(pub1);
    const b2 = await connection.getBalance(pub2);
    console.log(`${label} balances:`);
    console.log(`   ${pub1.toBase58()}: ${b1 / LAMPORTS_PER_SOL} SOL`);
    console.log(`   ${pub2.toBase58()}: ${b2 / LAMPORTS_PER_SOL} SOL`);
  }
});
