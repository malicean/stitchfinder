# stitchfinder

stitchfinder is a program which finds combinations of words which overlap with each other. It also checks if the two words,
without the overlapping segment, are valid words. One such combination is `twinknight`. `twinknight` could mean "twink night"
or "twin knight", both of which make sense but amount to the same text when the space is removed.

## Usage

stitchfinder requires a list of valid words (see [Words File](#words-file), these are called **found words**) and the word
to stitch with others, called the **given word**. It is invoked using:

```
stitchfinder <words file> <given word>
```

Invoking stitchfinder produces a table. Each row corresponds to a stitch, where the first column contains the **stitch word**.
Note that there might be multiple rows with the same stitch word; a stitch word can sometimes be produced multiple times from
the same given word but different found words, e.g. `ali` produces `alice` with `ice` or `lice`.
Also note that the stitch word might be the same as the given or found word; a found word may start/end with the given word
and vice versa.

### Example Output

The following is first 10 lines of `stitchfinder popular.txt twink`:

```
Stitched                  Found               I-sect   Rem-given   Rem-found          Pos     Valid   X-given   X-pos 
atwink                    at                  t        wink        a                  right   false                   
atwinkle                  at                  t        winkle      a                  right   false   twinkle   left  
abandonmentwink           abandonment         t        wink        abandonmen         right   false                   
abandonmentwinkle         abandonment         t        winkle      abandonmen         right   false   twinkle   left  
abbotwink                 abbot               t        wink        abbo               right   false                   
abbotwinkle               abbot               t        winkle      abbo               right   false   twinkle   left  
abductwink                abduct              t        wink        abduc              right   false                   
abductwinkle              abduct              t        winkle      abduc              right   false   twinkle   left  
abortwink                 abort               t        wink        abor               right   false                   
```

The meaning of the columns is:

- `Stitched`: stitched (final) words
- `Found`: words from `popular.txt` which stitch into the given word (`twink`)
- `I-sect` (intersection): text which the given word and each final word has, which allows the two to stitch together
- `Rem-given` (remaining given): text of the given word without the intersection
- `Rem-found` (remaining found): text of the found word without the intersection
- `Pos` (position): where the given word is relative to the found word
- `Valid`: whether the remaining given and found are valid words (i.e. are they in `popular.txt`)
- `X-given` (expanded given): see [Expansion](#expansion)
- `X-pos` (expanded position): see [Expansion](#expansion)

### With Nushell

Although stitchfinder does not require [Nushell](https://www.nushell.sh/), it makes filtering the output much easier.
The following command provides a neat table:

```
stitchfinder | detect columns | update Valid { into bool }
```

This allows the use of table commands like `where`, `sort-by`, `filter`, and `each`. Maybe eventually there will
be a build that interacts transparently with Nushell.

## Words File

The list of valid words must be a text file with a word on each line. This repository provides [big.txt] and
[popular.txt] for the sake of convenience.

- [big.txt] was obtained from [this repository file](https://github.com/dwyl/english-words/blob/a77cb15f4f5beb59c15b945f2415328a6b33c3b0/words.txt)
and any words with non-alphabetical characters were removed.
- [popular.txt] was obtained from [this repository file](https://github.com/dolph/dictionary/blob/c65f04b0b5b27a981f437b940cf62fe71320d5ec/popular.txt)
and had no filtering applied (there aren't any symbols to filter).

## Expansion

Expansion is a somewhat-complex feature of stitchfinder. It takes the given word and expands it into
other, valid words, and then uses these expanded words as their own given words.  

For example, let `ia` be the given word. `ia` expands into:

- `iambic` from the left side, which stitches with
	- `insignia` from the left side to make `insigniambic`
	- `bicycle` from the right side to make `iambicycle`
- `maria` from the right side, which stitches with
	- `avatar` from the left to make `avataria`
	- `iambic` from the right to make `ariambic`

Thus, the output includes:

```
Stitched                        Found               I-sect   Rem-given       Rem-found          Pos     Valid   X-given          X-pos
ariambic                        aria                ia       mbic            ar                 right   false   iambic           left  
ariambic                        iambic              ia       ar              mbic               left    false   aria             right 
avataria                        avatar              ar       ia              avat               right   false   aria             right 
iambicycle                      bicycle             bic      iam             ycle               left    false   iambic           left  
iambicycle                      cycle               c        iambi           ycle               left    false   iambic           left  
insigniambic                    insignia            ia       mbic            insign             right   false   iambic           left  
insigniambic                    iambic              ia       insign          mbic               left    false   insignia         right
```

Notice the two new columns: `X-given` and `X-pos`.

- `X-given` corresponds to what the given word was expanded into. For the first two lines, `ia` expanded into `iambic` and `aria`.
- `X-pos` corresponds to which side of the expanded word the given word is on. For the first line, `ia` expanded to `iambic` from
the left side, thus `X-pos` is `left`. In the second line, `ia` expanded to `aria` from the right side, so `X-pos` is `right`.

If `X-given` and `X-pos` are blank, it means no expansion occured, i.e. the given word was used as-is.

## Duplicate Stitched Words

While the `Stitched` column is sorted, it may have duplicates. For example, the following is the output of `stitchfinder popular.txt ash`:

```
Stitched                    Found               I-sect   Rem-given   Rem-found          Pos     Valid   X-given     X-pos
sodash                      sod                 d        ash         so                 right   true    dash        right 
sodash                      soda                da       sh          so                 right   true    dash        right 
sodash                      sodas               das      h           so                 right   false   dash        right 
sodash                      soda                a        sh          sod                right   true                      
sodash                      sodas               as       h           sod                right   false                     
```

Even without expansion, `sodash` can be formed from `soda` + `ash` and `sodas` + `ash`, thus creating two entries.

## TODO

These might be ambitious, considering the project was done in 5 hours past midnight.

### Nushell Feature Flag

This program was made with Nushell in mind. It would benefit from a feature flag which allows it to interact directly with Nushell, providing argument autocompletion and proper table output.
