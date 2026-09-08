# Rust line-count governance

Ran: detached Phase-A capture/oracle source inventory. No candidate derivative
or production Rust was added. Relevant files are 364 lines
(`solver_residual_corpus_capture.rs`), 327 (`solver_stem_jacobian_tests.rs`),
1,909 (`solver_tests.rs`), 309 (`solver_litter_phase.rs`), and 2,685
(`solver_covered_evaluation.rs`). The evaluator is a pre-existing cohesive
canonical calculation module above the 2,000-line WARN threshold; this package
adds test-support serialization/access only and splitting it would expand the
scientific diff. No relevant file reaches the 3,000-line refactor threshold.
If a future authorized production derivative touches that evaluator, its package
must first extract the canonical residual/normalizer nodes into a cohesive module
and re-establish bit-identical primal evidence; this closed Phase-A package does
not carry that source split.
