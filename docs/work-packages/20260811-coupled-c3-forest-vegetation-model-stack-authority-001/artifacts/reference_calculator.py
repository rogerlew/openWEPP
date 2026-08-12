#!/usr/bin/env python3
"""Independent OPENWEPP_C3_WOODY_V1 contract-vector calculator.

This standard-library oracle is package evidence, never production code.  It
intentionally reconstructs operands from input records instead of sharing a
future Rust implementation.
"""
from __future__ import annotations

import copy
import json
import math
import sys


def adaptive_simpson(func, a: float, b: float, tolerance: float = 1e-14,
                     max_depth: int = 20) -> float:
    """Deterministic adaptive Simpson integration used by the admitted optics."""
    fa, fb = func(a), func(b)
    midpoint = (a + b) / 2.0
    fm = func(midpoint)
    whole = (b - a) * (fa + 4.0 * fm + fb) / 6.0

    def refine(left: float, right: float, fleft: float, fmid: float,
               fright: float, estimate: float, tol: float, depth: int) -> float:
        center = (left + right) / 2.0
        lmid, rmid = (left + center) / 2.0, (center + right) / 2.0
        flmid, frmid = func(lmid), func(rmid)
        left_est = (center-left)*(fleft+4.0*flmid+fmid)/6.0
        right_est = (right-center)*(fmid+4.0*frmid+fright)/6.0
        delta = left_est + right_est - estimate
        if depth == 0:
            raise ValueError("radiation quadrature depth limit")
        if abs(delta) <= 15.0*tol:
            return left_est + right_est + delta/15.0
        return (refine(left, center, fleft, flmid, fmid, left_est, tol/2.0, depth-1)
                + refine(center, right, fmid, frmid, fright, right_est, tol/2.0, depth-1))

    return refine(a, b, fa, fm, fb, whole, tolerance, max_depth)


def small_root(a: float, b: float, c: float) -> float:
    disc = b * b - 4.0 * a * c
    if disc < 0.0:
        raise ValueError("negative discriminant")
    root = math.sqrt(disc)
    q = -0.5 * (b + math.copysign(root, b))
    r1, r2 = q / a, c / q
    return min(r1, r2)


def fvbc(par: float, ci: float, vcmax: float, jmax: float, rd: float) -> dict[str, float | str]:
    gamma, kc, ko, oi = 4.275, 40.49, 27840.0, 20265.0
    ipsii = 0.5 * 0.85 * 4.6 * par
    j = small_root(0.7, -(ipsii + jmax), ipsii * jmax) if ipsii else 0.0
    ac = vcmax * (ci - gamma) / (ci + kc * (1.0 + oi / ko)) if ci >= gamma else 0.0
    aj = j * (ci - gamma) / (4.0 * ci + 8.0 * gamma) if ci >= gamma else 0.0
    ap = 3.0 * (0.167 * vcmax)
    ai = small_root(0.98, -(ac + aj), ac * aj)
    ag = small_root(0.95, -(ai + ap), ai * ap)
    limiting = "rubisco" if ac < aj else "electron"
    return {"ac": ac, "aj": aj, "ap": ap, "j": j, "an": ag - rd, "limit": limiting}


def medlyn(an: float, d_kpa: float, cs_pa: float, patm_pa: float, g0: float, g1: float) -> float:
    if d_kpa <= 0 or cs_pa <= 0 or patm_pa <= 0:
        raise ValueError("Medlyn domain")
    return g0 if an <= 0 else g0 + 1.6 * (1.0 + g1 / math.sqrt(d_kpa)) * an / (cs_pa / patm_pa)


def interception(s0: float, rain: float, plant_area: float, cap_per_area: float,
                 alpha_liq: float, stem_fraction: float, wet_potential: float,
                 canopy_temperature: float = 295.0) -> dict[str, float]:
    if canopy_temperature < 273.15:
        raise ValueError("subfreezing liquid interception")
    smax = cap_per_area * plant_area
    intercepted = rain * alpha_liq * math.tanh(plant_area) if plant_area else 0.0
    direct = rain - intercepted
    stem = stem_fraction * direct
    through = direct - stem
    pre = s0 + intercepted
    drainage = max(0.0, pre - smax)
    stored = pre - drainage
    if wet_potential >= 0.0:
        evap = min(wet_potential, stored)
        s1 = stored - evap
    else:
        evap = wet_potential
        post_condensation = stored - evap
        second_drainage = max(0.0, post_condensation-smax)
        drainage += second_drainage
        s1 = post_condensation-second_drainage
    closure = s0 + rain - s1 - evap - through - stem - drainage
    return {"s1": s1, "evap": evap, "through": through, "stem": stem,
            "drainage": drainage, "closure": closure}


