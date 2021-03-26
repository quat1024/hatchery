Keys and values seem to alternate. Whitespace appears to be significant to split unquoted keys from values but I'll have to check.

It is valid to repeat a key.

## `editoritems.txt`

* Keys and values are always quoted, even numeric values.
* There is a root block named `"ItemData"`.
* Inner arrays are serialized with various keys "0", "1", "2", "3" etc but the outer "ItemData" array is serialized by repeating the key `"Item" { ... }`. I guess it depends on the use case.

## `gameinfo.txt`

* Keys are unquoted.
* Numeric values are unquoted, as well as the values in SearchPaths.
* There is a root block named `"GameInfo"`, but all the other keys are unquoted.

## A p2c file

* Line-endings of multiline strings are not escaped. The string continues onto the next line with no indentation until a " is visible.

## A random probably-Puzzlemaker generated VMF file (`sdk_content/maps/1579680910.vmf`, idk which map this corresponds to)

* There is no root block.
* Keys that lead to blocks are unquoted, but the rest of the keys are quoted.

## A random probably-Hammer generated VMF file (`sdk_content/maps/instances/p2editor/and_gate.vmf`) from BEEMOD

* Parameters in the `connections` block are stored in one single string and separated using `U+001B ESCAPE`.

### Interesting datatypes

* Colors: `"0 183 193"`
* U and V axes: `"[1  0 0 0] 0.25"` (normalized vector + the scale of that vector or something)
* Planes: `"(-128 512 128) (0 512 128) (0 640 128)"` (3 points and the winding order is probably significant)

# Open questions

* Do you need a newline after every value
* Do you need a newline before every opening curly
* How important is the "pretty printing"
* What happens when I serialize a double-quote character in the puzzlemaker map description.