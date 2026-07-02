#!/usr/bin/env python3
"""MOFEFID-D7 copyright-safe D-val comparison harness.

Reads the gitignored Papanicolaou supplemental workbook from the local
`references/copyrighted/` cache, verifies its sha256, extracts a single
enhanced-WEPP validation-case trace, runs the openWEPP `dval_case` example on
the same case, and emits ONLY DERIVED SCALAR METRICS (Nash-Sutcliffe of the
traces, peak, time-to-peak, rise metrics) as JSON. It never writes workbook
rows or full hydrograph series to the repo — the copyrighted series stay in
the ignored cache.

S0 cut-point map (Figure_4.xlsx; case = source experiment):
  Case 1 bare       = Abban    Enhanced_WEPP t_col=10(min) q_col=11(m2/s, physical)
  Case 2 isolated   = Jomaa    Enhanced_WEPP t_col=16(min) q_col=17(m2/s)
  Case 3 vegetation = Neibling Enhanced_WEPP t_col=5 (min) q_col=8 (m2/s)
  Case 4 shock      = Iwagaki  Enhanced_WEPP t_col=2 (s)   q_col=1 (m2/s)
See artifacts/cut-point-map.md for the like-for-like reasoning and the
Case-1 column-ambiguity caveat.
"""
import argparse, hashlib, json, subprocess, sys
import numpy as np, pandas as pd

FIG4_SHA256 = "2bf68787de6a715049ee635c154c640214936fd1181d08c8f7da7a34892d2fe8"
# case -> (t_col, t_unit_s_factor, q_col)
CASE_MAP = {
    1: (10, 60.0, 11),
    2: (16, 60.0, 17),
    3: (5,  60.0, 8),
    4: (2,  1.0,  1),
}

def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 16), b""):
            h.update(chunk)
    return h.hexdigest()

def nash_sutcliffe(obs, mod):
    obs, mod = np.asarray(obs), np.asarray(mod)
    denom = np.sum((obs - obs.mean()) ** 2)
    return float("nan") if denom == 0 else 1.0 - np.sum((obs - mod) ** 2) / denom

def rise_time_10_90(t, q):
    qpk = q.max(); ipk = int(np.argmax(q))
    tr = t[:ipk + 1]; qr = q[:ipk + 1]
    if len(tr) < 2: return float("nan")
    t10 = np.interp(0.1 * qpk, qr, tr); t90 = np.interp(0.9 * qpk, qr, tr)
    return float(t90 - t10)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", type=int, required=True)
    ap.add_argument("--fig4", required=True, help="path to gitignored Figure_4.xlsx")
    ap.add_argument("--ko", type=float, default=None)
    ap.add_argument("--ks", type=float, default=None)
    ap.add_argument("--crate-dir", default=".")
    args = ap.parse_args()

    got = sha256(args.fig4)
    if got != FIG4_SHA256:
        print(json.dumps({"error": "sha256 mismatch", "expected": FIG4_SHA256, "got": got}))
        sys.exit(3)

    t_col, tfac, q_col = CASE_MAP[args.case]
    raw = pd.read_excel(args.fig4, sheet_name="Enhanced_WEPP", header=None)
    tref = pd.to_numeric(raw.iloc[2:, t_col], errors="coerce")
    qref = pd.to_numeric(raw.iloc[2:, q_col], errors="coerce")
    m = tref.notna() & qref.notna()
    tref = tref[m].values * tfac; qref = qref[m].values
    order = np.argsort(tref); tref, qref = tref[order], qref[order]

    cmd = ["cargo", "run", "--example", "dval_case", "-p",
           "openwepp-hillslope-orchestrator", "-q", "--", str(args.case)]
    # dval_case takes positional [case, ko, ks]; ks needs ko present, so
    # default ko to the case's ko-of-record (500) when only ks is scanned.
    if args.ks is not None:
        cmd.append(str(args.ko if args.ko is not None else 500.0))
        cmd.append(str(args.ks))
    elif args.ko is not None:
        cmd.append(str(args.ko))
    out = subprocess.run(cmd, cwd=args.crate_dir, capture_output=True, text=True)
    rows = [ln for ln in out.stdout.splitlines() if "," in ln and not ln.startswith("t_s")]
    tm = np.array([float(r.split(",")[0]) for r in rows])
    qm = np.array([float(r.split(",")[1]) for r in rows])

    # like-for-like comparison window: overlap of the two time ranges
    lo = max(tref.min(), tm.min()); hi = min(tref.max(), tm.max())
    grid = np.linspace(lo, hi, 400)
    qr_i = np.interp(grid, tref, qref); qm_i = np.interp(grid, tm, qm)

    result = {
        "case": args.case, "ko": args.ko, "ks_mm_h": args.ks,
        "window_s": [float(lo), float(hi)],
        "NS_trace": nash_sutcliffe(qr_i, qm_i),
        "ref_peak_m2s": float(qref.max()), "ref_t_peak_s": float(tref[np.argmax(qref)]),
        "openwepp_peak_m2s": float(qm.max()), "openwepp_t_peak_s": float(tm[np.argmax(qm)]),
        "peak_ratio": float(qm.max() / qref.max()),
        "ref_rise_10_90_s": rise_time_10_90(tref, qref),
        "openwepp_rise_10_90_s": rise_time_10_90(tm, qm),
        "provenance": {"fig4_sha256": got, "sheet": "Enhanced_WEPP",
                       "t_col": t_col, "q_col": q_col},
    }
    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
