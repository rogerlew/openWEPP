# Contract and provenance evidence

Status: PASS, final authority and provenance verified
Evidence mode: Static and Ran

Static: amended `SC-INFILE-IRRIGATION-FIXEDDATE-001` to version `0.1.2` with
`INV-FDIR-015`, a uniform `FDIR-E-005` finite-token guard for `datver`,
`irint`, `irdept`, `nozzle`, `qspply`, `tstart`, `tend`, and `tdepl`, and A-H
test-vector obligations. The input spec is version `0.1.1` with matching field
constraints and typed failure behavior.

Static: every touched legacy anchor now names
`/workdir/wepp-forest_260430_baseline` and commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. Inspection of pinned
`infile.for:2133-2167` and `irrig.for:263-338` confirms the source tokens and
legacy compatibility branches; finite-value rejection is openWEPP's canonical
typed domain invariant, not a claim that Fortran rejected IEEE spellings.

Ran: `markdown-doc lint` on the contract and spec passed: 2 files, 0 errors,
0 warnings.

Final-review hashes:

- contract: `8ab23208b0e7c9c6c7ef221c1f4f7a4698c1d235c7ce386acec54b920523df64`
- spec: `306b97c88ea04cb157094aff69a5d16b070d4369393287c6b9ce1f55b0b6b192`
