/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU32Decoder,
  getU32Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type ReadonlySignerAccount,
  type TransactionSigner,
  type WritableAccount,
} from '@solana/kit';
import { STAKE_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const DELEGATE_STAKE_DISCRIMINATOR = 2;

export function getDelegateStakeDiscriminatorBytes() {
  return getU32Encoder().encode(DELEGATE_STAKE_DISCRIMINATOR);
}

export type DelegateStakeInstruction<
  TProgram extends string = typeof STAKE_PROGRAM_ADDRESS,
  TAccountStake extends string | IAccountMeta<string> = string,
  TAccountVote extends string | IAccountMeta<string> = string,
  TAccountClockSysvar extends
    | string
    | IAccountMeta<string> = 'SysvarC1ock11111111111111111111111111111111',
  TAccountStakeHistory extends string | IAccountMeta<string> = string,
  TAccountUnused extends string | IAccountMeta<string> = string,
  TAccountStakeAuthority extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountStake extends string
        ? WritableAccount<TAccountStake>
        : TAccountStake,
      TAccountVote extends string
        ? ReadonlyAccount<TAccountVote>
        : TAccountVote,
      TAccountClockSysvar extends string
        ? ReadonlyAccount<TAccountClockSysvar>
        : TAccountClockSysvar,
      TAccountStakeHistory extends string
        ? ReadonlyAccount<TAccountStakeHistory>
        : TAccountStakeHistory,
      TAccountUnused extends string
        ? ReadonlyAccount<TAccountUnused>
        : TAccountUnused,
      TAccountStakeAuthority extends string
        ? ReadonlySignerAccount<TAccountStakeAuthority> &
            IAccountSignerMeta<TAccountStakeAuthority>
        : TAccountStakeAuthority,
      ...TRemainingAccounts,
    ]
  >;

export type DelegateStakeInstructionData = { discriminator: number };

export type DelegateStakeInstructionDataArgs = {};

export function getDelegateStakeInstructionDataEncoder(): Encoder<DelegateStakeInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU32Encoder()]]),
    (value) => ({ ...value, discriminator: DELEGATE_STAKE_DISCRIMINATOR })
  );
}

export function getDelegateStakeInstructionDataDecoder(): Decoder<DelegateStakeInstructionData> {
  return getStructDecoder([['discriminator', getU32Decoder()]]);
}

export function getDelegateStakeInstructionDataCodec(): Codec<
  DelegateStakeInstructionDataArgs,
  DelegateStakeInstructionData
> {
  return combineCodec(
    getDelegateStakeInstructionDataEncoder(),
    getDelegateStakeInstructionDataDecoder()
  );
}

export type DelegateStakeInput<
  TAccountStake extends string = string,
  TAccountVote extends string = string,
  TAccountClockSysvar extends string = string,
  TAccountStakeHistory extends string = string,
  TAccountUnused extends string = string,
  TAccountStakeAuthority extends string = string,
> = {
  /** Initialized stake account to be delegated */
  stake: Address<TAccountStake>;
  /** Vote account to which this stake will be delegated */
  vote: Address<TAccountVote>;
  /** Clock sysvar */
  clockSysvar?: Address<TAccountClockSysvar>;
  /** Stake history sysvar */
  stakeHistory: Address<TAccountStakeHistory>;
  /** Unused account, formerly the stake config */
  unused: Address<TAccountUnused>;
  /** Stake authority */
  stakeAuthority: TransactionSigner<TAccountStakeAuthority>;
};

export function getDelegateStakeInstruction<
  TAccountStake extends string,
  TAccountVote extends string,
  TAccountClockSysvar extends string,
  TAccountStakeHistory extends string,
  TAccountUnused extends string,
  TAccountStakeAuthority extends string,
  TProgramAddress extends Address = typeof STAKE_PROGRAM_ADDRESS,
>(
  input: DelegateStakeInput<
    TAccountStake,
    TAccountVote,
    TAccountClockSysvar,
    TAccountStakeHistory,
    TAccountUnused,
    TAccountStakeAuthority
  >,
  config?: { programAddress?: TProgramAddress }
): DelegateStakeInstruction<
  TProgramAddress,
  TAccountStake,
  TAccountVote,
  TAccountClockSysvar,
  TAccountStakeHistory,
  TAccountUnused,
  TAccountStakeAuthority
> {
  // Program address.
  const programAddress = config?.programAddress ?? STAKE_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    stake: { value: input.stake ?? null, isWritable: true },
    vote: { value: input.vote ?? null, isWritable: false },
    clockSysvar: { value: input.clockSysvar ?? null, isWritable: false },
    stakeHistory: { value: input.stakeHistory ?? null, isWritable: false },
    unused: { value: input.unused ?? null, isWritable: false },
    stakeAuthority: { value: input.stakeAuthority ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Resolve default values.
  if (!accounts.clockSysvar.value) {
    accounts.clockSysvar.value =
      'SysvarC1ock11111111111111111111111111111111' as Address<'SysvarC1ock11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'omitted');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.stake),
      getAccountMeta(accounts.vote),
      getAccountMeta(accounts.clockSysvar),
      getAccountMeta(accounts.stakeHistory),
      getAccountMeta(accounts.unused),
      getAccountMeta(accounts.stakeAuthority),
    ],
    programAddress,
    data: getDelegateStakeInstructionDataEncoder().encode({}),
  } as DelegateStakeInstruction<
    TProgramAddress,
    TAccountStake,
    TAccountVote,
    TAccountClockSysvar,
    TAccountStakeHistory,
    TAccountUnused,
    TAccountStakeAuthority
  >;

  return instruction;
}

export type ParsedDelegateStakeInstruction<
  TProgram extends string = typeof STAKE_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Initialized stake account to be delegated */
    stake: TAccountMetas[0];
    /** Vote account to which this stake will be delegated */
    vote: TAccountMetas[1];
    /** Clock sysvar */
    clockSysvar: TAccountMetas[2];
    /** Stake history sysvar */
    stakeHistory: TAccountMetas[3];
    /** Unused account, formerly the stake config */
    unused: TAccountMetas[4];
    /** Stake authority */
    stakeAuthority: TAccountMetas[5];
  };
  data: DelegateStakeInstructionData;
};

export function parseDelegateStakeInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedDelegateStakeInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 6) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      stake: getNextAccount(),
      vote: getNextAccount(),
      clockSysvar: getNextAccount(),
      stakeHistory: getNextAccount(),
      unused: getNextAccount(),
      stakeAuthority: getNextAccount(),
    },
    data: getDelegateStakeInstructionDataDecoder().decode(instruction.data),
  };
}
