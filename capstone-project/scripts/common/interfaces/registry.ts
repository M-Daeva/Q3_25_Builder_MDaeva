import { PublicKey } from "@solana/web3.js";

export interface InitArgs {
  rotationTimeout?: number;
  accountRegistrationFee?: AssetItem;
  accountDataSizeRange?: Range;
}

export interface AssetItem {
  amount: number;
  asset: PublicKey;
}

export interface Range {
  min: number;
  max: number;
}
