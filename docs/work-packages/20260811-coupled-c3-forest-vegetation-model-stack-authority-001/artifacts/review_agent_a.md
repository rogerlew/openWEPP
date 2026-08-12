# Independent Science Review A — Final Source/Lifecycle Exact-Byte Recheck

Status: `PASS / no material findings`

Evidence mode: `Static + Ran`

Review role: independent radiation, interception, energy, gas-exchange,
aerodynamic, hydraulic, water-arbitration, and numerical reviewer.

## Exact Reviewed Snapshot

- Final source-identity review instant: `2026-08-11T23:12:30Z`.
- Repository HEAD: `669aafb60df3ac4eeed2661cc4db4ad33f3f2265`.
- Tracked binary-diff SHA-256:
  `bc193255bb858eb04f7c687ab7c2897ac8d2633ad07f3fa264e4de65709b2c6c`.
- `git status --porcelain=v1` SHA-256:
  `496a451812655169952e94f80615ab9a3b40bc2fd2f2b9798b6a17d2b66a88d9`.
- The status contained documentation, references, contract-derived tests, and
  package files; no production crate/runtime file appeared.

| Reviewed surface | SHA-256 |
|---|---|
| `SC-VEGETATION-001.md` | `2c8cc8322ce8c4404e212f3a12f7f2aea7547ec8f23ab3eafc5e53f7672127e7` |
| `SC-BIOGEOCHEM-001.md` | `6cfd2143f9941613e6f6324d2790f88773c9b9eafa1ab8cad72e5a95df6794b4` |
| science-contract index | `5e926ff5f46df0ea02d44e176ae764034e009ff7f14f8841c4f308485b93a337` |
| `references/annotated_bibliography.md` | `587b629d995b3c9312f4adcb10933b91104b158f80ceb7c9e173f829f882f959` |
| `artifacts/reference-acquisition-ledger.md` | `b1deb70b77c8312d70e26b9bae09a8f6001105d61a2b784a500c3bb1db7f1508` |
| `artifacts/reference-rights-and-checksum-disposition.md` | `9a28c5568fcc53eae6ceb2cdbdf6650fbb1580e7d1802b5053b939a778e32df7` |
| `artifacts/equation-authority-ledger.md` | `5ec5a24893c8d2b0516e764de2164ff027dbd331cafe885c01e27fe48e9bddb2` |
| `artifacts/numerical-solver-and-convergence-contract.md` | `91b7241132cc05b0feed4d525f056e093e0cb012632cba699ac93efe904b3a0a` |
| `artifacts/parameter-and-configuration-manifest.md` | `5ed13860c21dce30f4f0b09393bb70ea6f18304b3210a957f4925d3ef1c64e99` |
| `artifacts/state-ownership-and-transaction-ledger.md` | `f7e0c47eb7cdc17d20a1c9e9f22c46c06584390f5b422dce02534881b7fe19e1` |
| `artifacts/openwepp_c3_woody_v1_definition.json` | `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157` |
| `artifacts/reference_calculator.py` | `ac8caf95e2b8bccadc528e168d0e466504bca88c15e86b7bfba89438f4ec13e8` |
| `artifacts/test-vector-ledger.md` | `b341510cf168506764c0e45c6c2dbec2d818da2cc2aba8fb2f8afd71d2448ce6` |
| `artifacts/review-finding-disposition.md` | `ee5dc49ca88aa1c76aae75365442e65e775b48981733ac45d4833a9aada2972a` |
| `tests/integration/vegetation_boundary_authority_contract.rs` | `7aeb2201bec9fdf078b114ceb569ed6fad7b3d8f9d03c76c1fd21647dcb658b3` |
| reviewed CLM5 technical note | `9ca0f0e5b7aff712a0ef7f5198f111c4b250cac4417a4f000e36c6c143f2e363` |

## Commands And Direct Results

- Ran `.venv/bin/python .../artifacts/reference_calculator.py`: exit 0,
  `"all_pass": true`; output SHA-256
  `bd180d63e8d4e3ccae78fbeec308ddd27024db9c466e1d8eef47656d6df0f368`.
- Ran `cargo nextest run --test vegetation_boundary_authority_contract
  --profile quick`: 12 passed, 0 failed; run ID
  `7600958a-8338-4690-aa59-209c03491b50`; elapsed 3.311 seconds.
- Ran `git diff --check`: PASS.
- Ran DOI content negotiation against the Crossref DOI registry for
  `10.1111/j.1365-3040.1997.00094.x`: it returned de Pury and Farquhar,
  *Plant, Cell & Environment* 20 (1997), pp. 537--557, with the exact title
  recorded in the bibliography.
- Recomputed SHA-256 of the local reviewed PDF:
  `8a847133cf3d546bccd3e2dc076fa3b1e5e6f71edf2dd2efcc32282f3fc41fc6`;
  `pdfinfo` reported 21 unencrypted PDF pages, and extracted journal pages
  537--539 confirmed title, authors, journal, volume, year, pagination, and
  direct/diffuse sunlit/shaded subject matter.
- Imported the exact calculator and independently exercised the coupled
  gas/energy/hydraulic, integrated wet/dry canopy, radiation, active-cap,
  dry/frozen, rollback, and floor-nondonation paths. The aggregate calculator
  returned no false check.

Selected discriminating results were:

