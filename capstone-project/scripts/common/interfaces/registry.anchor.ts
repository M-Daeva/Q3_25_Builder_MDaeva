// Auto-generated Anchor types and converters
import * as anchor from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { InitArgs, AssetItem, Range, UpdateConfigArgs, WithdrawRevenueArgs, CreateAccountArgs, ReopenAccountArgs, ActivateAccountArgs, WriteDataArgs, RequestAccountRotationArgs } from './registry';

// Anchor-generated types
export type AnchorInitArgs = [
  number | null,
  AnchorAssetItem | null,
  AnchorRange | null
];

export interface AnchorAssetItem {
  amount: anchor.BN;
  asset: PublicKey;
}

export interface AnchorRange {
  min: number;
  max: number;
}

export type AnchorUpdateConfigArgs = [
  PublicKey | null,
  boolean | null,
  number | null,
  AnchorAssetItem | null,
  AnchorRange | null
];

export type AnchorWithdrawRevenueArgs = [
  anchor.BN | null
];

export type AnchorCreateAccountArgs = [
  number,
  number
];

export type AnchorReopenAccountArgs = [
  number
];

export type AnchorActivateAccountArgs = [
  PublicKey | null
];

export type AnchorWriteDataArgs = [
  string,
  anchor.BN
];

export type AnchorRequestAccountRotationArgs = [
  PublicKey
];


// Type converters
export function convertInitArgs(
  args: InitArgs
): AnchorInitArgs {
  return [
    args.rotationTimeout !== undefined ? args.rotationTimeout : null,
    args.accountRegistrationFee !== undefined ? convertAssetItem(args.accountRegistrationFee) : null,
    args.accountDataSizeRange !== undefined ? convertRange(args.accountDataSizeRange) : null
  ];
}

export function convertAssetItem(
  obj: AssetItem
): AnchorAssetItem {
  return {
    amount: new anchor.BN(obj.amount),
    asset: obj.asset,
  };
}

export function convertRange(
  obj: Range
): AnchorRange {
  return {
    min: obj.min,
    max: obj.max,
  };
}

export function convertUpdateConfigArgs(
  args: UpdateConfigArgs
): AnchorUpdateConfigArgs {
  return [
    args.admin !== undefined ? args.admin : null,
    args.is_paused !== undefined ? args.is_paused : null,
    args.rotation_timeout !== undefined ? args.rotation_timeout : null,
    args.registration_fee !== undefined ? convertAssetItem(args.registration_fee) : null,
    args.data_size_range !== undefined ? convertRange(args.data_size_range) : null
  ];
}

export function convertWithdrawRevenueArgs(
  args: WithdrawRevenueArgs
): AnchorWithdrawRevenueArgs {
  return [
    args.amount !== undefined ? new anchor.BN(args.amount) : null
  ];
}

export function convertCreateAccountArgs(
  args: CreateAccountArgs
): AnchorCreateAccountArgs {
  return [
    args.maxDataSize,
    args.expectedUserId
  ];
}

export function convertReopenAccountArgs(
  args: ReopenAccountArgs
): AnchorReopenAccountArgs {
  return [
    args.maxDataSize
  ];
}

export function convertActivateAccountArgs(
  args: ActivateAccountArgs
): AnchorActivateAccountArgs {
  return [
    args.user !== undefined ? args.user : null
  ];
}

export function convertWriteDataArgs(
  args: WriteDataArgs
): AnchorWriteDataArgs {
  return [
    args.data,
    new anchor.BN(args.nonce)
  ];
}

export function convertRequestAccountRotationArgs(
  args: RequestAccountRotationArgs
): AnchorRequestAccountRotationArgs {
  return [
    args.newOwner
  ];
}

