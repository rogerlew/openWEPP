import pandas as pd, numpy as np, json
S="/tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad"

def decompose(path, recession_days=5):
    wat = pd.read_parquet(path)
    outlet = wat["OFE"].max()
    ow = wat[wat["OFE"] == outlet].sort_values("sim_day_index").reset_index(drop=True)
    a_out = ow["Area"].iloc[0]
    a_total = wat[wat["sim_day_index"] == wat["sim_day_index"].min()]["Area"].sum()
    # daily hillslope-normalized depths (mm over A_total)
    day = pd.DataFrame({
        "d": ow["sim_day_index"].values,
        "P": ow["P"].values,                                   # precip depth (mm)
        "runvol_mm": ow["QOFE"].values * a_out / a_total,      # surface export as depth over A_total
        "latqcc_mm": ow["latqcc"].values * a_out / a_total,    # lateral export at outlet, depth over A_total
    })
    day["export_mm"] = day["runvol_mm"] + day["latqcc_mm"]
    # 14-day antecedent precip
    day["ap14"] = day["P"].rolling(14, min_periods=1).sum().shift(1).fillna(0.0)
    # storm delineation: clusters of consecutive wet days (P>0.2mm), separated by >=1 dry day
    wet = day["P"].values > 0.2
    storms = []
    i = 0; n = len(day)
    while i < n:
        if not wet[i]:
            i += 1; continue
        j = i
        while j < n and wet[j]:
            j += 1
        # storm window [i, j); export window extends through recession until next wet or cap
        k = min(j + recession_days, n)
        # truncate export window at next storm onset
        m = j
        while m < k and not wet[m]:
            m += 1
        ewin_end = m  # export accumulates over storm days + dry recession up to next storm
        P_ev = day["P"].iloc[i:j].sum()
        exp_ev = day["export_mm"].iloc[i:ewin_end].sum()
        storms.append({
            "start_d": int(day["d"].iloc[i]), "P_mm": float(P_ev),
            "export_mm": float(exp_ev), "ratio": float(exp_ev / P_ev) if P_ev > 0 else 0.0,
            "ap14_mm": float(day["ap14"].iloc[i]),
        })
        i = j
    return pd.DataFrame(storms), a_total

def analyze(label, df):
    n = len(df)
    small = df[df["P_mm"] < 15]; large = df[df["P_mm"] > 50]
    # commencement threshold: largest P with mean ratio still < 1% via step-fit over sorted P
    dfs = df.sort_values("P_mm").reset_index(drop=True)
    # step threshold minimizing SSE of ratio vs a low/high step
    best_t, best_sse = None, 1e18
    for t in np.arange(5, 80, 1.0):
        lo = dfs[dfs["P_mm"] <= t]["ratio"]; hi = dfs[dfs["P_mm"] > t]["ratio"]
        if len(lo) < 3 or len(hi) < 3: continue
        sse = ((lo - lo.mean())**2).sum() + ((hi - hi.mean())**2).sum()
        if sse < best_sse: best_sse, best_t = sse, t
    wetlarge = df[(df["P_mm"] > 50) & (df["ap14_mm"] > 20)]
    # ascending shape: spearman rank corr of ratio vs P, and vs ap14
    def spear(x, y):
        xr = pd.Series(x).rank(); yr = pd.Series(y).rank()
        return float(np.corrcoef(xr, yr)[0,1])
    out = {
        "label": label, "n_storms": int(n),
        "threshold_mm": float(best_t) if best_t else None,
        "small_lt15_mean_ratio": float(small["ratio"].mean()) if len(small) else None,
        "small_lt15_frac_below_1pct": float((small["ratio"] < 0.01).mean()) if len(small) else None,
        "large_gt50_mean_ratio": float(large["ratio"].mean()) if len(large) else None,
        "wet_large_gt50_ap14gt20_mean_ratio": float(wetlarge["ratio"].mean()) if len(wetlarge) else None,
        "wet_large_n": int(len(wetlarge)),
        "spearman_ratio_vs_P": spear(df["P_mm"], df["ratio"]),
        "spearman_ratio_vs_ap14": spear(df["ap14_mm"], df["ratio"]),
    }
    return out

results = {}
for label, path in [("post_dc01", f"{S}/dc01-m3/out/H2637.wat.parquet"),
                    ("pre_dc01", f"{S}/postmerge/out/H2637.wat.parquet")]:
    for rec in [3, 5, 7]:
        df, atot = decompose(path, recession_days=rec)
        results[f"{label}_rec{rec}"] = analyze(f"{label}_rec{rec}", df)
print(json.dumps(results, indent=2))
