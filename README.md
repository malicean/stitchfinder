# stitchfinder

stitchfinder is a program which finds combinations of words which overlap with each other to create double meanings.
One such combination is `twinknight`. `twinknight` could mean "twink night" or "twin knight", both of which
make sense but amount to the same text when the space is removed.

## Usage

stitchfinder requires a list of valid words (see [Words File](#words-file), these are called **found words**) and the word to stitch with others, called the **given word**. It is invoked using:

```
stitchfinder <words file> <given word>
```

Invoking stitchfinder produces a table. Each row corresponds to a stitch, where the first column contains the **stitch word**.
Note that there might be multiple rows with the same stitch word; a stitch word can sometimes be produced multiple times from the same given word but different found words, e.g. `ali` produces `alice` with `ice` or `lice`.
Also note that the stitch word might be the same as the given or found word; a found word may start/end with the given word and vice versa.

### Example Output

The following is the result of running `stitchfinder popular.txt twink`:

```
Stitched                Found               I-sect   Rem-given   Rem-found          Position   Valid 
atwink                  at                  t        wink        a                  right      false 
abandonmentwink         abandonment         t        wink        abandonmen         right      false 
abbotwink               abbot               t        wink        abbo               right      false 
abductwink              abduct              t        wink        abduc              right      false 
abortwink               abort               t        wink        abor               right      false 
aboutwink               about               t        wink        abou               right      false 
abreastwink             abreast             t        wink        abreas             right      false 
abruptwink              abrupt              t        wink        abrup              right      false 
absentwink              absent              t        wink        absen              right      false 
absorbentwink           absorbent           t        wink        absorben           right      false 
abstractwink            abstract            t        wink        abstrac            right      false 
```

### Nushell Interop

Although stitchfinder does not require [Nushell](https://www.nushell.sh/), it makes filtering the output much easier. The following command provides a neat table:

```
stitchfinder | detect columns | update Valid { into bool }
```

### Words File

The list of valid words must be a text file with a word on each line. This repository provides [big.txt] and [popular.txt] for the sake of convenience.

- [big.txt] was obtained from [this repository file](https://github.com/dwyl/english-words/blob/a77cb15f4f5beb59c15b945f2415328a6b33c3b0/words.txt)
and any words with non-alphabetical characters were removed.
- [popular.txt] was obtained from [this repository file](https://github.com/dolph/dictionary/blob/c65f04b0b5b27a981f437b940cf62fe71320d5ec/popular.txt)
and had no filtering applied (there aren't any symbols to filter).

stitchfinder will print a table containing the valid stitches in the first column and useful information in the rest, like the amount of overlap between words.
With the example of `twink`:

## TODO

These might be ambitious, considering the project was done in 5 hours past midnight.

### Extrapolation mode

This should find all words which start/end with the given word, and then run stitchfinder using each of these words as a given word.
The position column should then have an option for `mid` if the original given word is located in the the intersection of two other words.

For example, `ia` provides:

- `aria`, which stitches with
  - `avatar` to make `avataria` (ends with `ia`)
  - `iambic` to make `ariambic` (intersects with `ia`)
- `iambic`, which stitches with
	- `bicycle` to make `iambicycle` (starts with `ia`)
	- `insignia` to make `insigniambic` (intersects with `ia`)
	- `aria` to make `ariambic` (intersects with `ia` \[notice: intersections will duplicate\])

### Nushell Feature Flag

This program was made with Nushell in mind. It would benefit from a feature flag which allows it to interact directly with Nushell, providing argument autocompletion and proper table output.