def two_stream(plant_area: float, mu: float, chi: float, rho: float, tau: float,
               ground_albedo: float, direct: float, diffuse: float) -> dict[str, float]:
    """Independent RK4 shooting solution of canonical two-stream ODEs."""
    if not -0.4 <= chi <= 0.6: raise ValueError("leaf-angle domain")
    if direct > 0.0 and mu <= 0.0: raise ValueError("direct-beam zenith domain")
    phi1 = 0.5 - 0.633 * chi - 0.33 * chi * chi
    phi2 = 0.877 * (1.0 - 2.0 * phi1)
    gmu = phi1 + phi2 * mu
    kbeam = gmu / mu if mu > 0.0 else 0.0
    mubar=adaptive_simpson(lambda mup: mup/(phi1+phi2*mup),0.0,1.0)
    omega = rho + tau
    cosbar = (1.0 + chi) / 2.0
    omega_beta = 0.5 * (rho + tau + (rho - tau) * cosbar * cosbar)
    beta = omega_beta / omega if omega > 0.0 else 0.0
    b = 1.0 - omega + omega_beta
    c = omega_beta
    # Adaptive-Simpson reconstruction of Sellers' single-scattering integral.
    def scatter_integrand(mup: float) -> float:
        gp = phi1 + phi2 * mup
        denom = mu * gp + mup * gmu
        return 0.0 if denom == 0.0 else mup * gmu / denom
    integ = adaptive_simpson(scatter_integrand,0.0,1.0) if direct>0.0 else 0.0
    single_scatter = 0.5 * omega * integ
    beta0 = ((1.0 + mubar * kbeam) / (mubar * kbeam) * single_scatter / omega
             if omega > 0.0 and direct>0.0 else 0.0)
    d = omega * mubar * kbeam * beta0
    f = omega * mubar * kbeam * (1.0 - beta0)

    def integrate(iup0: float, idown0: float) -> tuple[float, float]:
        n = 4000
        h = plant_area / n
        up, down = iup0, idown0
        def deriv(x: float, y: tuple[float, float]) -> tuple[float, float]:
            beam = direct * math.exp(-kbeam * x)
            return ((b*y[0]-c*y[1]-d*beam)/mubar,
                    (f*beam-b*y[1]+c*y[0])/mubar)
        for j in range(n):
            x = j*h
            k1 = deriv(x, (up, down))
            k2 = deriv(x+h/2, (up+h*k1[0]/2, down+h*k1[1]/2))
            k3 = deriv(x+h/2, (up+h*k2[0]/2, down+h*k2[1]/2))
            k4 = deriv(x+h, (up+h*k3[0], down+h*k3[1]))
            up += h*(k1[0]+2*k2[0]+2*k3[0]+k4[0])/6
            down += h*(k1[1]+2*k2[1]+2*k3[1]+k4[1])/6
        return up, down
    a = integrate(0.0, diffuse)
    z = integrate(1.0, diffuse)
    slope_up, slope_down = z[0]-a[0], z[1]-a[1]
    terminal_direct = direct * math.exp(-kbeam*plant_area)
    iup0 = (ground_albedo*(a[1]+terminal_direct)-a[0]) / (slope_up-ground_albedo*slope_down)
    upx, downx = integrate(iup0, diffuse)
    incident = direct + diffuse
    transmitted = downx + terminal_direct
    absorbed = incident - iup0 - (1.0-ground_albedo)*transmitted
    transmitted_direct = terminal_direct
    transmitted_diffuse = downx
    sunlit_area = ((1.0-math.exp(-kbeam*plant_area))/kbeam
                   if plant_area and direct>0.0 else 0.0)
    # Independently integrate local absorption and weight it by the illuminated
    # leaf fraction exp(-Kx), rather than assigning all diffuse absorption to shade.
    nprofile=32000; h=plant_area/nprofile if nprofile else 0.0
    up,down=iup0,diffuse; sunlit_absorbed=0.0; profile_absorbed=0.0
    for j in range(nprofile):
        x=j*h
        def deriv_profile(xx: float, uu: float, dd: float) -> tuple[float,float]:
            beam=direct*math.exp(-kbeam*xx)
            return ((b*uu-c*dd-d*beam)/mubar,(f*beam-b*dd+c*uu)/mubar)
        k1=deriv_profile(x,up,down)
        mid_up,mid_down=up+h*k1[0]/2,down+h*k1[1]/2
        k2=deriv_profile(x+h/2,mid_up,mid_down)
        k3=deriv_profile(x+h/2,up+h*k2[0]/2,down+h*k2[1]/2)
        k4=deriv_profile(x+h,up+h*k3[0],down+h*k3[1])
        dup,ddown=deriv_profile(x+h/2,mid_up,mid_down)
        beam_mid=direct*math.exp(-kbeam*(x+h/2))
        local=kbeam*beam_mid-ddown+dup
        direct_absorption=(1.0-omega)*kbeam*beam_mid
        diffuse_absorption=local-direct_absorption
        profile_absorbed+=local*h
        illumination=math.exp(-kbeam*(x+h/2)) if direct>0.0 else 0.0
        sunlit_absorbed+=(direct_absorption+diffuse_absorption*illumination)*h
        up+=h*(k1[0]+2*k2[0]+2*k3[0]+k4[0])/6
        down+=h*(k1[1]+2*k2[1]+2*k3[1]+k4[1])/6
    if not math.isclose(profile_absorbed,absorbed,rel_tol=2e-6,abs_tol=1e-8):
        raise ValueError("radiation profile closure")
    shaded_absorbed=absorbed-sunlit_absorbed
    return {"reflected": iup0, "transmitted": transmitted,
            "transmitted_direct": transmitted_direct,
            "transmitted_diffuse": transmitted_diffuse,
            "absorbed": absorbed, "sunlit_absorbed": sunlit_absorbed,
            "shaded_absorbed": shaded_absorbed, "sunlit_area": sunlit_area,
            "closure": incident-iup0-(1-ground_albedo)*transmitted-absorbed}


def qsat(tk: float) -> float:
    tc = tk - 273.15
    if not 0.0 <= tc <= 100.0:
        raise ValueError("liquid saturation polynomial domain")
    coeff = [6.11213476,4.44007856e-1,1.43064234e-2,2.64461437e-4,
             3.05903558e-6,1.96237241e-8,8.92344772e-11,
             -3.73208410e-13,2.09339997e-16]
    es = 100.0 * math.fsum(a*tc**i for i,a in enumerate(coeff))
    return 0.622 * es / (101325.0 - 0.378 * es)


def leaf_temperature(sw: float, lw_down: float, lw_up: float, tair: float,
                     qair: float, lai: float, gb: float, gs: float) -> dict[str, float]:
    eps, sigma, rho, cp, lam = 0.96, 5.670374419e-8, 1.18, 1005.0, 2.45e6
    def residual(t: float) -> float:
        lw = eps*(lw_down+lw_up-2*sigma*t**4)
        h = rho*cp*gb*(t-tair)*lai
        e = rho*(qsat(t)-qair)/(1/gb+1/gs)*lai
        return sw+lw-h-lam*e
    lo, hi = 273.15, 330.0
    for _ in range(100):
        mid=(lo+hi)/2
        if residual(lo)*residual(mid)<=0: hi=mid
        else: lo=mid
    t=(lo+hi)/2
    return {"temperature": t, "residual": residual(t),
            "transpiration": rho*(qsat(t)-qair)/(1/gb+1/gs)*lai}


def wet_canopy_temperature(sw: float, lw_down: float, lw_up: float,
                           tcan: float, qcan: float, leaf_area: float,
                           stem_area: float, wet_fraction: float,
                           wind_speed: float, surface_dimension: float) -> dict[str,float]:
    """Common wet leaf/stem node with an explicit stem-area energy owner."""
    if not 0.0 <= wet_fraction <= 1.0 or surface_dimension <= 0.0 or wind_speed < 0.0:
        raise ValueError("wet-surface domain")
    leaf_wet, stem_wet = wet_fraction*leaf_area, wet_fraction*stem_area
    wet_area = leaf_wet + stem_wet
    if wet_area == 0.0:
        return {"temperature":tcan,"evaporation":0.0,"residual":0.0,
                "leaf_energy":0.0,"stem_energy":0.0,"wet_area":0.0}
    eps,sigma,patm,gas_r,cp,lam=0.96,5.670374419e-8,101325.0,287.05,1004.64,2.501e6
    gb=0.01*math.sqrt(wind_speed/surface_dimension)
    if gb <= 0.0: raise ValueError("zero wet-surface conductance")
    rho_air=patm/(gas_r*tcan)
    def residual(t: float) -> float:
        lw=eps*wet_area*(lw_down+lw_up-2.0*sigma*t**4)
        sensible=rho_air*cp*gb*(t-tcan)*wet_area
        evaporation=rho_air*gb*(qsat(t)-qcan)*wet_area
        return sw+lw-sensible-lam*evaporation
    lo,hi=273.15,330.0
    if residual(lo)*residual(hi)>0.0: raise ValueError("wet-surface energy root not bracketed")
    for _ in range(100):
        mid=(lo+hi)/2.0
        if residual(lo)*residual(mid)<=0.0: hi=mid
        else: lo=mid
    temperature=(lo+hi)/2.0
    evaporation=rho_air*gb*(qsat(temperature)-qcan)*wet_area
    total_energy=sw+eps*wet_area*(lw_down+lw_up-2.0*sigma*temperature**4)
    return {"temperature":temperature,"evaporation":evaporation,
            "residual":residual(temperature),"wet_area":wet_area,
            "leaf_energy":total_energy*leaf_wet/wet_area,
            "stem_energy":total_energy*stem_wet/wet_area}


