/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/clmm_mock.json`.
 */
export type ClmmMock = {
  "address": "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK",
  "metadata": {
    "name": "clmmMock",
    "version": "1.0.0",
    "spec": "0.1.0",
    "description": "Created with Anchor",
    "repository": "https://github.com/M-Daeva/solana-boilerplate"
  },
  "instructions": [
    {
      "name": "createAmmConfig",
      "discriminator": [
        137,
        52,
        237,
        212,
        215,
        117,
        108,
        104
      ],
      "accounts": [
        {
          "name": "owner",
          "docs": [
            "Address to be set as protocol owner."
          ],
          "writable": true,
          "signer": true
        },
        {
          "name": "ammConfig",
          "docs": [
            "Initialize config state account to store protocol owner address and fee rates."
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  109,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              },
              {
                "kind": "arg",
                "path": "index"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "index",
          "type": "u16"
        },
        {
          "name": "tickSpacing",
          "type": "u16"
        },
        {
          "name": "tradeFeeRate",
          "type": "u32"
        },
        {
          "name": "protocolFeeRate",
          "type": "u32"
        },
        {
          "name": "fundFeeRate",
          "type": "u32"
        }
      ]
    },
    {
      "name": "createOperationAccount",
      "discriminator": [
        63,
        87,
        148,
        33,
        109,
        35,
        8,
        104
      ],
      "accounts": [
        {
          "name": "owner",
          "docs": [
            "Address to be set as operation account owner."
          ],
          "writable": true,
          "signer": true
        },
        {
          "name": "operationState",
          "docs": [
            "Initialize operation state account to store operation owner address and white list mint."
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  112,
                  101,
                  114,
                  97,
                  116,
                  105,
                  111,
                  110
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "createPool",
      "discriminator": [
        233,
        146,
        209,
        142,
        207,
        104,
        64,
        188
      ],
      "accounts": [
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "sender",
          "writable": true,
          "signer": true
        },
        {
          "name": "poolConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "mintA"
              },
              {
                "kind": "account",
                "path": "mintB"
              }
            ]
          }
        },
        {
          "name": "mintA"
        },
        {
          "name": "mintB"
        },
        {
          "name": "senderAAta",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "sender"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "mintA"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "senderBAta",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "sender"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "mintB"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "appAAta",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "poolConfig"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "mintA"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "appBAta",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "poolConfig"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "mintB"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        }
      ],
      "args": [
        {
          "name": "amountA",
          "type": "u64"
        },
        {
          "name": "amountB",
          "type": "u64"
        }
      ]
    },
    {
      "name": "swapV2",
      "discriminator": [
        43,
        4,
        237,
        11,
        26,
        201,
        30,
        98
      ],
      "accounts": [
        {
          "name": "payer",
          "docs": [
            "The user performing the swap"
          ],
          "signer": true
        },
        {
          "name": "ammConfig",
          "docs": [
            "The factory state to read protocol fees"
          ]
        },
        {
          "name": "poolState",
          "docs": [
            "The program account of the pool in which the swap will be performed"
          ],
          "writable": true
        },
        {
          "name": "inputTokenAccount",
          "docs": [
            "The user token account for input token"
          ],
          "writable": true
        },
        {
          "name": "outputTokenAccount",
          "docs": [
            "The user token account for output token"
          ],
          "writable": true
        },
        {
          "name": "inputVault",
          "docs": [
            "The vault token account for input token"
          ],
          "writable": true
        },
        {
          "name": "outputVault",
          "docs": [
            "The vault token account for output token"
          ],
          "writable": true
        },
        {
          "name": "observationState",
          "docs": [
            "The program account for the most recent oracle observation"
          ],
          "writable": true
        },
        {
          "name": "tokenProgram",
          "docs": [
            "SPL program for token transfers"
          ],
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "tokenProgram2022",
          "docs": [
            "SPL program 2022 for token transfers"
          ],
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        },
        {
          "name": "memoProgram",
          "docs": [
            "Memo program"
          ],
          "address": "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"
        },
        {
          "name": "inputVaultMint",
          "docs": [
            "The mint of token vault 0"
          ]
        },
        {
          "name": "outputVaultMint",
          "docs": [
            "The mint of token vault 1"
          ]
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "otherAmountThreshold",
          "type": "u64"
        },
        {
          "name": "sqrtPriceLimitX64",
          "type": "u128"
        },
        {
          "name": "isBaseInput",
          "type": "bool"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "ammConfig",
      "discriminator": [
        218,
        244,
        33,
        104,
        203,
        203,
        43,
        111
      ]
    },
    {
      "name": "observationState",
      "discriminator": [
        122,
        174,
        197,
        53,
        129,
        9,
        165,
        132
      ]
    },
    {
      "name": "operationState",
      "discriminator": [
        19,
        236,
        58,
        237,
        81,
        222,
        183,
        252
      ]
    },
    {
      "name": "poolConfig",
      "discriminator": [
        26,
        108,
        14,
        123,
        116,
        230,
        129,
        43
      ]
    },
    {
      "name": "poolState",
      "discriminator": [
        247,
        237,
        227,
        245,
        215,
        195,
        222,
        70
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "noParameters",
      "msg": "Parameters are not provided!"
    },
    {
      "code": 6001,
      "name": "wrongAssetType",
      "msg": "Wrong asset type!"
    },
    {
      "code": 6002,
      "name": "zeroAmount",
      "msg": "Zero amount to send!"
    },
    {
      "code": 6003,
      "name": "exceededAvailableAssetAmount",
      "msg": "Exceeded available asset amount!"
    },
    {
      "code": 6004,
      "name": "contractIsPaused",
      "msg": "The contract is temporary paused!"
    },
    {
      "code": 6005,
      "name": "maxDataSizeIsOutOfRange",
      "msg": "Max data size is out of range!"
    },
    {
      "code": 6006,
      "name": "maxDataSizeIsExceeded",
      "msg": "Max data size is exceeded!"
    },
    {
      "code": 6007,
      "name": "wrongUserId",
      "msg": "Wrong user ID!"
    },
    {
      "code": 6008,
      "name": "activateAccountTwice",
      "msg": "Account can't be activated twice!"
    },
    {
      "code": 6009,
      "name": "accountIsNotActivated",
      "msg": "Account isn't activated!"
    },
    {
      "code": 6010,
      "name": "badNonce",
      "msg": "Nonce must be unique!"
    }
  ],
  "types": [
    {
      "name": "ammConfig",
      "docs": [
        "Holds the current owner of the factory"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "docs": [
              "Bump to identify PDA"
            ],
            "type": "u8"
          },
          {
            "name": "index",
            "type": "u16"
          },
          {
            "name": "owner",
            "docs": [
              "Address of the protocol owner"
            ],
            "type": "pubkey"
          },
          {
            "name": "protocolFeeRate",
            "docs": [
              "The protocol fee"
            ],
            "type": "u32"
          },
          {
            "name": "tradeFeeRate",
            "docs": [
              "The trade fee, denominated in hundredths of a bip (10^-6)"
            ],
            "type": "u32"
          },
          {
            "name": "tickSpacing",
            "docs": [
              "The tick spacing"
            ],
            "type": "u16"
          },
          {
            "name": "fundFeeRate",
            "docs": [
              "The fund fee, denominated in hundredths of a bip (10^-6)"
            ],
            "type": "u32"
          },
          {
            "name": "paddingU32",
            "type": "u32"
          },
          {
            "name": "fundOwner",
            "type": "pubkey"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u64",
                3
              ]
            }
          }
        ]
      }
    },
    {
      "name": "observation",
      "docs": [
        "The element of observations in ObservationState"
      ],
      "serialization": "bytemuckunsafe",
      "repr": {
        "kind": "c",
        "packed": true
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "blockTimestamp",
            "docs": [
              "The block timestamp of the observation"
            ],
            "type": "u32"
          },
          {
            "name": "sqrtPriceX64",
            "docs": [
              "the price of the observation timestamp, Q64.64"
            ],
            "type": "u128"
          },
          {
            "name": "cumulativeTimePriceX64",
            "docs": [
              "the cumulative of price during the duration time, Q64.64"
            ],
            "type": "u128"
          },
          {
            "name": "padding",
            "docs": [
              "padding for feature update"
            ],
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "observationState",
      "serialization": "bytemuckunsafe",
      "repr": {
        "kind": "c",
        "packed": true
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "initialized",
            "docs": [
              "Whether the ObservationState is initialized"
            ],
            "type": "bool"
          },
          {
            "name": "poolId",
            "type": "pubkey"
          },
          {
            "name": "observations",
            "docs": [
              "observation array"
            ],
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "observation"
                  }
                },
                1000
              ]
            }
          },
          {
            "name": "padding",
            "docs": [
              "padding for feature update"
            ],
            "type": {
              "array": [
                "u128",
                5
              ]
            }
          }
        ]
      }
    },
    {
      "name": "operationState",
      "docs": [
        "Holds the current owner of the factory"
      ],
      "serialization": "bytemuckunsafe",
      "repr": {
        "kind": "c",
        "packed": true
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "docs": [
              "Bump to identify PDA"
            ],
            "type": "u8"
          },
          {
            "name": "operationOwners",
            "docs": [
              "Address of the operation owner"
            ],
            "type": {
              "array": [
                "pubkey",
                10
              ]
            }
          },
          {
            "name": "whitelistMints",
            "docs": [
              "The mint address of whitelist to emit reward"
            ],
            "type": {
              "array": [
                "pubkey",
                100
              ]
            }
          }
        ]
      }
    },
    {
      "name": "poolConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "mintA",
            "type": "pubkey"
          },
          {
            "name": "mintB",
            "type": "pubkey"
          },
          {
            "name": "amountA",
            "type": "u64"
          },
          {
            "name": "amountB",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "poolState",
      "docs": [
        "The pool state",
        "",
        "PDA of `[POOL_SEED, config, token_mint_0, token_mint_1]`",
        ""
      ],
      "serialization": "bytemuckunsafe",
      "repr": {
        "kind": "c",
        "packed": true
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "docs": [
              "Bump to identify PDA"
            ],
            "type": {
              "array": [
                "u8",
                1
              ]
            }
          },
          {
            "name": "ammConfig",
            "type": "pubkey"
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "tokenMint0",
            "docs": [
              "Token pair of the pool, where token_mint_0 address < token_mint_1 address"
            ],
            "type": "pubkey"
          },
          {
            "name": "tokenMint1",
            "type": "pubkey"
          },
          {
            "name": "tokenVault0",
            "docs": [
              "Token pair vault"
            ],
            "type": "pubkey"
          },
          {
            "name": "tokenVault1",
            "type": "pubkey"
          },
          {
            "name": "observationKey",
            "docs": [
              "observation account key"
            ],
            "type": "pubkey"
          },
          {
            "name": "mintDecimals0",
            "docs": [
              "mint0 and mint1 decimals"
            ],
            "type": "u8"
          },
          {
            "name": "mintDecimals1",
            "type": "u8"
          },
          {
            "name": "tickSpacing",
            "docs": [
              "The minimum number of ticks between initialized ticks"
            ],
            "type": "u16"
          },
          {
            "name": "liquidity",
            "docs": [
              "The currently in range liquidity available to the pool."
            ],
            "type": "u128"
          },
          {
            "name": "sqrtPriceX64",
            "docs": [
              "The current price of the pool as a sqrt(token_1/token_0) Q64.64 value"
            ],
            "type": "u128"
          },
          {
            "name": "tickCurrent",
            "docs": [
              "The current tick of the pool, i.e. according to the last tick transition that was run."
            ],
            "type": "i32"
          },
          {
            "name": "observationIndex",
            "docs": [
              "the most-recently updated index of the observations array"
            ],
            "type": "u16"
          },
          {
            "name": "observationUpdateDuration",
            "type": "u16"
          },
          {
            "name": "feeGrowthGlobal0X64",
            "docs": [
              "The fee growth as a Q64.64 number, i.e. fees of token_0 and token_1 collected per",
              "unit of liquidity for the entire life of the pool."
            ],
            "type": "u128"
          },
          {
            "name": "feeGrowthGlobal1X64",
            "type": "u128"
          },
          {
            "name": "protocolFeesToken0",
            "docs": [
              "The amounts of token_0 and token_1 that are owed to the protocol."
            ],
            "type": "u64"
          },
          {
            "name": "protocolFeesToken1",
            "type": "u64"
          },
          {
            "name": "swapInAmountToken0",
            "docs": [
              "The amounts in and out of swap token_0 and token_1"
            ],
            "type": "u128"
          },
          {
            "name": "swapOutAmountToken1",
            "type": "u128"
          },
          {
            "name": "swapInAmountToken1",
            "type": "u128"
          },
          {
            "name": "swapOutAmountToken0",
            "type": "u128"
          },
          {
            "name": "status",
            "docs": [
              "Bitwise representation of the state of the pool",
              "bit0, 1: disable open position and increase liquidity, 0: normal",
              "bit1, 1: disable decrease liquidity, 0: normal",
              "bit2, 1: disable collect fee, 0: normal",
              "bit3, 1: disable collect reward, 0: normal",
              "bit4, 1: disable swap, 0: normal"
            ],
            "type": "u8"
          },
          {
            "name": "padding",
            "docs": [
              "Leave blank for future use"
            ],
            "type": {
              "array": [
                "u8",
                7
              ]
            }
          },
          {
            "name": "rewardInfos",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "rewardInfo"
                  }
                },
                3
              ]
            }
          },
          {
            "name": "tickArrayBitmap",
            "docs": [
              "Packed initialized tick array state"
            ],
            "type": {
              "array": [
                "u64",
                16
              ]
            }
          },
          {
            "name": "totalFeesToken0",
            "docs": [
              "except protocol_fee and fund_fee"
            ],
            "type": "u64"
          },
          {
            "name": "totalFeesClaimedToken0",
            "docs": [
              "except protocol_fee and fund_fee"
            ],
            "type": "u64"
          },
          {
            "name": "totalFeesToken1",
            "type": "u64"
          },
          {
            "name": "totalFeesClaimedToken1",
            "type": "u64"
          },
          {
            "name": "fundFeesToken0",
            "type": "u64"
          },
          {
            "name": "fundFeesToken1",
            "type": "u64"
          },
          {
            "name": "openTime",
            "type": "u64"
          },
          {
            "name": "padding1",
            "type": {
              "array": [
                "u64",
                25
              ]
            }
          },
          {
            "name": "padding2",
            "type": {
              "array": [
                "u64",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "rewardInfo",
      "serialization": "bytemuckunsafe",
      "repr": {
        "kind": "c",
        "packed": true
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rewardState",
            "docs": [
              "Reward state"
            ],
            "type": "u8"
          },
          {
            "name": "openTime",
            "docs": [
              "Reward open time"
            ],
            "type": "u64"
          },
          {
            "name": "endTime",
            "docs": [
              "Reward end time"
            ],
            "type": "u64"
          },
          {
            "name": "lastUpdateTime",
            "docs": [
              "Reward last update time"
            ],
            "type": "u64"
          },
          {
            "name": "emissionsPerSecondX64",
            "docs": [
              "Q64.64 number indicates how many tokens per second are earned per unit of liquidity."
            ],
            "type": "u128"
          },
          {
            "name": "rewardTotalEmissioned",
            "docs": [
              "The total amount of reward emissioned"
            ],
            "type": "u64"
          },
          {
            "name": "rewardClaimed",
            "docs": [
              "The total amount of claimed reward"
            ],
            "type": "u64"
          },
          {
            "name": "tokenMint",
            "docs": [
              "Reward token mint."
            ],
            "type": "pubkey"
          },
          {
            "name": "tokenVault",
            "docs": [
              "Reward vault token account."
            ],
            "type": "pubkey"
          },
          {
            "name": "authority",
            "docs": [
              "The owner that has permission to set reward param"
            ],
            "type": "pubkey"
          },
          {
            "name": "rewardGrowthGlobalX64",
            "docs": [
              "Q64.64 number that tracks the total tokens earned per unit of liquidity since the reward",
              "emissions were turned on."
            ],
            "type": "u128"
          }
        ]
      }
    }
  ]
};
