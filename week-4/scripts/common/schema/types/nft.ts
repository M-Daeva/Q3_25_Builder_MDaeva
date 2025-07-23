/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/nft.json`.
 */
export type Nft = {
  "address": "5JFttyrKH4mHkNDU6D5YDRo3FQs1RRCqq4HHy1Rn7msF",
  "metadata": {
    "name": "nft",
    "version": "1.0.0",
    "spec": "0.1.0",
    "description": "Created with Anchor",
    "repository": "https://github.com/M-Daeva/solana-boilerplate"
  },
  "instructions": [
    {
      "name": "createCollection",
      "discriminator": [
        156,
        251,
        92,
        54,
        233,
        2,
        16,
        82
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
          "name": "collection",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  108,
                  108,
                  101,
                  99,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "admin"
              },
              {
                "kind": "arg",
                "path": "id"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "id",
          "type": "u8"
        },
        {
          "name": "metadata",
          "type": "string"
        }
      ]
    },
    {
      "name": "mintToken",
      "discriminator": [
        172,
        137,
        183,
        14,
        207,
        110,
        234,
        56
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
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "recipient"
        },
        {
          "name": "collection",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  108,
                  108,
                  101,
                  99,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "admin"
              },
              {
                "kind": "arg",
                "path": "id"
              }
            ]
          }
        },
        {
          "name": "token",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  111,
                  107,
                  101,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "collection.address",
                "account": "collection"
              },
              {
                "kind": "account",
                "path": "collection.next_token_id",
                "account": "collection"
              }
            ]
          }
        },
        {
          "name": "mint",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  105,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "collection.address",
                "account": "collection"
              },
              {
                "kind": "account",
                "path": "collection.next_token_id",
                "account": "collection"
              }
            ]
          }
        },
        {
          "name": "appMintAta",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "collection"
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
                "path": "mint"
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
          "name": "recipientMintAta",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "recipient"
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
                "path": "mint"
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
          "name": "id",
          "type": "u8"
        },
        {
          "name": "metadata",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "collection",
      "discriminator": [
        48,
        160,
        232,
        205,
        191,
        207,
        26,
        141
      ]
    },
    {
      "name": "token",
      "discriminator": [
        131,
        254,
        39,
        144,
        4,
        179,
        134,
        127
      ]
    }
  ],
  "types": [
    {
      "name": "collection",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "id",
            "type": "u8"
          },
          {
            "name": "nextTokenId",
            "type": "u16"
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "address",
            "type": "pubkey"
          },
          {
            "name": "metadata",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "token",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenBump",
            "type": "u8"
          },
          {
            "name": "mintBump",
            "type": "u8"
          },
          {
            "name": "id",
            "type": "u16"
          },
          {
            "name": "collection",
            "type": "pubkey"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "metadata",
            "type": "string"
          }
        ]
      }
    }
  ]
};