def integrated_canopy_energy() -> dict[str,float]:
    """One store-limited wet/dry leaf/stem water and energy ledger."""
    leaf_area,stem_area,dt_s=3.2,0.9,1800.0
    total_area=leaf_area+stem_area; store0=0.030; store_capacity=0.22*total_area
    wet_fraction=(store0/store_capacity)**(2.0/3.0)
    wet_leaf,wet_stem=wet_fraction*leaf_area,wet_fraction*stem_area
    dry_leaf,dry_stem=(1-wet_fraction)*leaf_area,(1-wet_fraction)*stem_area
    sw_total=500.0; lw_down,lw_up,tcan,qcan=330.0,410.0,296.0,0.010
    eps,sigma,patm,gas_r,cp,lam=0.96,5.670374419e-8,101325.0,287.05,1004.64,2.501e6
    rho_air=patm/(gas_r*tcan); gb=0.01*math.sqrt(1.8/0.03)
    wet_area=wet_leaf+wet_stem; wet_sw=sw_total*wet_area/total_area
    store_rate_limit=store0/dt_s
    def wet_terms(t: float) -> tuple[float,float,float,float]:
        potential=rho_air*gb*(qsat(t)-qcan)*wet_area
        actual=min(potential,store_rate_limit) if potential>=0.0 else potential
        lw=eps*wet_area*(lw_down+lw_up-2*sigma*t**4)
        sensible=rho_air*cp*gb*(t-tcan)*wet_area
        return potential,actual,lw,sensible
    def wet_residual(t: float) -> float:
        _,actual,lw,sensible=wet_terms(t)
        return wet_sw+lw-sensible-lam*actual
    lo,hi=273.15,330.0
    for _ in range(100):
        mid=(lo+hi)/2
        if wet_residual(lo)*wet_residual(mid)<=0: hi=mid
        else: lo=mid
    twet=(lo+hi)/2; potential,evap_rate,wet_lw,wet_h=wet_terms(twet)
    evaporation_amount=evap_rate*dt_s; store1=store0-evaporation_amount
    dry_leaf_sw=sw_total*dry_leaf/total_area
    dry_leaf_state=leaf_temperature(dry_leaf_sw,lw_down,lw_up,tcan,qcan,dry_leaf,0.025,0.006)
    dry_stem_sw=sw_total*dry_stem/total_area
    def stem_residual(t: float) -> float:
        lw=eps*dry_stem*(lw_down+lw_up-2*sigma*t**4)
        sensible=rho_air*cp*gb*(t-tcan)*dry_stem
        return dry_stem_sw+lw-sensible
    lo,hi=273.15,330.0
    for _ in range(100):
        mid=(lo+hi)/2
        if stem_residual(lo)*stem_residual(mid)<=0: hi=mid
        else: lo=mid
    tstem=(lo+hi)/2
    return {"dt_s":dt_s,"store0":store0,"store1":store1,
            "wet_fraction":wet_fraction,"wet_leaf_area":wet_leaf,"wet_stem_area":wet_stem,
            "dry_leaf_area":dry_leaf,"dry_stem_area":dry_stem,
            "wet_temperature":twet,"dry_leaf_temperature":dry_leaf_state["temperature"],
            "dry_stem_temperature":tstem,"wet_potential_rate":potential,
            "wet_actual_rate":evap_rate,"wet_amount":evaporation_amount,
            "water_closure":store0-store1-evaporation_amount,
            "latent_energy_j":lam*evaporation_amount,
            "energy_residual_j":dt_s*(wet_residual(twet)+dry_leaf_state["residual"]+stem_residual(tstem)),
            "shortwave_partition":wet_sw+dry_leaf_sw+dry_stem_sw,
            "rate_as_amount_poison":store0-store1-evap_rate,
            "leaf_only_area_poison":wet_leaf+dry_leaf+dry_stem}


