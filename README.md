# stitchfinder

stitchfinder is a program which finds combinations of words which overlap with each other, and checks if the two words
without the overlapping segment are valid words.

`twinknight` is one such combination, which could mean "twink night" or "twin knight". Both possibilities make sense,
amount to the same text when the space is removed. This combination is what inspired the project, after I saw someone
named it in a Halo lobby.

There isn't much industrial purpose to this project. I do recommend using it to find cool words to use as usernames,
project names, and whatever else you want :)

## Usage

stitchfinder requires a list of valid words (these are called **found words**) and the word
to stitch with others, called the **given word**. It is invoked using:

```
stitchfinder <words file> <given word>
```

For example, the following is first 10 lines of `stitchfinder popular.txt twink`:

```
 Stitched                  Pos-given   Pos-expans   Valid   I-sect   Expansion   Found               Rem-expans   Rem-found        
 atwink                                right        false   t        twink       at                  wink         a                
 atwinkle                  left        right        false   t        twinkle     at                  winkle       a                
 abandonmentwink                       right        false   t        twink       abandonment         wink         abandonmen       
 abandonmentwinkle         left        right        false   t        twinkle     abandonment         winkle       abandonmen       
 abbotwink                             right        false   t        twink       abbot               wink         abbo             
 abbotwinkle               left        right        false   t        twinkle     abbot               winkle       abbo             
 abductwink                            right        false   t        twink       abduct              wink         abduc            
 abductwinkle              left        right        false   t        twinkle     abduct              winkle       abduc            
 abortwink                             right        false   t        twink       abort               wink         abor            
```

The meaning of the columns is:

- `Stitched`: the word which stiches `Expansion` and `Found` together, with the overlap being `I-sect`
- `Pos-given` (position of given): where the given word is relative to `Expansion`. Blank when the expansion is the given word.
- `Pos-expans` (position of expansion): where `Expansion` is relative to `Found` in `Stitched`.
- `Valid`: whether the `Rem-expans` and `Rem-found` are valid words (i.e. are they in `popular.txt`)
- `I-sect` (intersection): the overlapping text between `Expansion` and `Found`
- `Expansion`: what the given word expanded to. May be the same as the given word.
- `Found`: the other word
- `Rem-given` (remaining given): text of the given word without `I-sect`
- `Rem-found` (remaining found): text of `Found` without `I-sect`

For more information about `Expansion` and `Pos-given`, see [Expansion](#expansion).

### With Nushell

Although stitchfinder does not require [Nushell](https://www.nushell.sh/), it makes filtering the output much easier.
The following command provides a neat table:

```
stitchfinder | detect columns | update Valid { into bool }
```

This allows the use of table commands like `where`, `sort-by`, `filter`, and `each`. Maybe eventually there will
be a build that interacts transparently with Nushell.

## Words File

The list of valid words must be a text file with a word on each line. This repository provides [big.txt](big.txt) and
[popular.txt](popular.txt) for the sake of convenience.

- [big.txt](big.txt) was obtained from [this repository file](https://github.com/dwyl/english-words/blob/a77cb15f4f5beb59c15b945f2415328a6b33c3b0/words.txt)
and any words with non-alphabetical characters were removed.
- [popular.txt](popular.txt) was obtained from [this repository file](https://github.com/dolph/dictionary/blob/c65f04b0b5b27a981f437b940cf62fe71320d5ec/popular.txt)
and had no filtering applied (there aren't any symbols to filter).

## Expansion

Expansion is the first step of stitchfinder, and may be disabled with `--disable-expansion`. It uses the
given word as-is, but also and expands it into as many found words as possible, and then tries to stitch
using all of those words.

For example, let `ia` be the given word. `ia` expands into:

- `iambic` from the left side, which stitches with
	- `insignia` from the left side to make `insigniambic`
	- `bicycle` from the right side to make `iambicycle`
- `aria` from the right side, which stitches with
	- `avatar` from the left to make `avataria`
	- `iambic` from the right to make `ariambic`

Thus, the output includes:

```
 Stitched                        Pos-given   Pos-expans   Valid   I-sect   Expansion        Found               Rem-expans      Rem-found        
 ariambic                        left        right        false   ia       iambic           aria                mbic            ar               
 ariambic                        right       left         false   ia       aria             iambic              ar              mbic             
 avataria                        right       right        false   ar       aria             avatar              ia              avat             
 iambicycle                      left        left         false   bic      iambic           bicycle             iam             ycle             
 iambicycle                      left        left         false   c        iambic           cycle               iambi           ycle             
 insigniambic                    left        right        false   ia       iambic           insignia            mbic            insign           
 insigniambic                    right       left         false   ia       insignia         iambic              insign          mbic             
```

Notice the two columns `Expansion` and `Pos-given`

- `Expansion` corresponds to what the given word expanded into.
- `Pos-given` corresponds to where the given word is within the expansion.

In the first example, `Expansion` says that `ia` expanded into `iambic`. As stated earlier, it expands from the left side. This side matches with that of `Pos-given`.  
Similarly, the second row has an `Expansion` of `aria`, which comes from the right side. This aligns with the tree from earlier and what `Pos-given` says.

However, not every word can be expanded. When a row does not use expansion, i.e. it uses the given word as-is, `Pos-given` is blank and `Expansion` is the same as the
given word. 

## Duplicate Stitched Words

While the `Stitched` column is sorted, it may have duplicates. For example, the following is included in the output of `stitchfinder popular.txt ash`:

```
 Stitched                    Pos-given   Pos-expans   Valid   I-sect   Expansion   Found               Rem-expans   Rem-found        
 sodash                      right       right        false   das      dash        sodas               h            so               
 sodash                      right       right        true    d        dash        sod                 ash          so               
 sodash                      right       right        true    da       dash        soda                sh           so               
 sodash                                  right        false   as       ash         sodas               h            sod              
 sodash                                  right        true    a        ash         soda                sh           sod              
```

`sodash` can be formed from many different stitches. Even `--disable-expansion`, the existence of both `soda` + `ash` and `sodas` + `ash` causes there to be two entries
for the same stitched word. 

## TODO

These might be ambitious, considering the project was done in 5 hours past midnight.

### Nushell Feature Flag

This program was made with Nushell in mind. It would benefit from a feature flag which allows it to interact directly with Nushell, providing argument autocompletion and proper table output.
