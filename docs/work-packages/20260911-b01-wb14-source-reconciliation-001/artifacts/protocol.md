# Frozen assessment predicates

Frozen before measurement on 2026-09-11 from package `20260911-b01-wb14-source-reconciliation-001`.

1. Actual-byte identity is SHA-256 over every `files` entry in the retained
   `build145-execution-identity.json`; its map digest is SHA-256 of canonical
   JSON (`sort_keys=True`, separators `(',', ':')`).  Expected identity is
   `b6fb949e967af911bd3d508c02e023bf0a349b30e3c0697631276b351b50e88f`.
2. Exact recovery requires all 927 entries and the expected map digest.  Any
   unavailable expected bytes remain UNKNOWN; an available byte difference is
   never treated as semantic equivalence.
3. Beyond-map inventory records every regular file and symlink under the
   retained root relative to the manifest, including Cargo/config/build/include
   and fixture paths.  Presence is identity information only, not execution
   evidence.
4. Static harness sufficiency for later cadence execution requires a registered,
   ignored fresh-process test with a fully qualified name and a source-visible
   direct ingress path for the day-4/interval-22/transaction-255 boundary. A
   generic B01 test, helper, or static name alone fails that execution predicate.
   Its absence may instead be stated as a precise prospective test-scope
   amendment in a revised-baseline proposal; it is not evidence of execution.
5. No Rust compilation, test, runtime, fixture authoring, cadence candidate patch
   application, or original-source modification is performed by this assessment.


Review correction: predicate 5 originally used unqualified “patch application”.
The initial protocol is retained as `protocol-initial.txt` in package artifacts.
The package always allowed fresh retained-recipe reconstruction; Astra directed
that additional comparison before it ran. R0/final016/correction145 recipe patches
were applied only to the new reconstruction copy. This correction clarifies the
execution-method statement; predicates 1–4 and acceptance thresholds are unchanged.