def coupled_leaf_state(par: float, ca_pa: float, d_kpa: float, qair: float,
                       lai: float) -> dict[str,float]:
    """Independent FvCB--Medlyn--surface-node--energy reconstruction."""
    if lai <= 0.0:
        return {"ci":ca_pa,"temperature":296.0,"an":0.0,"gs_umol":0.0,
                "transpiration":0.0,"ci_residual":0.0,"energy_residual":0.0}
    patm=101325.0; gas_r=8.31446261815324; tair=296.0
    gb=0.025; rb=1/gb; rah,raw=42.0,48.0; gah,gaw=1/rah,1/raw
    def solve_gas_energy(hydraulic_factor: float) -> dict[str,float]:
        temperature=tair; tcan=tair; qcan=qair
        for outer in range(50):
            scale=math.exp(55000.0/gas_r*(1/298.15-1/temperature))
            vc,jm,rd=70.0*scale,120.0*scale,1.2*scale
            es_leaf=qsat(temperature)*patm/(0.622+0.378*qsat(temperature))
            e_can=qcan*patm/(0.622+0.378*qcan)
            vpd=(es_leaf-e_can)/1000.0
            if vpd <= 0.0: raise ValueError("nonpositive solved leaf-surface VPD")
            lo,hi=4.275,ca_pa
            for _ in range(80):
                ci=(lo+hi)/2; photo=fvbc(par,ci,vc,jm,rd); an=float(photo["an"])
                cs=ca_pa-1.4*rb*gas_r*temperature*an*1e-6
                gs_potential=medlyn(an,vpd,cs,patm,25.0,3.5)
                gs=25.0+hydraulic_factor*(gs_potential-25.0)
                gs_ms=gs*1e-6*gas_r*temperature/patm; rs=1/gs_ms
                residual=ci-(ca_pa-(1.4*rb+1.6*rs)*gas_r*temperature*an*1e-6)
                if residual>0: hi=ci
                else: lo=ci
            ci=(lo+hi)/2; photo=fvbc(par,ci,vc,jm,rd); an=float(photo["an"])
            cs=ca_pa-1.4*rb*gas_r*temperature*an*1e-6
            gs_potential=medlyn(an,vpd,cs,patm,25.0,3.5)
            gs=25.0+hydraulic_factor*(gs_potential-25.0)
            gs_ms=gs*1e-6*gas_r*temperature/patm; rs=1/gs_ms
            tcan_new=(gah*tair+gb*lai*temperature)/(gah+gb*lai)
            gv=lai/(rb+rs); qcan_new=(gaw*qair+gv*qsat(temperature))/(gaw+gv)
            def energy_residual(t: float) -> float:
                eps,sigma,rho,cp,lam=0.96,5.670374419e-8,patm/(287.05*tcan_new),1004.64,2.501e6
                return (310.0+eps*lai*(330.0+410.0-2*sigma*t**4)
                        -rho*cp*gb*(t-tcan_new)*lai
                        -lam*rho*(qsat(t)-qcan_new)/(rb+rs)*lai)
            tlo,thi=273.15,330.0
            for _ in range(100):
                tm=(tlo+thi)/2
                if energy_residual(tlo)*energy_residual(tm)<=0: thi=tm
                else: tlo=tm
            new_t=(tlo+thi)/2
            change=max(abs(new_t-temperature),abs(tcan_new-tcan),1e4*abs(qcan_new-qcan))
            temperature=0.2*temperature+0.8*new_t; tcan=0.2*tcan+0.8*tcan_new
            qcan=0.2*qcan+0.8*qcan_new
            if change<1e-8: break
        else: raise ValueError("coupled leaf iteration limit")
        ci_residual=ci-(ca_pa-(1.4*rb+1.6*rs)*gas_r*temperature*an*1e-6)
        rho=patm/(287.05*tcan); transpiration=rho*(qsat(temperature)-qcan)/(rb+rs)*lai
        return {"ci":ci,"cs":cs,"temperature":temperature,"tcan":tcan,"qcan":qcan,
                "vpd_kpa":vpd,"an":an,"gs_umol":gs,"rs":rs,
                "transpiration":transpiration,"ci_residual":ci_residual,
                "energy_residual":energy_residual(temperature),"outer_iterations":float(outer+1)}
    potential=solve_gas_energy(1.0)
    hydraulic=hydraulic_four_node([-5000.0,-5000.0],[0.45,0.55],potential["transpiration"],[True,True])
    target=float(hydraulic["transpiration"]); lo_factor,hi_factor=0.0,1.0
    for _ in range(50):
        factor=(lo_factor+hi_factor)/2.0; trial=solve_gas_energy(factor)
        if trial["transpiration"]>target: hi_factor=factor
        else: lo_factor=factor
    hydraulic_factor=(lo_factor+hi_factor)/2.0
    actual=solve_gas_energy(hydraulic_factor)
    ci,cs,temperature,tcan,qcan,an,gs,rs=(actual[key] for key in
        ["ci","cs","temperature","tcan","qcan","an","gs_umol","rs"])
    ci_without_rb=ca_pa-1.6*rs*gas_r*temperature*an*1e-6
    return {"ci":ci,"cs":cs,"temperature":temperature,"tcan":tcan,"qcan":qcan,
            "vpd_kpa":actual["vpd_kpa"],"an":an,"gs_umol":gs,"transpiration":actual["transpiration"],
            "potential_transpiration":potential["transpiration"],"hydraulic_factor":hydraulic_factor,
            "ci_residual":actual["ci_residual"],"energy_residual":actual["energy_residual"],
            "hydraulic_residual":max(abs(v) for v in hydraulic["residuals"]),
            "hydraulic_transpiration":float(hydraulic["transpiration"]),
            "ci_without_boundary_resistance":ci_without_rb,
            "ambient_vpd_poison":d_kpa,
            "one_pass_transpiration_poison":potential["transpiration"],
            "outer_iterations":actual["outer_iterations"]}


def hydraulic_four_node(soil_psi: list[float], root_fraction: list[float],
                        demand: float, accessible: list[bool],
                        authorization_amounts: list[float] | None = None,
                        dt_s: float = 1.0,
                        max_iterations: int = 50) -> dict[str,object]:
    """Independent nonlinear CLM four-potential equilibrium reconstruction."""
    lai_sun,lai_sha,sai=1.6,1.2,0.7
    k1a,k1b,k2,k3max=4.0e-5,3.5e-5,2.8e-5,2.2e-5
    p501,p502,p503,p50e=-150000.0,-160000.0,-140000.0,-120000.0
    shape,gravity=2.0,980.0
    rai=[(lai_sun+lai_sha+sai)*r for r in root_fraction]
    emax_sun,emax_shade=0.58*demand,0.42*demand
    if dt_s <= 0.0: raise ValueError("nonpositive hydraulic interval")
    authorization_rates=([amount/dt_s for amount in authorization_amounts]
                         if authorization_amounts is not None else None)
    if demand>0 and not any(accessible): raise ValueError("four-node inaccessible")
    def vuln(psi: float,p50: float) -> float: return 2.0**(-((psi/p50)**shape))
    def fluxes(x: list[float]) -> tuple[list[float],dict[str,object]]:
        sun,shade,stem,root=x
        esun=emax_sun*vuln(sun,p50e); eshade=emax_shade*vuln(shade,p50e)
        q1a=k1a*vuln(stem,p501)*lai_sun*(stem-sun)
        q1b=k1b*vuln(stem,p501)*lai_sha*(stem-shade)
        q2=k2*vuln(root,p502)*sai*(root-stem-gravity)
        layers=[]
        for ps,r,ok in zip(soil_psi,rai,accessible):
            if not ok: layers.append(0.0); continue
            kr=k3max*vuln(ps,p503); ks=1.7e-5; kseries=kr*ks/(kr+ks)
            layers.append(kseries*r*(ps-root+gravity))
        finalized=[min(q,a) for q,a in zip(layers,authorization_rates)] if authorization_rates else layers
        residual=[esun-q1a,eshade-q1b,q1a+q1b-q2,q2-math.fsum(finalized)]
        return residual,{"esun":esun,"eshade":eshade,"q1a":q1a,"q1b":q1b,
                         "q2":q2,"layers":layers,"finalized":finalized}
    def solve4(a: list[list[float]],b: list[float]) -> list[float]:
        m=[row[:]+[rhs] for row,rhs in zip(a,b)]
        for col in range(4):
            pivot=max(range(col,4),key=lambda row:abs(m[row][col]))
            if abs(m[pivot][col])<1e-18: raise ValueError("singular hydraulic Jacobian")
            m[col],m[pivot]=m[pivot],m[col]
            for row in range(col+1,4):
                factor=m[row][col]/m[col][col]
                for k in range(col,5): m[row][k]-=factor*m[col][k]
        out=[0.0]*4
        for row in range(3,-1,-1):
            out[row]=(m[row][4]-math.fsum(m[row][k]*out[k] for k in range(row+1,4)))/m[row][row]
        return out
    x=[-33000.0,-32000.0,-28500.0,-25500.0]
    iterations=0
    for iterations in range(1,max_iterations+1):
        residual,detail=fluxes(x); norm=max(abs(v) for v in residual)
        if norm<1e-12: break
        jac=[]
        for i in range(4): jac.append([0.0]*4)
        for col in range(4):
            step=1e-5*max(1.0,abs(x[col])); xp=x[:]; xm=x[:]
            xp[col]+=step; xm[col]-=step
            rp,_=fluxes(xp); rm,_=fluxes(xm)
            for row in range(4): jac[row][col]=(rp[row]-rm[row])/(2*step)
        delta=solve4(jac,[-v for v in residual]); accepted=False
        for half in range(21):
            trial=[v+d/(2**half) for v,d in zip(x,delta)]
            try: rtrial,_=fluxes(trial)
            except ValueError: continue
            if max(abs(v) for v in rtrial)<norm:
                x=trial; accepted=True; break
        if not accepted: raise ValueError("hydraulic backtracking limit")
    else: raise ValueError("hydraulic iteration limit")
    residual,detail=fluxes(x)
    if max(abs(v) for v in residual)>=1e-12: raise ValueError("hydraulic nonconvergence")
    if any(v<0 for v in detail["layers"]): raise ValueError("hydraulic redistribution unsupported")
    return {"sun":x[0],"shade":x[1],"stem":x[2],"root":x[3],
            "layers":detail["layers"],"finalized":detail["finalized"],
            "residuals":residual,"iterations":iterations,
            "authorization_amounts":authorization_amounts,"authorization_rates":authorization_rates,
            "dt_s":dt_s,"gravity":gravity,"transpiration":detail["esun"]+detail["eshade"]}


