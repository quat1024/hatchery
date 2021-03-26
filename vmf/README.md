# vmf

Serde for Valve's funny key-value store used across many of their products. Notably used in Hammer `.vmf` map files, but also in Portal 2 `editoritems.txt` and `.p2c` files, `gameinfo.txt` files, all sorts of things. I think it's a bit ad-hoc over there.

## Progress

* Ser: Basic data types and structs only. Always quotes keys and values.
* De: Not implemented.

## Todo

Maybe rename it, since this format is sometimes called `vdf` for Valve's Data Format. There's also a serde implementation by boringcactus on crates.io that i could help out with (or crib from)

## Problems

There's no array format per-se, but many arrays are encoded using a repeated key (e.g. hammer `.vmf` files include each entity under a top-level `entity` block, `devtest.p2c` has a `Voxels` entry with repeated `Voxel` keys, `gameinfo.txt` has repeated `Game` entries, etc). But that's not always the case (`editoritems.txt` variants are encoded with `"0"` `"1"` `"2"` keys)! It'd be nice to get that keying information into Serde somehow, so you don't have to write a whack ser/de implementation whenever you want to read and write a `Vec`.