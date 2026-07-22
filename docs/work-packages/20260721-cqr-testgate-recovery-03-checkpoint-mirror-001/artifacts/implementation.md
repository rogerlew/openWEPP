# Implementation

Static: characterization commit `d51a6bc8` precedes production decomposition.
Implementation commit `424a1a5c` extracted configured-root admission, canonical
root preparation, alias validation, output copying, checkpoint publication, and
directory-component helpers.

Dual review found two behavior drifts: source reads used the canonical artifact
root rather than the caller's lexical root, and RootDir gained a metadata lookup.
Correction commit `d5af6207` retains the borrowed caller path for reads and makes
RootDir return a skip signal consumed by the directory loop. Direct regressions
bind both corrections. The final order, bytes, canonical JSON, and errors match
the pre-refactor source.