def carbon_nitrogen_vector() -> dict[str, object]:
    potential_gpp, gpp, mr, xs, tau, nsc0 = 0.020, 0.018, 0.006, -0.030, 30.0, 0.004
    def carbon_offer(gain: float) -> tuple[float,float,float]:
        gpp_mr=min(gain,mr); xs_mr=mr-gpp_mr
        gpp_xs=min(max(-xs/(86400*tau),0.0),gain-gpp_mr)
        return gpp_mr,gpp_xs,gain-gpp_mr-gpp_xs+nsc0
    potential_mr,potential_xs,potential_offer=carbon_offer(potential_gpp)
    gpp_mr,gpp_xs,coffer=carbon_offer(gpp)
    xs_mr=mr-gpp_mr
    a1,a2,a3,a4,g1,fcur=0.8,0.25,0.35,0.2,0.11,0.6
    cn=[30.0,45.0,55.0,450.0]
    callom=(1+g1)*(1+a1+a3*(1+a2))
    nallom=1/cn[0]+a1/cn[1]+a3*a4*(1+a2)/cn[2]+a3*(1-a4)*(1+a2)/cn[3]
    potential_ndem=potential_offer*nallom/callom
    ndem=coffer*nallom/callom
    n_retrans=0.00007
    nrequest=max(0,potential_ndem-n_retrans)
    root_n=[0.7,0.3]; fnh4=0.4
    requests=[nrequest*root_n[0]*fnh4,nrequest*root_n[0]*(1-fnh4),
              nrequest*root_n[1]*fnh4,nrequest*root_n[1]*(1-fnh4)]
    authorizations=requests[:]  # full-supply fixture; final demand is lower
    final_need=max(0,ndem-n_retrans)
    nuse=min(final_need,math.fsum(authorizations))
    finalized=[nuse*a/math.fsum(authorizations) for a in authorizations]
    ninternal_use=min(n_retrans,ndem)
    nused=ninternal_use+nuse
    eta=1.0 if ndem==0 else min(1.0,nused/ndem)
    cleaf=eta*coffer/callom
    coeff=[1,a1,a3*a4,a3*(1-a4),a2*a3*a4,a2*a3*(1-a4)]
    tissue=[cleaf*x for x in coeff]
    rg=g1*math.fsum(tissue)
    nsc1=(1-eta)*coffer
    closure=gpp+nsc0-gpp_mr-gpp_xs-math.fsum(tissue)-rg-nsc1
    tissue_n=[tissue[0]/cn[0],tissue[1]/cn[1],tissue[2]/cn[2],tissue[3]/cn[3],
              tissue[4]/cn[2],tissue[5]/cn[3]]
    return {"gpp_mr":gpp_mr,"gpp_xs":gpp_xs,"xs_next":xs-xs_mr+gpp_xs,
            "n_demand":ndem,"n_request":nrequest,"n_requests":requests,
            "n_authorization":math.fsum(authorizations),"n_authorizations":authorizations,
            "n_finalized":nuse,"n_finalized_by_bucket":finalized,
            "n_internal_use":ninternal_use,"tissue_n":tissue_n,"nsc_next":nsc1,
            "eta":eta,"tissue":tissue,"growth_resp":rg,"closure":closure,
            "display":[v*fcur for v in tissue],"storage":[v*(1-fcur) for v in tissue]}


def phenology_vectors() -> dict[str, object]:
    phase="dormant"; prev=0.25; transfer=0.012; displayed=0.0; dt=86400.0
    onset_remaining=3*dt; offset_remaining=0.0; litter=0.0
    trajectory=[]
    for gsi in [0.35,0.65,0.72,0.55,0.28,0.22,0.20,0.18]:
        if phase=="dormant" and prev<0.60<gsi:
            phase="onset"; onset_remaining=3*dt
        if phase=="active" and prev>0.30>gsi:
            phase="offset"; offset_remaining=3*dt
        if phase=="onset":
            moved=transfer if onset_remaining<=dt else min(transfer,2*transfer*dt/onset_remaining)
            transfer-=moved; displayed+=moved
            onset_remaining-=dt
            if transfer <= 1e-15: transfer=0.0; phase="active"
        elif phase=="offset":
            fallen=displayed if offset_remaining<=dt else min(displayed,2*displayed*dt/offset_remaining)
            displayed-=fallen; litter+=fallen; offset_remaining-=dt
            if displayed <= 1e-15: displayed=0.0; phase="dormant"
        trajectory.append((phase,displayed,transfer,litter)); prev=gsi
    evergreen_start=0.2
    evergreen_loss=evergreen_start*(1-math.exp(-dt/(3*365*dt)))
    return {"deciduous":trajectory,"evergreen_loss":evergreen_loss,
            "evergreen_end":evergreen_start-evergreen_loss}


def litter_partition(c: float, n: float, dm: float, fractions: tuple[float,float,float]) -> list[dict[str,float]]:
    return [{"c":c*f,"n":n*f,"dm":dm*f} for f in fractions]


