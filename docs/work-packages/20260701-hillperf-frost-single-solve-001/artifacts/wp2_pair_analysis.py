#!/usr/bin/env python3
"""WP-2 Stage 1: pair the builder/r4a frost solves per (lane, day) and diff."""
import json
import sys
from collections import defaultdict

OUT_FIELDS = [
    "out_active",
    "out_infcap_frz_m_s",
    "out_frost_depth_after_m",
    "out_frozen_water_after_m",
    "out_thdp_after_m",
    "out_dthaw_after_m",
    "out_tfrdp_after_m",
    "out_tthawd_after_m",
    "out_fgthwd_flag_after",
    "out_frwatc_net_liquid_delta_m",
    "out_total_fine_layer_count",
]
IN_FIELDS = [
    "in_soil_water_m",
    "in_layer_theta_sum_m",
    "in_layer_frozen_water_sum_m",
    "in_prior_dfrost_m",
    "in_prior_ws_frz_m",
]
THRESHOLDS = [0.0, 1e-12, 1e-9, 1e-6, 1e-3]

rows = defaultdict(dict)
with open(sys.argv[1]) as fh:
    for line in fh:
        r = json.loads(line)
        key = (r["lane_index"], r["day_index"])
        # A lane-day can hit r4a once per day; builder once per day. Keep last
        # per source but flag duplicates.
        src = r["source"]
        if src in rows[key]:
            rows[key][src + "_dup"] = rows[key].get(src + "_dup", 0) + 1
        rows[key][src] = r

pairs, builder_only, r4a_only, dups = [], 0, 0, 0
for key, srcs in rows.items():
    dups += sum(v for k, v in srcs.items() if k.endswith("_dup"))
    has_b, has_r = "builder" in srcs, "r4a" in srcs
    if has_b and has_r:
        pairs.append((key, srcs["builder"], srcs["r4a"]))
    elif has_b:
        builder_only += 1
    else:
        r4a_only += 1

print(f"lane-days with any solve: {len(rows)}")
print(f"paired: {len(pairs)}  builder-only: {builder_only}  r4a-only: {r4a_only}  dup-rows: {dups}")

def fnum(v):
    return float(v) if v is not None else float("nan")

print("\nPer-field absolute deltas over paired lane-days (r4a - builder):")
print(f"{'field':38s} {'max':>12s} {'p99':>12s} {'median':>12s} " + " ".join(f">{t:g}" for t in THRESHOLDS))
worst = {}
for f in OUT_FIELDS + IN_FIELDS:
    if f == "out_active":
        mism = [(k) for k, b, r in pairs if b[f] != r[f]]
        print(f"{f:38s} bool-mismatches: {len(mism)}" + (f"  first: {mism[:5]}" if mism else ""))
        continue
    deltas = sorted(abs(fnum(r[f]) - fnum(b[f])) for _, b, r in pairs)
    if not deltas:
        continue
    n = len(deltas)
    counts = [sum(1 for d in deltas if d > t) for t in THRESHOLDS]
    print(f"{f:38s} {deltas[-1]:12.3e} {deltas[int(n*0.99)]:12.3e} {deltas[n//2]:12.3e} " + " ".join(f"{c:6d}" for c in counts))
    worst[f] = deltas[-1]

# Show the worst divergent pairs for the headline outcome fields
for f in ("out_infcap_frz_m_s", "out_frost_depth_after_m", "out_frozen_water_after_m", "out_frwatc_net_liquid_delta_m"):
    ranked = sorted(pairs, key=lambda p: abs(fnum(p[2][f]) - fnum(p[1][f])), reverse=True)[:3]
    print(f"\nworst {f}:")
    for key, b, r in ranked:
        print(f"  lane={key[0]} day={key[1]}  builder={b[f]}  r4a={r[f]}  d_in_soil={fnum(r['in_soil_water_m'])-fnum(b['in_soil_water_m']):.3e}")
