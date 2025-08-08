/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/dex_adapter.json`.
 */
export type DexAdapter = {
  "address": "FMsjKKPk7FQb1B9H8UQTLrdCUZ9MaoAeTnNK9kdVJmtt",
  "metadata": {
    "name": "dexAdapter",
    "version": "1.0.0",
    "spec": "0.1.0",
    "description": "Created with Anchor",
    "repository": "https://github.com/M-Daeva/solana-boilerplate"
  },
  "instructions": [
    {
      "name": "init",
      "discriminator": [
        220,
        59,
        207,
        236,
        108,
        250,
        47,
        100
      ],
      "accounts": [
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "sender",
          "writable": true,
          "signer": true
        },
        {
          "name": "bump",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112
                ]
              }
            ]
          }
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "adminRotationState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  100,
                  109,
                  105,
                  110,
                  95,
                  114,
                  111,
                  116,
                  97,
                  116,
                  105,
                  111,
                  110,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "dex",
          "type": "pubkey"
        },
        {
          "name": "registry",
          "type": {
            "option": "pubkey"
          }
        },
        {
          "name": "rotationTimeout",
          "type": {
            "option": "u32"
          }
        },
        {
          "name": "tokenInWhitelist",
          "type": {
            "option": {
              "vec": "pubkey"
            }
          }
        }
      ]
    },
    {
      "name": "saveRoute",
      "discriminator": [
        159,
        32,
        189,
        85,
        230,
        5,
        208,
        143
      ],
      "accounts": [
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "sender",
          "writable": true,
          "signer": true
        },
        {
          "name": "bump",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112
                ]
              }
            ]
          }
        },
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "route",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  111,
                  117,
                  116,
                  101
                ]
              },
              {
                "kind": "arg",
                "path": "mintFirst"
              },
              {
                "kind": "arg",
                "path": "mintLast"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "mintFirst",
          "type": "pubkey"
        },
        {
          "name": "mintLast",
          "type": "pubkey"
        },
        {
          "name": "route",
          "type": {
            "vec": {
              "defined": {
                "name": "routeItem"
              }
            }
          }
        }
      ]
    },
    {
      "name": "swapMultihop",
      "discriminator": [
        138,
        70,
        253,
        6,
        221,
        75,
        252,
        147
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
          "name": "tokenProgram2022"
        },
        {
          "name": "memoProgram"
        },
        {
          "name": "clmmMockProgram"
        },
        {
          "name": "sender",
          "writable": true,
          "signer": true
        },
        {
          "name": "bump",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112
                ]
              }
            ]
          }
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "route",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  111,
                  117,
                  116,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "inputTokenMint"
              },
              {
                "kind": "account",
                "path": "outputTokenMint"
              }
            ]
          }
        },
        {
          "name": "inputTokenMint"
        },
        {
          "name": "outputTokenMint"
        },
        {
          "name": "inputTokenSenderAta",
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
                "path": "inputTokenMint"
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
          "name": "outputTokenSenderAta",
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
                "path": "outputTokenMint"
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
          "name": "inputTokenAppAta",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "config"
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
                "path": "inputTokenMint"
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
          "name": "outputTokenAppAta",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "config"
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
                "path": "outputTokenMint"
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
          "name": "amountIn",
          "type": "u64"
        },
        {
          "name": "amountOutMinimum",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "bump",
      "discriminator": [
        16,
        214,
        115,
        207,
        20,
        247,
        184,
        128
      ]
    },
    {
      "name": "config",
      "discriminator": [
        155,
        12,
        170,
        224,
        30,
        250,
        204,
        130
      ]
    },
    {
      "name": "rotationState",
      "discriminator": [
        173,
        83,
        106,
        140,
        2,
        64,
        93,
        114
      ]
    },
    {
      "name": "route",
      "discriminator": [
        80,
        179,
        58,
        115,
        52,
        19,
        146,
        134
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "unsortedMints",
      "msg": "Mints are unsorted!"
    },
    {
      "code": 6001,
      "name": "slippageExceeded",
      "msg": "Swap slippage exceeded maximum allowed"
    },
    {
      "code": 6002,
      "name": "invalidSwapRatio",
      "msg": "Invalid swap ratio configuration"
    },
    {
      "code": 6003,
      "name": "dexCallFailed",
      "msg": "DEX program call failed"
    },
    {
      "code": 6004,
      "name": "forwardingFailed",
      "msg": "Token forwarding failed"
    },
    {
      "code": 6005,
      "name": "contractPaused",
      "msg": "Contract is paused"
    },
    {
      "code": 6006,
      "name": "invalidRouteLength",
      "msg": "Route must contain at least 2 tokens"
    },
    {
      "code": 6007,
      "name": "invalidAmount",
      "msg": "Amount must be greater than 0"
    },
    {
      "code": 6008,
      "name": "invalidTokenAccount",
      "msg": "Invalid token account"
    },
    {
      "code": 6009,
      "name": "invalidRemainingAccounts",
      "msg": "Invalid number of remaining accounts"
    },
    {
      "code": 6010,
      "name": "noOutputTokens",
      "msg": "No output tokens received from swap"
    }
  ],
  "types": [
    {
      "name": "bump",
      "docs": [
        "to store bumps for all app accounts"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "config",
            "type": "u8"
          },
          {
            "name": "rotationState",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "config",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "docs": [
              "can update the config and execute priveled instructions"
            ],
            "type": "pubkey"
          },
          {
            "name": "dex",
            "type": "pubkey"
          },
          {
            "name": "registry",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "isPaused",
            "type": "bool"
          },
          {
            "name": "rotationTimeout",
            "type": "u32"
          },
          {
            "name": "tokenInWhitelist",
            "docs": [
              "list of supported SPL/Token2022 tokens"
            ],
            "type": {
              "vec": "pubkey"
            }
          }
        ]
      }
    },
    {
      "name": "rotationState",
      "docs": [
        "to transfer ownership from one address to another in 2 steps (for security reasons)"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "newOwner",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "expirationDate",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "route",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "value",
            "type": {
              "vec": {
                "defined": {
                  "name": "routeItem"
                }
              }
            }
          }
        ]
      }
    },
    {
      "name": "routeItem",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "ammIndex",
            "type": "u16"
          },
          {
            "name": "tokenOut",
            "type": "pubkey"
          }
        ]
      }
    }
  ]
};