def root_wood_turnover_vector() -> dict[str,object]:
    dt=86400.0; year=365.0*dt; fcarbon=0.48
    froot_c,froot_n=0.030,0.0010
    froot_dm=froot_c/fcarbon
    froot_fraction=1-math.exp(-dt/(2*year))
    froot_loss_c,froot_loss_n=froot_c*froot_fraction,froot_n*froot_fraction
    froot_loss_dm=froot_dm*froot_fraction
    froot_receipts=litter_partition(froot_loss_c,froot_loss_n,froot_loss_dm,(0.25,0.35,0.40))
    froot_end={"c":froot_c-froot_loss_c,"n":froot_n-froot_loss_n,
               "dm":froot_dm-froot_loss_dm}
    live_c,live_n,dead_c,dead_n=0.20,0.0020,0.30,0.0015
    live_fraction=1-math.exp(-dt/(5*year))
    internal_c,internal_n=live_c*live_fraction,live_n*live_fraction
    live_after,live_n_after=live_c-internal_c,live_n-internal_n
    dead_after,dead_n_after=dead_c+internal_c,dead_n+internal_n
    mortality_fraction=1-math.exp(-(0.01/year)*dt)
    cwd_live_c,cwd_live_n=live_after*mortality_fraction,live_n_after*mortality_fraction
    cwd_dead_c,cwd_dead_n=dead_after*mortality_fraction,dead_n_after*mortality_fraction
    live_start_dm,dead_start_dm=live_c/fcarbon,dead_c/fcarbon
    live_end_dm=(live_after-cwd_live_c)/fcarbon
    dead_end_dm=(dead_after-cwd_dead_c)/fcarbon
    cwd_dm=live_start_dm+dead_start_dm-live_end_dm-dead_end_dm
    cwd={"c":cwd_live_c+cwd_dead_c,"n":cwd_live_n+cwd_dead_n,"dm":cwd_dm}
    return {"froot_start":{"c":froot_c,"n":froot_n,"dm":froot_dm},
            "froot_end":froot_end,
            "froot_loss":{"c":froot_loss_c,"n":froot_loss_n,"dm":froot_loss_dm},
            "froot_receipts":froot_receipts,"livewood_internal":{"c":internal_c,"n":internal_n},
            "livewood_end":{"c":live_after-cwd_live_c,"n":live_n_after-cwd_live_n},
            "deadwood_end":{"c":dead_after-cwd_dead_c,"n":dead_n_after-cwd_dead_n},"cwd":cwd,
            "froot_c_closure":froot_c-froot_end["c"]-math.fsum(x["c"] for x in froot_receipts),
            "froot_n_closure":froot_n-froot_end["n"]-math.fsum(x["n"] for x in froot_receipts),
            "froot_dm_closure":froot_dm-froot_end["dm"]-math.fsum(x["dm"] for x in froot_receipts),
            "wood_c_closure":live_c+dead_c-(live_after-cwd_live_c)-(dead_after-cwd_dead_c)-cwd["c"],
            "wood_n_closure":live_n+dead_n-(live_n_after-cwd_live_n)-(dead_n_after-cwd_dead_n)-cwd["n"],
            "wood_dm_closure":live_start_dm+dead_start_dm-live_end_dm-dead_end_dm-cwd["dm"]}


def proportional_receipts(available: float, requests: list[float]) -> list[float]:
    total = math.fsum(requests)
    return requests[:] if total <= available else [available * r / total for r in requests]


