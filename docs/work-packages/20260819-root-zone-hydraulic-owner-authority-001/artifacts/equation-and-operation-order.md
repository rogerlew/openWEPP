# Equation and operation order

1. `theta=liquid/thickness`
2. `S_raw=theta/porosity`
3. `S=min(1,max(0,S_raw))`
4. `S_psi=max(0.01,S)`
5. `psi=max(psi_sat*pow(S_psi,-B),-1e8)`
6. `exponent=2*B+3`
7. `K=min(Ksat,Ksat*pow(S,exponent))`
8. `K_mm_s=1000*K`
9. `node=ordered_top+0.5*thickness`
10. `gravity_mm=-1000*node`
11. `z3_mm=1000*(node+required_stratum_lateral_path)`

`libm 0.2.16::pow`; exact binary64-bit comparison; positive-zero normalization.
