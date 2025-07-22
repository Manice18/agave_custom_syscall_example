import { Buffer } from "node:buffer";
import {
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import * as borsh from "borsh";
import BN from "bn.js";

export enum InstructionType {
  CpiTransfer = 0,
}

export class TransferInstruction {
  instruction: InstructionType;
  amount: BN;

  constructor(props: { instruction: InstructionType; amount: number | BN }) {
    this.instruction = props.instruction;
    this.amount = new BN(props.amount);
  }

  toBuffer() {
    return Buffer.from(borsh.serialize(TransferInstructionSchema, this));
  }

  static fromBuffer(buffer: Buffer) {
    return borsh.deserialize(
      TransferInstructionSchema,
      // @ts-ignore
      TransferInstruction,
      buffer
    );
  }
}

export const TransferInstructionSchema = {
  struct: {
    instruction: "u8",
    amount: "u64",
  },
};

export function createTransferInstruction(
  payerPubkey: PublicKey,
  recipientPubkey: PublicKey,
  programId: PublicKey,
  instruction: InstructionType,
  amount: number
): TransactionInstruction {
  const instructionObject = new TransferInstruction({
    instruction,
    amount,
  });

  return new TransactionInstruction({
    keys: [
      { pubkey: payerPubkey, isSigner: true, isWritable: true },
      { pubkey: recipientPubkey, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId,
    data: instructionObject.toBuffer(),
  });
}