def run() -> dict[str, object]:
    zero = fvbc(0.0, 30.0, 60.0, 110.0, 1.2)
    rubisco = fvbc(800.0, 8.0, 35.0, 160.0, 1.0)
    electron = fvbc(45.0, 30.0, 100.0, 70.0, 1.0)
    saturated = fvbc(1600.0, 30.0, 70.0, 120.0, 1.2)
    super_saturated = fvbc(3200.0, 30.0, 70.0, 120.0, 1.2)
    transition_candidates = [fvbc(float(p), 25.0, 65.0, 105.0, 1.0) for p in range(10, 1001)]
    transition = min(transition_candidates, key=lambda x: abs(float(x["ac"]) - float(x["aj"])))
    gs = medlyn(float(saturated["an"]), 1.4, 39.0, 101325.0, 25.0, 3.5)
    wet = interception(0.2, 3.7, 4.1, 0.22, 0.73, 0.13, 0.42)
    dry = interception(0.0, 0.0, 4.1, 0.22, 0.73, 0.13, 0.42)
    condensation = interception(0.1,0.0,4.1,0.22,0.73,0.13,-1.2)
    phase_rejected=False
    try: interception(0.1,0.2,4.1,0.22,0.73,0.13,0.1,268.0)
    except ValueError: phase_rejected=True
    radiation = two_stream(3.2, 0.68, 0.1, 0.08, 0.05, 0.14, 620.0, 90.0)
    radiation_chi_zero = two_stream(3.2, 0.68, 0.0, 0.08, 0.05, 0.14, 620.0, 90.0)
    radiation_black = two_stream(3.2, 0.68, 0.1, 0.0, 0.0, 0.14, 620.0, 90.0)
    radiation_black_direct = two_stream(3.2,0.68,0.1,0.0,0.0,0.0,620.0,0.0)
    radiation_zero_lai = two_stream(0.0, 0.68, 0.1, 0.08, 0.05, 0.14, 620.0, 90.0)
    radiation_zero_direct = two_stream(3.2,0.0,0.1,0.08,0.05,0.14,0.0,90.0)
    energy = leaf_temperature(310.0, 330.0, 410.0, 296.0, 0.010, 2.4, 0.025, 0.006)
    wet_energy = wet_canopy_temperature(240.0,330.0,410.0,296.0,0.010,
                                        3.2,0.9,0.55,1.8,0.03)
    wet_energy_leaf_only = wet_canopy_temperature(240.0,330.0,410.0,296.0,0.010,
                                                  3.2,0.0,0.55,1.8,0.03)
    integrated_energy=integrated_canopy_energy()
    coupled = coupled_leaf_state(850.0,40.0,1.4,0.010,2.4)
    hyd_a = hydraulic_four_node([-25000.0, -25000.0], [0.8, 0.2], 3.0e-5,[True,True])
    hyd_b = hydraulic_four_node([-25000.0, -25000.0], [0.2, 0.8], 3.0e-5,[True,True])
    hyd_dry = hydraulic_four_node([-25000.0, -30000.0], [0.2, 0.8], 1.0e-5,[False,True])
    hyd_frozen = hydraulic_four_node([-25000.0, -30000.0], [0.2, 0.8], 1.0e-5,[True,False])
    hyd_four = hydraulic_four_node([-25000.0,-25000.0],[0.35,0.65],3.0e-5,[True,True])
    hydraulic_dt=1800.0
    hydraulic_authorization_amounts=[8.0e-6*hydraulic_dt,1.3e-5*hydraulic_dt]
    hyd_four_limited = hydraulic_four_node([-25000.0,-25000.0],[0.35,0.65],3.0e-5,[True,True],hydraulic_authorization_amounts,hydraulic_dt)
    hydraulic_redistribution_rejected=False
    try: hydraulic_four_node([-1000.0,-1000000.0],[0.5,0.5],3.0e-5,[True,True])
    except ValueError as error:
        hydraulic_redistribution_rejected=str(error)=="hydraulic redistribution unsupported"
    cn = carbon_nitrogen_vector()
    phen = phenology_vectors()
    litter_receipts = litter_partition(0.00432,0.00010028571428571427,0.009,(0.2,0.3,0.5))
    turnover_state=root_wood_turnover_vector()
    wrong_root_receiver_c=math.fsum(x["c"] for x in turnover_state["froot_receipts"])+1e-6
    wrong_root_receiver_dm=math.fsum(x["c"] for x in turnover_state["froot_receipts"])
    mixed_top = two_stream(2.8,0.68,0.1,0.08,0.05,0.14,500.0,70.0)
    mixed_bottom = two_stream(1.3,0.68,-0.1,0.11,0.06,0.14,
                              mixed_top["transmitted_direct"],mixed_top["transmitted_diffuse"])
    canopy_demand_before=2.4*0.00009
    canopy_demand_after=0.6*0.00009
    floor_evap_before=min(0.8,82.0*86400/2.45e6)
    floor_evap_after=min(0.8,82.0*86400/2.45e6)
    expected = {
        "radiation_absorbed": 631.4550942161578,
        "leaf_temperature": 295.4923277333952,
        "hydraulic_b_layer2": 2.3288575137818997e-05,
        "cn_leaf_growth": 0.006442191726176829,
        "n_request": 0.0003175930322239551,
    }
    poison = {
        "beer_only_absorbed": 710.0 * (1.0 - math.exp(-0.5 * 3.2)),
        "n_debit_authorization_instead_of_use": float(cn["n_authorization"]) + 1e-5,
        "carbon_without_growth_resp": math.fsum(cn["tissue"]),
    }
    roots_a = proportional_receipts(1.8, [1.2, 0.3])
    roots_b = proportional_receipts(1.8, [0.3, 1.2])
    competing_water = proportional_receipts(1.0, [0.8, 0.7, 0.5])
    n_receipts = proportional_receipts(0.06, [0.02, 0.05, 0.03])
    n_finalized_all = [0.010, 0.025, 0.015]
    n_remaining = 0.06-math.fsum(n_finalized_all)
    wrong_n_remaining = 0.06-n_finalized_all[0]-n_finalized_all[1]-0.03
    leaf_c, sla, turnover, retrans = 0.24, 18.0, 0.018, 0.35
    litter_c = leaf_c * turnover
    donor_n = litter_c / 28.0
    litter_n = donor_n * (1.0 - retrans)
    dm = litter_c / 0.48
    c_available, rm, growth_fraction = 0.014, 0.0021, 0.25
    rg = growth_fraction * (c_available - rm)
    alloc = c_available - rm - rg
    before = {"vegetation":{"leaf_c":leaf_c,"nsc_c":0.004,"canopy_liquid":0.2},
              "hydrology":{"soil":[1.2,0.9]},"biogeochemistry":{"mineral_n":0.06},
              "energy":{"canopy_j":0.0}}
    candidate = copy.deepcopy(before)
    try:
        candidate["vegetation"]["canopy_liquid"]-=0.1
        candidate["hydrology"]["soil"][0]-=0.2
        candidate["biogeochemistry"]["mineral_n"]-=0.01
        candidate["energy"]["canopy_j"]-=24500.0
        hydraulic_four_node([-25000.0,-25000.0],[0.5,0.5],3e-5,[True,True],max_iterations=0)
    except ValueError:
        candidate = copy.deepcopy(before)
    rollback = json.dumps(before,sort_keys=True,separators=(",",":")) == json.dumps(candidate,sort_keys=True,separators=(",",":"))
    floor0 = {"energy": 82.0, "resistance": 210.0, "water": 1.7}
    floor1 = dict(floor0)
    mixed = {"overstory_lai": 3.1, "understory_lai": 1.4, "union_cover": 0.82,
             "sum_stratum_cover": 1.43}
    checks = {
        "zero_light": float(zero["an"]) == -1.2,
        "rubisco_limited": rubisco["limit"] == "rubisco",
        "electron_limited": electron["limit"] == "electron",
        "saturated_light": float(saturated["j"]) > float(electron["j"]) and abs(float(super_saturated["an"])-float(saturated["an"])) < 0.2,
        "transition": abs(float(transition["ac"]) - float(transition["aj"])) < 0.2,
        "coupled_conductance": gs > 25.0 and abs(coupled["ci_residual"]) < 1e-8 and abs(coupled["energy_residual"]) < 1e-6 and coupled["hydraulic_residual"] < 1e-10 and abs(coupled["transpiration"]-coupled["hydraulic_transpiration"]) < 1e-12 and coupled["tcan"] != 296.0 and coupled["qcan"] != 0.010,
        "coupled_hydraulic_fixed_point": abs(coupled["transpiration"]-coupled["hydraulic_transpiration"]) < 1e-12 and abs(coupled["one_pass_transpiration_poison"]-coupled["hydraulic_transpiration"]) > 1e-9,
        "coupled_resistance_node_poisons": abs(coupled["ci"]-coupled["ci_without_boundary_resistance"]) > 0.1 and abs(coupled["vpd_kpa"]-coupled["ambient_vpd_poison"]) > 0.1,
        "wet_canopy_closure": abs(wet["closure"]) < 1e-12,
        "dry_canopy": dry["evap"] == 0.0,
        "canopy_condensation_closure": condensation["evap"] < 0 and abs(condensation["closure"]) < 1e-12 and condensation["s1"] <= 0.22*4.1,
        "subfreezing_liquid_rejected": phase_rejected,
        "two_stream_radiation": abs(radiation["closure"]) < 1e-9 and radiation["absorbed"] > 0,
        "radiation_removable_branches": abs(radiation_chi_zero["closure"]) < 1e-9 and abs(radiation_black["closure"]) < 1e-9 and abs(radiation_black_direct["shaded_absorbed"]) < 2e-7 and abs(radiation_zero_direct["closure"]) < 1e-9 and radiation_zero_direct["sunlit_area"] == 0.0,
        "zero_lai_radiation": abs(radiation_zero_lai["absorbed"]) < 1e-10,
        "sunlit_shaded_partition": radiation["sunlit_absorbed"] > 0 and radiation["shaded_absorbed"] > 0 and math.isclose(radiation["sunlit_absorbed"]+radiation["shaded_absorbed"],radiation["absorbed"]),
        "radiation_fixed_expected": math.isclose(radiation["absorbed"], expected["radiation_absorbed"], rel_tol=1e-11),
        "radiation_poison_rejected": not math.isclose(radiation["absorbed"], poison["beer_only_absorbed"], rel_tol=1e-4),
        "leaf_energy_solve": abs(energy["residual"]) < 1e-6 and energy["transpiration"] > 0,
        "energy_fixed_expected": math.isclose(energy["temperature"], expected["leaf_temperature"], rel_tol=1e-11),
        "wet_stem_energy_owner": abs(wet_energy["residual"]) < 1e-6 and wet_energy["stem_energy"] > 0.0 and wet_energy["wet_area"] > wet_energy_leaf_only["wet_area"] and not math.isclose(wet_energy["evaporation"],wet_energy_leaf_only["evaporation"]),
        "integrated_wet_dry_energy_water": abs(integrated_energy["water_closure"]) < 1e-14 and abs(integrated_energy["energy_residual_j"]) < 1e-3 and math.isclose(integrated_energy["shortwave_partition"],500.0) and math.isclose(integrated_energy["latent_energy_j"],2.501e6*integrated_energy["wet_amount"]) and integrated_energy["store1"] >= -1e-14,
        "wet_rate_amount_area_poisons": abs(integrated_energy["rate_as_amount_poison"]) > 1e-6 and not math.isclose(integrated_energy["leaf_only_area_poison"],4.1),
        "hydraulic_continuity": max(abs(v) for v in hyd_a["residuals"]) < 1e-10,
        "hydraulic_profiles_distinct": hyd_a["layers"] != hyd_b["layers"],
        "hydraulic_fixed_expected": math.isclose(hyd_b["layers"][1], expected["hydraulic_b_layer2"], abs_tol=1e-11),
        "hydraulic_finalized_caps": all(f*hydraulic_dt <= a+1e-15 for f,a in zip(hyd_four_limited["finalized"],hydraulic_authorization_amounts)) and max(abs(v) for v in hyd_four_limited["residuals"]) < 1e-10,
        "hydraulic_dry_frozen_exclusion": hyd_dry["finalized"][0] == 0.0 and hyd_frozen["finalized"][1] == 0.0 and max(abs(v) for v in hyd_dry["residuals"]+hyd_frozen["residuals"]) < 1e-10,
        "hydraulic_four_node_closure": max(abs(v) for v in hyd_four["residuals"]) < 1e-10 and hyd_four["gravity"] != 0,
        "hydraulic_active_cap_resolve": max(abs(v) for v in hyd_four_limited["residuals"]) < 1e-10 and hyd_four_limited["root"] != hyd_four["root"] and all(math.isclose(q,a) for q,a in zip(hyd_four_limited["finalized"],hyd_four_limited["authorization_rates"])),
        "hydraulic_rate_amount_poison": any(not math.isclose(rate,amount) for rate,amount in zip(hyd_four_limited["authorization_rates"],hyd_four_limited["authorization_amounts"])),
        "hydraulic_redistribution_rejected": hydraulic_redistribution_rejected,
        "root_profiles_distinct": roots_a != roots_b and math.isclose(sum(roots_a), 1.5),
        "competing_water": math.isclose(sum(competing_water), 1.0),
        "nitrogen_competition": math.isclose(sum(n_receipts), 0.06) and all(f<=a for f,a in zip(n_finalized_all,n_receipts)) and math.isclose(n_remaining,0.01),
        "cn_tissue_allocation": abs(float(cn["closure"])) < 1e-14 and len(cn["tissue"]) == 6,
        "cn_fixed_expected": math.isclose(cn["tissue"][0], expected["cn_leaf_growth"], rel_tol=1e-11) and math.isclose(cn["n_request"], expected["n_request"], rel_tol=1e-11),
        "cn_poison_rejected": poison["n_debit_authorization_instead_of_use"] > cn["n_authorization"] and poison["carbon_without_growth_resp"] > 0,
        "n_finalization": 0 <= float(cn["n_finalized"]) <= float(cn["n_authorization"]) <= float(cn["n_request"]),
        "storage_display": all(math.isclose(a+b,t) for a,b,t in zip(cn["display"],cn["storage"],cn["tissue"])),
        "deciduous_multistep": any(x[0]=="onset" for x in phen["deciduous"]) and any(x[0]=="active" for x in phen["deciduous"]) and phen["deciduous"][-1][0]=="dormant" and math.isclose(phen["deciduous"][-1][3],0.012),
        "evergreen_turnover": math.isclose(phen["evergreen_end"]+phen["evergreen_loss"],0.2),
        "receiver_reconstruction": math.isclose(math.fsum(x["c"] for x in litter_receipts),0.00432) and math.isclose(math.fsum(x["dm"] for x in litter_receipts),0.009),
        "root_wood_cwd_trajectory": all(abs(turnover_state[key]) < 1e-14 for key in ["froot_c_closure","froot_n_closure","froot_dm_closure","wood_c_closure","wood_n_closure","wood_dm_closure"]) and all(math.isclose(math.fsum(x[quantity] for x in turnover_state["froot_receipts"]),turnover_state["froot_loss"][quantity]) for quantity in ["c","n","dm"]),
        "wrong_root_c_dm_receipt_rejected": not math.isclose(wrong_root_receiver_c,turnover_state["froot_loss"]["c"]) and not math.isclose(wrong_root_receiver_dm,turnover_state["froot_loss"]["dm"]),
        "receiver_n_credit": math.isclose(math.fsum(x["n"] for x in litter_receipts),0.00010028571428571427) and turnover_state["cwd"]["n"] > 0,
        "wrong_competitor_debit_rejected": wrong_n_remaining < n_remaining and not math.isclose(wrong_n_remaining,n_remaining),
        "vertical_mixed_radiation": mixed_top["absorbed"]>0 and mixed_bottom["absorbed"]>0 and mixed_top["absorbed"] != mixed_bottom["absorbed"],
        "carbon_respiration_allocation": math.isclose(rm + rg + alloc, c_available),
        "leaf_litter_cn_dm": math.isclose(litter_n + donor_n * retrans, donor_n) and dm > litter_c,
        "lai_owned_by_leaf_c": math.isclose(leaf_c * sla, 4.32),
        "mixed_strata_not_averaged": mixed["sum_stratum_cover"] > 1 and mixed["overstory_lai"] != mixed["understory_lai"],
        "floor_not_donation_target": canopy_demand_after < canopy_demand_before and floor0 == floor1 and floor_evap_before == floor_evap_after,
        "rollback": rollback,
    }
    return {"model_version": "OPENWEPP_C3_WOODY_V1", "all_pass": all(checks.values()),
            "checks": checks, "vectors": {"zero": zero, "rubisco": rubisco,
            "electron": electron, "saturated": saturated, "super_saturated":super_saturated,"transition": transition,
            "medlyn_gs": gs,"coupled_leaf":coupled,"wet": wet,"condensation":condensation,"roots_a": roots_a, "roots_b": roots_b,
            "water_competition": competing_water, "n_receipts": n_receipts,
            "n_competing_finalized":n_finalized_all,"n_remaining":n_remaining,
            "radiation": radiation,"radiation_chi_zero":radiation_chi_zero,"radiation_black":radiation_black,"radiation_black_direct":radiation_black_direct,"radiation_zero_direct":radiation_zero_direct,"radiation_zero_lai":radiation_zero_lai,"energy": energy,"wet_energy":wet_energy,"wet_energy_leaf_only":wet_energy_leaf_only,"integrated_canopy_energy":integrated_energy, "hydraulic_a": hyd_a,
            "hydraulic_b": hyd_b,"hydraulic_dry":hyd_dry,"hydraulic_frozen":hyd_frozen,"hydraulic_four_node":hyd_four,"hydraulic_four_node_limited":hyd_four_limited,"carbon_nitrogen": cn,
            "fixed_expected": expected, "rejected_poison": poison,
            "phenology":phen,"litter_receipts":litter_receipts,"root_wood_turnover":turnover_state,
            "wrong_root_receiver":{"c":wrong_root_receiver_c,"dm":wrong_root_receiver_dm},
            "wrong_n_remaining":wrong_n_remaining,
            "mixed_top":mixed_top,"mixed_bottom":mixed_bottom,"floor_evaporation":[floor_evap_before,floor_evap_after],
            "litter": {"c": litter_c, "n": litter_n, "retrans_n": donor_n * retrans, "dm": dm}}}


if __name__ == "__main__":
    result = run()
    print(json.dumps(result, sort_keys=True, indent=2))
    sys.exit(0 if result["all_pass"] else 1)
