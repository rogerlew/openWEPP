import pandas as pd, numpy as np, json
S="/tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad"

def series(path):
    wat = pd.read_parquet(path); outlet = wat["OFE"].max()
    ow = wat[wat["OFE"]==outlet].sort_values("sim_day_index").reset_index(drop=True)
    a_out = ow["Area"].iloc[0]; a_total = wat[wat["sim_day_index"]==wat["sim_day_index"].min()]["Area"].sum()
    d = pd.DataFrame({"d":ow["sim_day_index"].values,"P":ow["P"].values,
        "runvol_mm":ow["QOFE"].values*a_out/a_total,
        "latqcc_mm":ow["latqcc"].values*a_out/a_total})
    d["total_mm"]=d["runvol_mm"]+d["latqcc_mm"]
    # Hewlett-Hibbert-style baseflow separation on the total-export daily series:
    # rising baseflow at fixed slope from each hydrograph onset; quickflow = total - baseflow.
    bf_slope = 0.02  # mm/day baseflow rise per day (Hewlett-Hibbert 0.55 L/s/km2/h daily-equiv ~ 0.047 mm/d; use conservative 0.02)
    q = d["total_mm"].values; bf = np.zeros_like(q); 
    bf[0]=min(q[0], q.min())
    base = q.min()
    for i in range(1,len(q)):
        cand = bf[i-1]+bf_slope
        bf[i]=min(q[i], cand) if q[i] < cand else min(cand, q[i])
        bf[i]=min(bf[i], q[i]); bf[i]=max(bf[i], 0.0)
    d["quick_mm"]=np.maximum(q-bf,0.0)
    d["ap14"]=d["P"].rolling(14,min_periods=1).sum().shift(1).fillna(0.0)
    return d

def storms(d, export_col, rec=5):
    wet=d["P"].values>0.2; out=[]; i=0; n=len(d)
    while i<n:
        if not wet[i]: i+=1; continue
        j=i
        while j<n and wet[j]: j+=1
        k=min(j+rec,n); m=j
        while m<k and not wet[m]: m+=1
        P=d["P"].iloc[i:j].sum(); E=d[export_col].iloc[i:m].sum()
        out.append({"P_mm":float(P),"export_mm":float(E),"ratio":float(E/P) if P>0 else 0.0,"ap14":float(d["ap14"].iloc[i])})
        i=j
    return pd.DataFrame(out)

def spear(x,y):
    return float(np.corrcoef(pd.Series(x).rank(),pd.Series(y).rank())[0,1])

def judge(df):
    small=df[df["P_mm"]<15]; large=df[df["P_mm"]>50]; wl=df[(df["P_mm"]>50)&(df["ap14"]>20)]
    dfs=df.sort_values("P_mm").reset_index(drop=True); bt,bs=None,1e18
    for t in np.arange(5,80,1.0):
        lo=dfs[dfs["P_mm"]<=t]["ratio"]; hi=dfs[dfs["P_mm"]>t]["ratio"]
        if len(lo)<3 or len(hi)<3: continue
        sse=((lo-lo.mean())**2).sum()+((hi-hi.mean())**2).sum()
        if sse<bs: bs,bt=sse,t
    return {"n":int(len(df)),"threshold_mm":float(bt) if bt else None,
        "small_lt15_mean_ratio":round(float(small["ratio"].mean()),4),
        "small_lt15_median_ratio":round(float(small["ratio"].median()),4),
        "large_gt50_mean_ratio":round(float(large["ratio"].mean()),4),
        "wet_large_mean_ratio":round(float(wl["ratio"].mean()),4),"wet_large_n":int(len(wl)),
        "spearman_ratio_vs_P":round(spear(df["P_mm"],df["ratio"]),4),
        "spearman_ratio_vs_ap14":round(spear(df["ap14"],df["ratio"]),4)}

res={}
for lbl,path in [("post_dc01",f"{S}/dc01-m3/out/H2637.wat.parquet")]:
    d=series(path)
    res[f"{lbl}_QUICKFLOW"]=judge(storms(d,"quick_mm"))
    res[f"{lbl}_SURFACE_runvol"]=judge(storms(d,"runvol_mm"))
    # annual quickflow fraction sanity
    yrs=34
    res[f"{lbl}_annual"]={"quick_frac_of_P":round(float(d["quick_mm"].sum()/d["P"].sum()),4),
        "surface_frac_of_P":round(float(d["runvol_mm"].sum()/d["P"].sum()),4),
        "total_yield_frac":round(float(d["total_mm"].sum()/d["P"].sum()),4)}
print(json.dumps(res,indent=2))

# ENV-T shape: size-binned quickflow response (post-DC01)
print("\n=== ENV-T size-bin table (post-DC01, quickflow) ===")
d=series(f"{S}/dc01-m3/out/H2637.wat.parquet")
sd=storms(d,"quick_mm")
bins=[(0,10),(10,20),(20,30),(30,50),(50,80),(80,999)]
print(f"{'P bin (mm)':<12}{'n':>5}{'mean ratio':>12}{'median':>9}{'frac>1mm QF':>13}")
for lo,hi in bins:
    b=sd[(sd['P_mm']>=lo)&(sd['P_mm']<hi)]
    if len(b)==0: continue
    fq=float((b['export_mm']>1.0).mean())
    print(f"{f'{lo}-{hi}':<12}{len(b):>5}{b['ratio'].mean():>12.3f}{b['ratio'].median():>9.3f}{fq:>13.3f}")
# snow influence: fraction of small-storm quickflow-bearing days that had snowmelt
w=pd.read_parquet(f"{S}/dc01-m3/out/H2637.wat.parquet"); ow=w[w['OFE']==w['OFE'].max()]
snowdays=(ow['Snow-Water'].values>1.0).mean()
print(f"\nfrac of outlet days with Snow-Water>1mm: {snowdays:.3f}")
