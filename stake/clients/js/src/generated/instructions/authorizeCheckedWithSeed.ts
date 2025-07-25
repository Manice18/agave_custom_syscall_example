/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  addDecoderSizePrefix,
  addEncoderSizePrefix,
  combineCodec,
  getAddressDecoder,
  getAddressEncoder,
  getStructDecoder,
  getStructEncoder,
  getU32Decoder,
  getU32Encoder,
  getUtf8Decoder,
  getUtf8Encoder,
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
import {
  getStakeAuthorizeDecoder,
  getStakeAuthorizeEncoder,
  type StakeAuthorize,
  type StakeAuthorizeArgs,
} from '../types';

export const AUTHORIZE_CHECKED_WITH_SEED_DISCRIMINATOR = 11;

export function getAuthorizeCheckedWithSeedDiscriminatorBytes() {
  return getU32Encoder().encode(AUTHORIZE_CHECKED_WITH_SEED_DISCRIMINATOR);
}

export type AuthorizeCheckedWithSeedInstruction<
  TProgram extends string = typeof STAKE_PROGRAM_ADDRESS,
  TAccountStake extends string | IAccountMeta<string> = string,
  TAccountBase extends string | IAccountMeta<string> = string,
  TAccountClockSysvar extends
    | string
    | IAccountMeta<string> = 'SysvarC1ock11111111111111111111111111111111',
  TAccountNewAuthority extends string | IAccountMeta<string> = string,
  TAccountLockupAuthority extends
    | string
    | IAccountMeta<string>
    | undefined = undefined,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountStake extends string
        ? WritableAccount<TAccountStake>
        : TAccountStake,
      TAccountBase extends string
        ? ReadonlySignerAccount<TAccountBase> & IAccountSignerMeta<TAccountBase>
        : TAccountBase,
      TAccountClockSysvar extends string
        ? ReadonlyAccount<TAccountClockSysvar>
        : TAccountClockSysvar,
      TAccountNewAuthority extends string
        ? ReadonlySignerAccount<TAccountNewAuthority> &
            IAccountSignerMeta<TAccountNewAuthority>
        : TAccountNewAuthority,
      ...(TAccountLockupAuthority extends undefined
        ? []
        : [
            TAccountLockupAuthority extends string
              ? ReadonlySignerAccount<TAccountLockupAuthority> &
                  IAccountSignerMeta<TAccountLockupAuthority>
              : TAccountLockupAuthority,
          ]),
      ...TRemainingAccounts,
    ]
  >;

export type AuthorizeCheckedWithSeedInstructionData = {
  discriminator: number;
  stakeAuthorize: StakeAuthorize;
  authoritySeed: string;
  authorityOwner: Address;
};

export type AuthorizeCheckedWithSeedInstructionDataArgs = {
  stakeAuthorize: StakeAuthorizeArgs;
  authoritySeed: string;
  authorityOwner: Address;
};

export function getAuthorizeCheckedWithSeedInstructionDataEncoder(): Encoder<AuthorizeCheckedWithSeedInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU32Encoder()],
      ['stakeAuthorize', getStakeAuthorizeEncoder()],
      [
        'authoritySeed',
        addEncoderSizePrefix(getUtf8Encoder(), getU32Encoder()),
      ],
      ['authorityOwner', getAddressEncoder()],
    ]),
    (value) => ({
      ...value,
      discriminator: AUTHORIZE_CHECKED_WITH_SEED_DISCRIMINATOR,
    })
  );
}

export function getAuthorizeCheckedWithSeedInstructionDataDecoder(): Decoder<AuthorizeCheckedWithSeedInstructionData> {
  return getStructDecoder([
    ['discriminator', getU32Decoder()],
    ['stakeAuthorize', getStakeAuthorizeDecoder()],
    ['authoritySeed', addDecoderSizePrefix(getUtf8Decoder(), getU32Decoder())],
    ['authorityOwner', getAddressDecoder()],
  ]);
}

export function getAuthorizeCheckedWithSeedInstructionDataCodec(): Codec<
  AuthorizeCheckedWithSeedInstructionDataArgs,
  AuthorizeCheckedWithSeedInstructionData
> {
  return combineCodec(
    getAuthorizeCheckedWithSeedInstructionDataEncoder(),
    getAuthorizeCheckedWithSeedInstructionDataDecoder()
  );
}

