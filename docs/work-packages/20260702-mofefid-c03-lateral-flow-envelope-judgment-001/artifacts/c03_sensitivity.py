import pandas as pd, numpy as np, json
S="/tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad"
wat=pd.read_parquet(f"{S}/dc01-m3/out/H2637.wat.parquet"); outlet=wat["OFE"].max()
ow=wat[wat["OFE"]==outlet].sort_values("sim_day_index").reset_index(drop=True)
a_out=ow["Area"].iloc[0]; a_total=wat[wat["sim_day_index"]==wat["sim_day_index"].min()]["Area"].sum()
d=pd.DataFrame({"P":ow["P"].values,"runvol_mm":ow["QOFE"].values*a_out/a_total,
    "latqcc_mm":ow["latqcc"].values*a_out/a_total})
d["total_mm"]=d["runvol_mm"]+d["latqcc_mm"]
d["ap14"]=d["P"].rolling(14,min_periods=1).sum().shift(1).fillna(0.0)

def sep(q,slope):
    bf=np.zeros_like(q); bf[0]=min(q[0],q.min())
    for i in range(1,len(q)):
        bf[i]=max(0.0,min(q[i],bf[i-1]+slope))
    return np.maximum(q-bf,0.0)

def evE(quick):
    wet=d["P"].values>0.2; st=[]; i=0; n=len(d)
    while i<n:
        if not wet[i]: i+=1; continue
        j=i
        while j<n and wet[j]: j+=1
        k=min(j+5,n); m=j
        while m<k and not wet[m]: m+=1
        P=d["P"].iloc[i:j].sum(); E=quick[i:m].sum()
        st.append((P,E/P if P>0 else 0,d["ap14"].iloc[i])); i=j
    st=pd.DataFrame(st,columns=["P","ratio","ap14"])
    wl=st[(st["P"]>50)&(st["ap14"]>20)]
    sp=lambda x,y:float(np.corrcoef(pd.Series(x).rank(),pd.Series(y).rank())[0,1])
    return round(float(wl["ratio"].mean()),4),int(len(wl)),round(sp(st["P"],st["ratio"]),3),round(sp(st["ap14"],st["ratio"]),3)

# Hewlett-Hibbert 0.55 L/s/km2/h -> daily depth-rise: 0.55*3.6e-3 mm/h per hour *24h = daily rise in mm/day
hh_daily = 0.55*3.6e-3*24   # mm/day rise
print(f"Hewlett-Hibbert canonical daily baseflow-rise slope = {hh_daily:.4f} mm/day")
print(f"{'slope mm/d':>12}{'ENV-E ratio':>13}{'n':>5}{'spear_P':>9}{'spear_ap':>9}")
for slope in [0.005, 0.0158, 0.02, 0.05, 0.1, 0.2, 0.5]:
    r,nn,sp1,sp2=evE(sep(d["total_mm"].values,slope))
    tag=" <- H-H canonical" if abs(slope-hh_daily)<0.002 else ""
    print(f"{slope:>12.4f}{r:>13}{nn:>5}{sp1:>9}{sp2:>9}{tag}")
# parameter-free anchor: surface runoff only (unambiguously quick)
r,nn,sp1,sp2=evE(d["runvol_mm"].values)
print(f"\nSURFACE-ONLY (no separation param): ENV-E ratio={r} n={nn} spear_P={sp1} spear_ap={sp2}")
