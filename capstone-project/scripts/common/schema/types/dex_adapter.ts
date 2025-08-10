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
      "name": "swapAndActivate",
      "docs": [
        "swap tokens and forward result to registry program (call receive_payment)"
      ],
      "discriminator": [
        211,
        229,
        13,
        51,
        221,
        165,
        179,
        242
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
          "name": "registryProgram"
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
          "name": "registryBump",
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
            ],
            "program": {
              "kind": "account",
              "path": "registryProgram"
            }
          }
        },
        {
          "name": "registryConfig",
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
            ],
            "program": {
              "kind": "account",
              "path": "registryProgram"
            }
          }
        },
        {
          "name": "registryUserId",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  105,
                  100
                ]
              },
              {
                "kind": "account",
                "path": "sender"
              }
            ],
            "program": {
              "kind": "account",
              "path": "registryProgram"
            }
          }
        },
        {
          "name": "inputTokenMint",
          "writable": true
        },
        {
          "name": "outputTokenMint",
          "writable": true
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
          "name": "revenueAppAta",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "registryConfig"
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
          "name": "inputTokenMint",
          "writable": true
        },
        {
          "name": "outputTokenMint",
          "writable": true
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
      "name": "daBump",
      "discriminator": [
        194,
        87,
        137,
        28,
        114,
        203,
        28,
        178
      ]
    },
    {
      "name": "daConfig",
      "discriminator": [
        35,
        43,
        191,
        197,
        211,
        171,
        233,
        201
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
    },
    {
      "name": "userId",
      "discriminator": [
        41,
        242,
        241,
        112,
        148,
        47,
        120,
        243
      ]
    }
  ],
  "types": [
    {
      "name": "assetItem",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "asset",
            "type": "pubkey"
          }
        ]
      }
    },
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
            "name": "userCounter",
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
            "name": "isPaused",
            "type": "bool"
          },
          {
            "name": "rotationTimeout",
            "type": "u32"
          },
          {
            "name": "registrationFee",
            "type": {
              "defined": {
                "name": "assetItem"
              }
            }
          },
          {
            "name": "dataSizeRange",
            "type": {
              "defined": {
                "name": "range"
              }
            }
          }
        ]
      }
    },
    {
      "name": "daBump",
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
      "name": "daConfig",
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
      "name": "range",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "min",
            "type": "u32"
          },
          {
            "name": "max",
            "type": "u32"
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
    },
    {
      "name": "userId",
      "docs": [
        "get by user: Pubkey"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u32"
          },
          {
            "name": "isOpen",
            "type": "bool"
          },
          {
            "name": "isActivated",
            "type": "bool"
          },
          {
            "name": "accountBump",
            "type": "u8"
          },
          {
            "name": "rotationStateBump",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
