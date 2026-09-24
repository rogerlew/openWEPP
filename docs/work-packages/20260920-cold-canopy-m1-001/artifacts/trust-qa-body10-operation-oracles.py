import math, struct, json, pathlib

def bits(x): return f'0x{struct.unpack(">Q", struct.pack(">d",x))[0]:016x}'
def hx(x): return x.hex()
# Jacobi three-column fixture: first pair receives an isolated rotation;
# col 2 is independent.  Choose desired t then derive tau and z.
t_target=float.fromhex('0x1.779b97f4a7c15p-1')
tau_target=(1.0-t_target*t_target)/(2.0*t_target)
z=math.sqrt(2.0*tau_target)
# B = [[1,1,0],[0,z,0],[0,0,1]]. Pair (0,1): alpha=1,beta=1+z*z,gamma=1.
alpha=1.0; beta=1.0+z*z; gamma=1.0
tau=(beta-alpha)/(2.0*gamma)
t=math.copysign(1.0,tau)/(abs(tau)+math.hypot(1.0,tau))
c_hypot=1.0/math.hypot(1.0,t)
c_sqrt=1.0/math.sqrt(1.0+t*t)
s=c_hypot*t
# V begins identity; after isolated 0,1 rotation V[0][0]=c.
# Merit fixture: square roots of independently selected binary64 sums. The
# wrapper recomputes their squares in checked_sum_squares.
a_target=float.fromhex('0x0.1705006573f57p-1022')
b_target=float.fromhex('0x1.08e299691f466p-1018')
x=math.sqrt(a_target); y=math.sqrt(b_target)
a=x*x; b=y*y
grouped=0.5*(a-b); split=0.5*a-0.5*b
assert grouped != split
result={
 'jacobi': {'matrix': [[1.0,1.0,0.0],[0.0,z,0.0],[0.0,0.0,1.0]], 'tau': [hx(tau),bits(tau)], 't':[hx(t),bits(t)], 'c_hypot':[hx(c_hypot),bits(c_hypot)], 'c_sqrt':[hx(c_sqrt),bits(c_sqrt)], 'expected_v00_bits':bits(c_hypot), 'expected_pair_visits':6},
 'merit': {'raw_residual':[hx(x),'0x0.0p+0'], 'raw_jacobian':[[hx(y-x),'0x0.0p+0'],['0x0.0p+0','0x0.0p+0']], 'normalizers':['0x1.0p+0','0x1.0p+0'], 'scales':['0x1.0p+0','0x1.0p+0'], 'step':['0x1.0p+0','0x0.0p+0'], 'candidate_raw_residual':[hx(y),'0x0.0p+0'], 'moving_normalizers':['0x1.0p+0','0x1.0p+0'], 'f2':[hx(a),bits(a)], 'linear2':[hx(b),bits(b)], 'pred_grouped':[hx(grouped),bits(grouped)], 'pred_split':[hx(split),bits(split)], 'actual_split':[hx(split),bits(split)], 'rho':[hx(split/grouped),bits(split/grouped)]}
}
p=pathlib.Path('/workdir/openWEPP/docs/work-packages/20260920-cold-canopy-m1-001/artifacts')
(p/'trust-qa-body10-operation-oracles.py').write_text(pathlib.Path(__file__).read_text())
(p/'trust-qa-body10-operation-oracles.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
# Weighting fixture: wrapper applies weight=1/normalizer then weight*raw.
# Make linear and candidate zero so reported predicted/actual expose weighted^2/2.
raw=float.fromhex('0x1.528cb464bb40cp+0')
norm=float.fromhex('0x1.de6e5fd29f054p+0')
weighted_mul=(1.0/norm)*raw
weighted_div=raw/norm
assert weighted_mul != weighted_div
pred_mul=0.5*(weighted_mul*weighted_mul)
pred_div=0.5*(weighted_div*weighted_div)
assert pred_mul != pred_div
result['weighting']={'raw_residual':[hx(raw),'0x0.0p+0'], 'raw_jacobian':[[hx(-raw),'0x0.0p+0'],['0x0.0p+0','0x0.0p+0']], 'normalizers':[hx(norm),'0x1.0p+0'], 'scales':['0x1.0p+0','0x1.0p+0'], 'step':['0x1.0p+0','0x0.0p+0'], 'candidate_raw_residual':['0x0.0p+0','0x0.0p+0'], 'moving_normalizers':['0x1.0p+0','0x1.0p+0'], 'weighted_recip_mul':[hx(weighted_mul),bits(weighted_mul)], 'weighted_div':[hx(weighted_div),bits(weighted_div)], 'pred_recip_mul':[hx(pred_mul),bits(pred_mul)], 'pred_div':[hx(pred_div),bits(pred_div)], 'actual_recip_mul':[hx(pred_mul),bits(pred_mul)], 'rho':['0x1.0p+0',bits(1.0)]}
(p/'trust-qa-body10-operation-oracles.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result['weighting'],indent=2))
# Finalize the retained script after all fixture generation code is present.
(p/'trust-qa-body10-operation-oracles.py').write_text(pathlib.Path(__file__).read_text())
