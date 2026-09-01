# V40 parity-monotone active-set pre-implementation red

Status: `CONTRACT_FIRST_EXPECTED_RED`

Retained r107 `/tmp/wghl_001d_v39_64m_r107.log`, SHA-256
`2fdfcb0c54e1845670f9d95a4b3770f3f43ab129e297901fc9e7b786dafa6c75`,
proved the corrected rolling exact reset dispatches only at shared budget 96:
`EvaluationBudget`, zero solver physical evaluations, and no private result.

`SC-SNOWENERGY-001@40` and the package now authorize only a four-window,
exact-static, parity-monotone earlier trigger for the unchanged physical
solver. The source-bound gate requires the typed reset/eligibility/observer
seams and five exact behavior/poison names. Before production implementation,
that gate is expected red; no V40 physics, tolerance, cap, acceptance, receipt,
rollback, or publication change is authorized.