```text
gas/energy transpiration       4.677543030368935e-5 kg m-2 s-1
hydraulic transpiration        4.677543030368933e-5 kg m-2 s-1
shared-flux residual           2.032879073410321e-20 kg m-2 s-1
one-pass hydraulic poison      4.683184809558713e-5 kg m-2 s-1
hydraulic factor beta_hyd      0.997334886752856

integrated wet-canopy dt       1800 s
store start / evaporation      0.03 / 0.030000000000000002 kg m-2
water closure                  0
latent energy                  75030 J m-2
combined energy residual       9.160672220787092e-10 J m-2
shortwave partition            500 W m-2

black direct-only absorption:
  total                        564.5003016127 W m-2
  sunlit                       564.5003014789 W m-2
  shaded                       1.3378e-7 W m-2
```

## Science Assessment

### Lifecycle and digest recheck

PASS. `SC-VEGETATION-001@5` and `SC-BIOGEOCHEM-001@1` are consistently
`approved / active` in frontmatter, rendered status, and the canonical index.
The BGC whole-file identity and model-definition JSON were rebound together;
the focused contract test independently recomputed the current definition,
contract-section, and BGC digests and passed. The equation, numerical,
parameter, state, and reference-calculator science surfaces retained their
reviewed meanings. The calculator SHA-256 remains
`ac8caf95e2b8bccadc528e168d0e466504bca88c15e86b7bfba89438f4ec13e8`,
and its byte-for-byte output remains
`bd180d63e8d4e3ccae78fbeec308ddd27024db9c466e1d8eef47656d6df0f368`.
The lifecycle promotion and two Rust helper cleanups therefore do not
invalidate the prior science assessment.

### Source identity and locator recheck

PASS. The corrected DOI
`10.1111/j.1365-3040.1997.00094.x` resolves through the DOI registry to the
same de Pury and Farquhar (1997) article represented by the exact local bytes.
The canonical contract, annotated bibliography, acquisition ledger, rights
ledger, and equation ledger consistently identify its sunlit/shaded canopy
role. The bibliography records the complete citation, acquisition route,
restricted-rights disposition, local path, exact checksum, and supporting
locators: journal pp. 539--543, Table 1 equations 1--14, Table 2, and Appendix
1 Tables A1--A2. The local artifact remains only under gitignored
`references/copyrighted/`; no restricted full text is committed. The DOI
correction changes source identity metadata, not the selected equation,
algorithm, parameter, state, numerical, or oracle science surfaces.

### Radiation and canopy scaling

PASS. The canonical two-stream sign/coefficient convention, defining
integrals, typed leaf-angle domain, removable `chi=0` and zero-scattering
branches, zero-direct/zero-zenith behavior, and direct/diffuse terminal streams
agree with the executable reconstruction. Direct-only black optics assigns
only integration noise to the shaded class. Vertically overlapping strata
retain separate direct and diffuse transmission and distinct stratum identity.

### Interception, energy, and aerodynamic transfer

PASS. Interception has an ordered finite liquid store, throughfall, stemflow,
drainage, wet fraction, evaporation, condensation/second-drainage, and typed
subfreezing rejection. Wet leaf and stem, dry leaf, and dry stem surfaces have
explicit areas, temperature/resistance nodes, and energy ownership. The
integrated vector uses one `dt`, limits evaporation by stored water inside the
energy solve, uses the identical interval amount for store debit and latent
energy, closes the surface-energy sum, and rejects rate/amount and leaf-only
area aliases. Neutral aerodynamic and boundary transfer has complete typed
forcing/domain guards and no hidden conductance floor. The forest-floor vector
retains unchanged floor operands when canopy demand is reduced.

### Photosynthesis, stomata, and coupled leaf solve

PASS. Bounded FvCB branches, temperature responses, gross/net carbon roles,
Medlyn surface-CO2/VPD coupling, boundary resistance, canopy-air heat/vapor
nodes, and leaf energy are explicit and digest-bound. `beta_hyd` is solved to
make actual gas/energy transpiration equal hydraulic flux at the canonical
water tolerance. The exact reconstruction closes to `2.03e-20 kg m-2 s-1`
and distinguishes the prior one-pass diagnostic result by more than
`5.6e-8 kg m-2 s-1`.

### Hydraulics, root uptake, and arbitration

PASS. All positive root-profile, cap, dry/frozen, and exclusion evidence now
uses the selected four-potential vulnerability/path/gravity equations.
Authorization inputs are interval amounts and are explicitly converted by
`dt`; the amount-as-rate poison differs. The cap-active system is re-solved,
finalized use is bounded by authorization, layer/root/stem/leaf continuity
closes, and negative hydraulic redistribution is typed unsupported rather
than projected to zero. The nonconvergence rollback fixture restores all
candidate-owner bytes.

### Numerical authority and evidence limits

PASS. The selected algorithms, tolerances, finite limits, failure identities,
and no-last-iterate rule are canonical and digest-bound. The calculator is an
authority/test oracle, not evidence of production implementation, calibration,
validation, transferability, deployment, or runtime cutover; the package and
contract preserve those distinctions.

## Prior-Finding Disposition

`RA3-001` through `RA3-005` and `RA4-001` through `RA4-003` are verified
closed on the exact bytes above. The remediations are substantive rather than
label-only: each previously failing or non-discriminating path now has an
equation-aligned calculation and a plausible-wrong poison or typed rejection.

## Recommendation

`PASS` for reviewer-A science acceptance. I found no unresolved material issue
within radiation, interception, canopy/ground energy ownership,
photosynthesis-stomatal coupling, aerodynamics, hydraulics, root-water
arbitration, water/energy closure, or numerical authority that blocks the
package's contract-first implementation-authority release. This review does
not claim production implementation, empirical calibration, independent field
validation, transferability, activation, or cutover.
