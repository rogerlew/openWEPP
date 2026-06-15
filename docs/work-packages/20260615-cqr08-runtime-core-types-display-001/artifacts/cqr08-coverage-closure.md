# Coverage Closure

Ran: target LCOV before refactor:

| Counter | Value |
| --- | ---: |
| `FNF` | 2 |
| `FNH` | 1 |
| `LF` | 425 |
| `LH` | 24 |

Ran: target LCOV after refactor:

| Counter | Value |
| --- | ---: |
| `FNF` | 20 |
| `FNH` | 20 |
| `LF` | 515 |
| `LH` | 497 |

Ran: target line coverage improved from `24/425` to `497/515`.

Ran: target function coverage improved from `1/2` to `20/20`.

Ran: `cargo llvm-cov` emitted its existing warning that 124 source files had no
matching LCOV entry. The target source file was present in both LCOV captures,
and `cargo crap` consumed both captures successfully with exit `0`.

Disposition: target coverage closure is sufficient for this behavior-preserving
code-quality package.
