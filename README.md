# AID
Official Aniwene ID system

## ID format

This repository provides a small, configurable ID system for generating and validating
human-friendly IDs up to 16 characters long. The default configuration:

- Uses only uppercase alphanumeric characters.
- Removes easily confused characters (I, O, L).
- Removes vowels to reduce accidental profanity.
- Enforces a maximum length of 16 characters.

See `src/lib.rs` for implementation details.

## Running

Generate a 16-character ID (default length):

```bash
cargo run
```

Generate an 8-character ID:

```bash
cargo run -- 8
```

## Zoho Creator (Deluge) example

Use this Deluge snippet in a custom field/workflow to generate an AID using the
same default alphabet and max length:

```deluge
alphabet = "23456789BCDFGHJKMNPQRSTVWXYZ";
length = 16;
aid = "";
for each  i in range(0, length - 1)
{
    idx = zoho.random.number(1, alphabet.length()) - 1;
    aid = aid + alphabet.subString(idx, idx + 1);
}
info aid;
```

## Creator info

- Creator: add your preferred name here.
- DOPR: add the intended meaning for DOPR here.