export type AuthorizeCheckedWithSeedInput<
  TAccountStake extends string = string,
  TAccountBase extends string = string,
  TAccountClockSysvar extends string = string,
  TAccountNewAuthority extends string = string,
  TAccountLockupAuthority extends string = string,
> = {
  /** Stake account to be updated */
  stake: Address<TAccountStake>;
  /** Base key of stake or withdraw authority */
  base: TransactionSigner<TAccountBase>;
  /** Clock sysvar */
  clockSysvar?: Address<TAccountClockSysvar>;
  /** The new stake or withdraw authority */
  newAuthority: TransactionSigner<TAccountNewAuthority>;
  /** Lockup authority */
  lockupAuthority?: TransactionSigner<TAccountLockupAuthority>;
  stakeAuthorize: AuthorizeCheckedWithSeedInstructionDataArgs['stakeAuthorize'];
  authoritySeed: AuthorizeCheckedWithSeedInstructionDataArgs['authoritySeed'];
  authorityOwner: AuthorizeCheckedWithSeedInstructionDataArgs['authorityOwner'];
};

export function getAuthorizeCheckedWithSeedInstruction<
  TAccountStake extends string,
  TAccountBase extends string,
  TAccountClockSysvar extends string,
  TAccountNewAuthority extends string,
  TAccountLockupAuthority extends string,
  TProgramAddress extends Address = typeof STAKE_PROGRAM_ADDRESS,
>(
  input: AuthorizeCheckedWithSeedInput<
    TAccountStake,
    TAccountBase,
    TAccountClockSysvar,
    TAccountNewAuthority,
    TAccountLockupAuthority
  >,
  config?: { programAddress?: TProgramAddress }
): AuthorizeCheckedWithSeedInstruction<
  TProgramAddress,
  TAccountStake,
  TAccountBase,
  TAccountClockSysvar,
  TAccountNewAuthority,
  TAccountLockupAuthority
> {
  // Program address.
  const programAddress = config?.programAddress ?? STAKE_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    stake: { value: input.stake ?? null, isWritable: true },
    base: { value: input.base ?? null, isWritable: false },
    clockSysvar: { value: input.clockSysvar ?? null, isWritable: false },
    newAuthority: { value: input.newAuthority ?? null, isWritable: false },
    lockupAuthority: {
      value: input.lockupAuthority ?? null,
      isWritable: false,
    },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  // Resolve default values.
  if (!accounts.clockSysvar.value) {
    accounts.clockSysvar.value =
      'SysvarC1ock11111111111111111111111111111111' as Address<'SysvarC1ock11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'omitted');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.stake),
      getAccountMeta(accounts.base),
      getAccountMeta(accounts.clockSysvar),
      getAccountMeta(accounts.newAuthority),
      getAccountMeta(accounts.lockupAuthority),
    ].filter(<T>(x: T | undefined): x is T => x !== undefined),
    programAddress,
    data: getAuthorizeCheckedWithSeedInstructionDataEncoder().encode(
      args as AuthorizeCheckedWithSeedInstructionDataArgs
    ),
  } as AuthorizeCheckedWithSeedInstruction<
    TProgramAddress,
    TAccountStake,
    TAccountBase,
    TAccountClockSysvar,
    TAccountNewAuthority,
    TAccountLockupAuthority
  >;

  return instruction;
}

export type ParsedAuthorizeCheckedWithSeedInstruction<
  TProgram extends string = typeof STAKE_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Stake account to be updated */
    stake: TAccountMetas[0];
    /** Base key of stake or withdraw authority */
    base: TAccountMetas[1];
    /** Clock sysvar */
    clockSysvar: TAccountMetas[2];
    /** The new stake or withdraw authority */
    newAuthority: TAccountMetas[3];
    /** Lockup authority */
    lockupAuthority?: TAccountMetas[4] | undefined;
  };
  data: AuthorizeCheckedWithSeedInstructionData;
};

export function parseAuthorizeCheckedWithSeedInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedAuthorizeCheckedWithSeedInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 4) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  let optionalAccountsRemaining = instruction.accounts.length - 4;
  const getNextOptionalAccount = () => {
    if (optionalAccountsRemaining === 0) return undefined;
    optionalAccountsRemaining -= 1;
    return getNextAccount();
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      stake: getNextAccount(),
      base: getNextAccount(),
      clockSysvar: getNextAccount(),
      newAuthority: getNextAccount(),
      lockupAuthority: getNextOptionalAccount(),
    },
    data: getAuthorizeCheckedWithSeedInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
