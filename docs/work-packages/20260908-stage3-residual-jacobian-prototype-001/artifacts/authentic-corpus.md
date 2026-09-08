# Authentic residual corpus — superseded inactive-stem attempt

Ran: detached A-source-05 observation build, 2026-09-08 PDT. **Disposition:
INVALID for v33 derivative admission.** Both occupancies in all eight records
have exact-zero stem area, so the file contains no supported v33 stem column.
The FixedFinal records also do not yet retain the complete generic V3 context
needed to replay perturbed residuals. It remains failure/coverage evidence and
must not enter oracle, accuracy, cost, or performance denominators. This is untimed
Phase-A evidence and performs no additional scientific evaluation in the
measured path.

- External retained file: `/tmp/openwepp-rj-corpus-v1-combined.json`
- SHA-256: `b8f6a923065e7b396f84831ae6e3435cded05fae324b92d13c05ac646b783fec`
- Size: 600,588 bytes; schema `openwepp.covered-residual-corpus.v1`
- 8 records: first/last Potential = 2/2; first/last FixedFinal = 2/2.
- Every record: authentic N=2, S=6, D=29; exact-zero executed stem area. The
  historical covered-solver Potential records retain replay inputs; the generic
  V3 FixedFinal records retain x/base/frozen/caps but not their complete context.
- 5,338 f64 values encoded as tagged 16-hex u64 bits.
- Full native serialize/decode equality: PASS before write.
- Focused codec and mutation negatives: 4/4 PASS.
- Release ignored `stage3_untimed_residual_corpus_capture`: PASS 1/1, 5.06 s.
- Scoped rustfmt for changed capture files: PASS.

The first capture contained only historical Potential bases. Static diagnosis
showed actual V3 fixed-final construction uses the generic normalized Jacobian
path in `solver_litter_phase.rs`; the final observation hook clones its already
computed predecessor/trial/frozen/caps without evaluating again. The combined
file supersedes the Potential-only `/tmp/openwepp-rj-corpus-v1.json` for claims.

This attempt demonstrates an authentic inactive-stem workload only. A separately
identified positive-stem capture with complete replay inputs is required before
Phase A can exit.
