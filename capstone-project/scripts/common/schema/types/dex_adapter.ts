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
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "slippageExceeded",
      "msg": "Swap slippage exceeded maximum allowed"
    },
    {
      "code": 6001,
      "name": "invalidSwapRatio",
      "msg": "Invalid swap ratio configuration"
    },
    {
      "code": 6002,
      "name": "dexCallFailed",
      "msg": "DEX program call failed"
    },
    {
      "code": 6003,
      "name": "forwardingFailed",
      "msg": "Token forwarding failed"
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
    }
  ]
};
