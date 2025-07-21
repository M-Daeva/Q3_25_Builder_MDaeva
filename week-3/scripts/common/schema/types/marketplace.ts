/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/marketplace.json`.
 */
export type Marketplace = {
  "address": "DWTez7iNT13kaqEEZfzFQb5Msv7JdF9EFKRize99wQTi",
  "metadata": {
    "name": "marketplace",
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
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "marketplace",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  97,
                  114,
                  107,
                  101,
                  116,
                  112,
                  108,
                  97,
                  99,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "admin"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "feeBps",
          "type": "u16"
        },
        {
          "name": "collectionWhitelist",
          "type": {
            "vec": "pubkey"
          }
        },
        {
          "name": "assetWhitelist",
          "type": {
            "vec": {
              "defined": {
                "name": "asset"
              }
            }
          }
        },
        {
          "name": "name",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "marketplace",
      "discriminator": [
        70,
        222,
        41,
        62,
        78,
        3,
        32,
        174
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "unathorized",
      "msg": "Incorrect sender!"
    }
  ],
  "types": [
    {
      "name": "asset",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "sol"
          },
          {
            "name": "mint",
            "fields": [
              "pubkey"
            ]
          }
        ]
      }
    },
    {
      "name": "marketplace",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "admin",
            "docs": [
              "The wallet address of the marketplace administrator/authority"
            ],
            "type": "pubkey"
          },
          {
            "name": "feeBps",
            "docs": [
              "The marketplace fee percentage in basis points (e.g., 250 = 2.5%)"
            ],
            "type": "u16"
          },
          {
            "name": "collectionWhitelist",
            "type": {
              "vec": "pubkey"
            }
          },
          {
            "name": "assetWhitelist",
            "type": {
              "vec": {
                "defined": {
                  "name": "asset"
                }
              }
            }
          },
          {
            "name": "name",
            "docs": [
              "The name of the marketplace used for branding and identification"
            ],
            "type": "string"
          }
        ]
      }
    }
  ]
};
