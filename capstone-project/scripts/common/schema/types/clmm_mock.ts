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
          "name": "poolCreator",
          "docs": [
            "Address paying to create the pool. Can be anyone"
          ],
          "writable": true,
          "signer": true
        },
        {
          "name": "ammConfig",
          "docs": [
            "Which config the pool belongs to."
          ]
        },
        {
          "name": "poolState",
          "docs": [
            "Initialize an account to store the pool state"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "ammConfig"
              },
              {
                "kind": "account",
                "path": "tokenMint0"
              },
              {
                "kind": "account",
                "path": "tokenMint1"
              }
            ]
          }
        },
        {
          "name": "tokenMint0",
          "docs": [
            "Token_0 mint, the key must be smaller then token_1 mint."
          ]
        },
        {
          "name": "tokenMint1",
          "docs": [
            "Token_1 mint"
          ]
        },
        {
          "name": "tokenVault0",
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
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "poolState"
              },
              {
                "kind": "account",
                "path": "tokenMint0"
              }
            ]
          }
        },
        {
          "name": "tokenVault1",
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
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "poolState"
              },
              {
                "kind": "account",
                "path": "tokenMint1"
              }
            ]
          }
        },
        {
          "name": "observationState",
          "docs": [
            "Initialize an account to store oracle observations"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  98,
                  115,
                  101,
                  114,
                  118,
                  97,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "poolState"
              }
            ]
          }
        },
        {
          "name": "tickArrayBitmap",
          "docs": [
            "Initialize an account to store if a tick array is initialized."
          ],
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
                  116,
                  105,
                  99,
                  107,
                  95,
                  97,
                  114,
                  114,
                  97,
                  121,
                  95,
                  98,
                  105,
                  116,
                  109,
                  97,
                  112,
                  95,
                  101,
                  120,
                  116,
                  101,
                  110,
                  115,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "poolState"
              }
            ]
          }
        },
        {
          "name": "tokenProgram0",
          "docs": [
            "Spl token program or token program 2022"
          ]
        },
        {
          "name": "tokenProgram1",
          "docs": [
            "Spl token program or token program 2022"
          ]
        },
        {
          "name": "systemProgram",
          "docs": [
            "To create a new program account"
          ],
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "rent",
          "docs": [
            "Sysvar for program account"
          ],
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "sqrtPriceX64",
          "type": "u128"
        },
        {
          "name": "openTime",
          "type": "u64"
        }
      ]
    },
    {
      "name": "createPoolNew",
      "discriminator": [
        82,
        73,
        224,
        134,
        134,
        56,
        120,
        149
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
    },
    {
      "name": "tickArrayBitmapExtension",
      "discriminator": [
        60,
        150,
        36,
        219,
        97,
        128,
        139,
        153
      ]
    }
  ],
  "events": [
    {
      "name": "collectProtocolFeeEvent",
      "discriminator": [
        206,
        87,
        17,
        79,
        45,
        41,
        213,
        61
      ]
    },
    {
      "name": "liquidityChangeEvent",
      "discriminator": [
        126,
        240,
        175,
        206,
        158,
        88,
        153,
        107
      ]
    },
    {
      "name": "poolCreatedEvent",
      "discriminator": [
        25,
        94,
        75,
        47,
        112,
        99,
        53,
        63
      ]
    },
    {
      "name": "swapEvent",
      "discriminator": [
        64,
        198,
        205,
        232,
        38,
        8,
        113,
        226
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "lok",
      "msg": "lok"
    },
    {
      "code": 6001,
      "name": "notApproved",
      "msg": "Not approved"
    },
    {
      "code": 6002,
      "name": "invalidUpdateConfigFlag",
      "msg": "invalid update amm config flag"
    },
    {
      "code": 6003,
      "name": "accountLack",
      "msg": "Account lack"
    },
    {
      "code": 6004,
      "name": "closePositionErr",
      "msg": "Remove liquitity, collect fees owed and reward then you can close position account"
    },
    {
      "code": 6005,
      "name": "zeroMintAmount",
      "msg": "Minting amount should be greater than 0"
    },
    {
      "code": 6006,
      "name": "invalidTickIndex",
      "msg": "Tick out of range"
    },
    {
      "code": 6007,
      "name": "tickInvalidOrder",
      "msg": "The lower tick must be below the upper tick"
    },
    {
      "code": 6008,
      "name": "tickLowerOverflow",
      "msg": "The tick must be greater, or equal to the minimum tick(-443636)"
    },
    {
      "code": 6009,
      "name": "tickUpperOverflow",
      "msg": "The tick must be lesser than, or equal to the maximum tick(443636)"
    },
    {
      "code": 6010,
      "name": "tickAndSpacingNotMatch",
      "msg": "tick % tick_spacing must be zero"
    },
    {
      "code": 6011,
      "name": "invalidTickArray",
      "msg": "Invalid tick array account"
    },
    {
      "code": 6012,
      "name": "invalidTickArrayBoundary",
      "msg": "Invalid tick array boundary"
    },
    {
      "code": 6013,
      "name": "sqrtPriceLimitOverflow",
      "msg": "Square root price limit overflow"
    },
    {
      "code": 6014,
      "name": "sqrtPriceX64",
      "msg": "sqrt_price_x64 out of range"
    },
    {
      "code": 6015,
      "name": "liquiditySubValueErr",
      "msg": "Liquidity sub delta L must be smaller than before"
    },
    {
      "code": 6016,
      "name": "liquidityAddValueErr",
      "msg": "Liquidity add delta L must be greater, or equal to before"
    },
    {
      "code": 6017,
      "name": "invalidLiquidity",
      "msg": "Invalid liquidity when update position"
    },
    {
      "code": 6018,
      "name": "forbidBothZeroForSupplyLiquidity",
      "msg": "Both token amount must not be zero while supply liquidity"
    },
    {
      "code": 6019,
      "name": "liquidityInsufficient",
      "msg": "Liquidity insufficient"
    },
    {
      "code": 6020,
      "name": "transactionTooOld",
      "msg": "Transaction too old"
    },
    {
      "code": 6021,
      "name": "priceSlippageCheck",
      "msg": "Price slippage check"
    },
    {
      "code": 6022,
      "name": "tooLittleOutputReceived",
      "msg": "Too little output received"
    },
    {
      "code": 6023,
      "name": "tooMuchInputPaid",
      "msg": "Too much input paid"
    },
    {
      "code": 6024,
      "name": "zeroAmountSpecified",
      "msg": "Swap special amount can not be zero"
    },
    {
      "code": 6025,
      "name": "invalidInputPoolVault",
      "msg": "Input pool vault is invalid"
    },
    {
      "code": 6026,
      "name": "tooSmallInputOrOutputAmount",
      "msg": "Swap input or output amount is too small"
    },
    {
      "code": 6027,
      "name": "notEnoughTickArrayAccount",
      "msg": "Not enought tick array account"
    },
    {
      "code": 6028,
      "name": "invalidFirstTickArrayAccount",
      "msg": "Invalid first tick array account"
    },
    {
      "code": 6029,
      "name": "invalidRewardIndex",
      "msg": "Invalid reward index"
    },
    {
      "code": 6030,
      "name": "fullRewardInfo",
      "msg": "The init reward token reach to the max"
    },
    {
      "code": 6031,
      "name": "rewardTokenAlreadyInUse",
      "msg": "The init reward token already in use"
    },
    {
      "code": 6032,
      "name": "exceptRewardMint",
      "msg": "The reward tokens must contain one of pool vault mint except the last reward"
    },
    {
      "code": 6033,
      "name": "invalidRewardInitParam",
      "msg": "Invalid reward init param"
    },
    {
      "code": 6034,
      "name": "invalidRewardDesiredAmount",
      "msg": "Invalid collect reward desired amount"
    },
    {
      "code": 6035,
      "name": "invalidRewardInputAccountNumber",
      "msg": "Invalid collect reward input account number"
    },
    {
      "code": 6036,
      "name": "invalidRewardPeriod",
      "msg": "Invalid reward period"
    },
    {
      "code": 6037,
      "name": "notApproveUpdateRewardEmissiones",
      "msg": "Modification of emissiones is allowed within 72 hours from the end of the previous cycle"
    },
    {
      "code": 6038,
      "name": "unInitializedRewardInfo",
      "msg": "uninitialized reward info"
    },
    {
      "code": 6039,
      "name": "notSupportMint",
      "msg": "Not support token_2022 mint extension"
    },
    {
      "code": 6040,
      "name": "missingTickArrayBitmapExtensionAccount",
      "msg": "Missing tickarray bitmap extension account"
    },
    {
      "code": 6041,
      "name": "insufficientLiquidityForDirection",
      "msg": "Insufficient liquidity for this direction"
    },
    {
      "code": 6042,
      "name": "maxTokenOverflow",
      "msg": "Max token overflow"
    },
    {
      "code": 6043,
      "name": "calculateOverflow",
      "msg": "Calculate overflow"
    },
    {
      "code": 6044,
      "name": "transferFeeCalculateNotMatch",
      "msg": "TransferFee calculate not match"
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
      "name": "collectProtocolFeeEvent",
      "docs": [
        "Emitted when the collected protocol fees are withdrawn by the factory owner"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "poolState",
            "docs": [
              "The pool whose protocol fee is collected"
            ],
            "type": "pubkey"
          },
          {
            "name": "recipientTokenAccount0",
            "docs": [
              "The address that receives the collected token_0 protocol fees"
            ],
            "type": "pubkey"
          },
          {
            "name": "recipientTokenAccount1",
            "docs": [
              "The address that receives the collected token_1 protocol fees"
            ],
            "type": "pubkey"
          },
          {
            "name": "amount0",
            "docs": [
              "The amount of token_0 protocol fees that is withdrawn"
            ],
            "type": "u64"
          },
          {
            "name": "amount1",
            "docs": [
              "The amount of token_0 protocol fees that is withdrawn"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "liquidityChangeEvent",
      "docs": [
        "Emitted pool liquidity change when increase and decrease liquidity"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "poolState",
            "docs": [
              "The pool for swap"
            ],
            "type": "pubkey"
          },
          {
            "name": "tick",
            "docs": [
              "The tick of the pool"
            ],
            "type": "i32"
          },
          {
            "name": "tickLower",
            "docs": [
              "The tick lower of position"
            ],
            "type": "i32"
          },
          {
            "name": "tickUpper",
            "docs": [
              "The tick lower of position"
            ],
            "type": "i32"
          },
          {
            "name": "liquidityBefore",
            "docs": [
              "The liquidity of the pool before liquidity change"
            ],
            "type": "u128"
          },
          {
            "name": "liquidityAfter",
            "docs": [
              "The liquidity of the pool after liquidity change"
            ],
            "type": "u128"
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
            "name": "tickCumulative",
            "docs": [
              "the cumulative of tick during the duration time"
            ],
            "type": "i64"
          },
          {
            "name": "padding",
            "docs": [
              "padding for feature update"
            ],
            "type": {
              "array": [
                "u64",
                4
              ]
            }
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
            "name": "recentEpoch",
            "docs": [
              "recent update epoch"
            ],
            "type": "u64"
          },
          {
            "name": "observationIndex",
            "docs": [
              "the most-recently updated index of the observations array"
            ],
            "type": "u16"
          },
          {
            "name": "poolId",
            "docs": [
              "belongs to which pool"
            ],
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
                100
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
                "u64",
                4
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
      "name": "poolCreatedEvent",
      "docs": [
        "Emitted when a pool is created and initialized with a starting price",
        ""
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenMint0",
            "docs": [
              "The first token of the pool by address sort order"
            ],
            "type": "pubkey"
          },
          {
            "name": "tokenMint1",
            "docs": [
              "The second token of the pool by address sort order"
            ],
            "type": "pubkey"
          },
          {
            "name": "tickSpacing",
            "docs": [
              "The minimum number of ticks between initialized ticks"
            ],
            "type": "u16"
          },
          {
            "name": "poolState",
            "docs": [
              "The address of the created pool"
            ],
            "type": "pubkey"
          },
          {
            "name": "sqrtPriceX64",
            "docs": [
              "The initial sqrt price of the pool, as a Q64.64"
            ],
            "type": "u128"
          },
          {
            "name": "tick",
            "docs": [
              "The initial tick of the pool, i.e. log base 1.0001 of the starting price of the pool"
            ],
            "type": "i32"
          },
          {
            "name": "tokenVault0",
            "docs": [
              "Vault of token_0"
            ],
            "type": "pubkey"
          },
          {
            "name": "tokenVault1",
            "docs": [
              "Vault of token_1"
            ],
            "type": "pubkey"
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
            "name": "padding3",
            "type": "u16"
          },
          {
            "name": "padding4",
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
            "name": "recentEpoch",
            "type": "u64"
          },
          {
            "name": "padding1",
            "type": {
              "array": [
                "u64",
                24
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
    },
    {
      "name": "swapEvent",
      "docs": [
        "Emitted by when a swap is performed for a pool"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "poolState",
            "docs": [
              "The pool for which token_0 and token_1 were swapped"
            ],
            "type": "pubkey"
          },
          {
            "name": "sender",
            "docs": [
              "The address that initiated the swap call, and that received the callback"
            ],
            "type": "pubkey"
          },
          {
            "name": "tokenAccount0",
            "docs": [
              "The payer token account in zero for one swaps, or the recipient token account",
              "in one for zero swaps"
            ],
            "type": "pubkey"
          },
          {
            "name": "tokenAccount1",
            "docs": [
              "The payer token account in one for zero swaps, or the recipient token account",
              "in zero for one swaps"
            ],
            "type": "pubkey"
          },
          {
            "name": "amount0",
            "docs": [
              "The real delta amount of the token_0 of the pool or user"
            ],
            "type": "u64"
          },
          {
            "name": "transferFee0",
            "docs": [
              "The transfer fee charged by the withheld_amount of the token_0"
            ],
            "type": "u64"
          },
          {
            "name": "amount1",
            "docs": [
              "The real delta of the token_1 of the pool or user"
            ],
            "type": "u64"
          },
          {
            "name": "transferFee1",
            "docs": [
              "The transfer fee charged by the withheld_amount of the token_1"
            ],
            "type": "u64"
          },
          {
            "name": "zeroForOne",
            "docs": [
              "if true, amount_0 is negtive and amount_1 is positive"
            ],
            "type": "bool"
          },
          {
            "name": "sqrtPriceX64",
            "docs": [
              "The sqrt(price) of the pool after the swap, as a Q64.64"
            ],
            "type": "u128"
          },
          {
            "name": "liquidity",
            "docs": [
              "The liquidity of the pool after the swap"
            ],
            "type": "u128"
          },
          {
            "name": "tick",
            "docs": [
              "The log base 1.0001 of price of the pool after the swap"
            ],
            "type": "i32"
          }
        ]
      }
    },
    {
      "name": "tickArrayBitmapExtension",
      "serialization": "bytemuckunsafe",
      "repr": {
        "kind": "c",
        "packed": true
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "poolId",
            "type": "pubkey"
          },
          {
            "name": "positiveTickArrayBitmap",
            "docs": [
              "Packed initialized tick array state for start_tick_index is positive"
            ],
            "type": {
              "array": [
                {
                  "array": [
                    "u64",
                    8
                  ]
                },
                14
              ]
            }
          },
          {
            "name": "negativeTickArrayBitmap",
            "docs": [
              "Packed initialized tick array state for start_tick_index is negitive"
            ],
            "type": {
              "array": [
                {
                  "array": [
                    "u64",
                    8
                  ]
                },
                14
              ]
            }
          }
        ]
      }
    }
  ]
};
