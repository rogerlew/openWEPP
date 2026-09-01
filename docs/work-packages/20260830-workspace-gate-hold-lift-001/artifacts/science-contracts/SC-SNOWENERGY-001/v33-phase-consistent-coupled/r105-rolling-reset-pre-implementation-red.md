# R105 rolling exact transition reset pre-implementation red

Status: `CONTRACT_FIRST_EXPECTED_RED`

Retained canonical r105 `/tmp/wghl_001d_v39_64m_r105.log`, SHA-256
`6564e1addd0b8b63950ee48233bf308bbf704ce0935ce33ef1dcf61b814d4479`,
cleared all V39 transaction/custody seams and failed at exact support
`1860..1920 s` after 96 Picard evaluations. Static trace proved the V33 reset
predicate remains exact, but its root anchor is stale-first: after an
Interface-after-BranchEntry reset fails exact equality, the code neither
promotes the current validated interface nor clears the branch-entry window.

The amended `INV-SNOWENERGY-057` and `OBL-SNOWENERGY-C-025` require a rolling
exact window with no new tolerance. Source-bound obligations require
`v33_transition_window_rearms_stale_root_then_dispatches_exact_reset` and
`v33_transition_window_never_dispatches_nonexact_reset`; before focused
behavior tests are added, the source-bound gate is expected red.
