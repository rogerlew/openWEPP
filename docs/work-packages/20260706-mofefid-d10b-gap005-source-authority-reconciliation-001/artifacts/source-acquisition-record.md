# Source Acquisition Record (D10B-S1)

Status: executed
Evidence mode: Ran (identity/extraction commands run this package) + Static

## Acquired (operator, 2026-07-06)

| Source | Local path | Identity verification | Rights first-pass | Role |
|---|---|---|---|---|
| Davis, S. F. (1984). *TVD Finite Difference Schemes and Artificial Viscosity*. ICASE Report 84-20 / NASA CR-172373. | `references/copyrighted/19840021490.pdf` (+ Gemini-converted `19840021490.md`) | Ran: title page + p. 9 rendered and read visually (`pdftoppm`); eq. (3.20) in the `.md` verified faithful against the rendered page | NTRS public download (doc ID 19840021490); ICASE = USRA-operated under NASA contract, so not automatically 17 U.S.C. 105 public domain — kept in `copyrighted/` conservatively; vendorable candidate pending an explicit NTRS rights statement | REQUIRED: binds the limiter branch (eq. 3.20) and the two-sided face dissipation (eqs. 3.17-3.18) |
| Tseng, M.-H. (2010). *Kinematic wave computation using an efficient implicit method*. J. Hydroinformatics 12(3), 329-338. | `references/copyrighted/Tseng2010_Hydroinformatics.pdf` | Ran: first page extracted (`pdftotext`), title/author/journal confirmed | IWA Publishing copyright; local restricted cache in `copyrighted/` | CONFIRMATORY + precedent: R-63 §2.3's named source for the applied scheme; validates KWE MacCormack schemes against ANALYTICAL SOLUTIONS + experiment (abstract, lines 11-15 of extract) — published precedent for the Leg-B oracle acceptance shape |

Bibliography rows: R-102 (Davis), R-103 (Tseng) in
`references/annotated_bibliography.md` (registered 2026-07-06). Rights-log
addendum appended to
`references/rights_classification_first_pass_2026-05-11.md` this package.

## Conversion-fidelity spot-checks (Gemini `.md` of Davis)

| Equation | Check | Result |
|---|---|---|
| (3.20) limiter | Rendered p. 9 read visually vs `.md` line 212 | FAITHFUL: `min(2r,1) if r>0; 0 if r<=0` |
| (3.18) dissipation coefficients | Rendered p. 9 (both `K±_{k+1/2} = (|v|/2)(1-|v|)[1-phi(r±)]`) vs `.md` | FAITHFUL (two-sided face form visible on the page) |

Rule: any further Davis equation cited as binding authority must be
spot-checked against the PDF page render first (the `.md` is a conversion,
not primary).

## Already in hand (D10 acquisitions, reused)

- Garcia-Navarro 1992 (R-81): `10.1061@ASCE0733-94291992118@101359.pdf`.
- Mingham 2001 (R-82): `mingham2001.pdf` — D10B extracted §4.2-4.3
  (eqs. 28b, 31a, 31f, 31g, `C(x)` and the CFL `nu = 0.9` statement) via
  `pdftotext`; these bind the exact scheme variant R-63 §2.3 follows.
- Iwagaki 1955 (R-74): `Iwagaki1955_runoff_characteristics_DPRI10.pdf` —
  D10B extracted the experiment (A)/(B) configuration text (B=19.6 cm,
  L=24 m, n=0.009 m-s, nu=0.01 cm^2/s; (B): sin theta = 0.020/0.015/0.010
  and q = 0.1080/0.0638/0.0800 cm/s per 8 m reach; durations T=10/20/30/40 s),
  the Manning-type resistance statement, and the characteristics equations
  (dh/dt = q along characteristics; straight-line characteristics after
  cutoff; laminar/turbulent branches switched on Re 500/1500).
- Lighthill & Whitham 1955 (R-01): kinematic shock (Rankine-Hugoniot)
  fitting authority for the oracle.

## Not acquired / not sought

- Papanicolaou implementation material beyond the published paper +
  supplemental: EXCLUDED by the clean-room posture
  (`docs/planning/mofe-water-balance-sequencing.md` §3). No attempt made.
