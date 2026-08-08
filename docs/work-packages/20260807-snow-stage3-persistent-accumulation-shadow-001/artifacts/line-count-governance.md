# Line-Count Governance

Status: pass with warnings

Evidence mode: Ran

Current key files: runoff_reconciliation.rs 1392; stage3_solver.rs 2868;
persistent test module 169; evaluation.rs 1389; runner day builder 2963;
authority adapter 796; trace formatter 304 lines.
The two 2000+ files are WARN, below the 3000-line closure block. Logic remains
split between evaluator, evaluation mechanics, builder, adapter, and trace
modules; further decomposition inside this bounded increment would increase
seam churn.
