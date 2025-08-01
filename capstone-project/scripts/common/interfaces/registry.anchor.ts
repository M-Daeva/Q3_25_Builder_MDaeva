// Auto-generated Anchor types and converters
import * as anchor from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { InitArgs, AssetItem, Range } from './registry';

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

