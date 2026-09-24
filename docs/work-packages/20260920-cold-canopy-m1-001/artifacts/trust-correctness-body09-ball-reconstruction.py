import json, math, struct
from pathlib import Path

OUT = Path('docs/work-packages/20260920-cold-canopy-m1-001/artifacts/trust-correctness-body09-ball-reconstruction.json')

def bits(x):
    return f'{struct.unpack(">Q", struct.pack(">d", x))[0]:016x}'

def tagged(x):
    return {'decimal': repr(x), 'hex': x.hex(), 'bits': bits(x)}

def solve(normalizer):
    rows, cols = 21, 2
    raw_r = [2.2, 2.6] + [0.0] * 19
    raw_j = [[0.0, 0.0] for _ in range(rows)]
    raw_j[0] = [1.0, 2.0]
    raw_j[1] = [3.0, 1.0]
    scale = [1.0e-5, 1.0e-5]
    weight = 1.0 / normalizer
    y = [weight * x for x in raw_r]
    a = [[(weight * raw_j[i][j]) * scale[j] for j in range(cols)] for i in range(rows)]
    original = [row[:] for row in a]
    b = [row[:] for row in a]
    v = [[1.0, 0.0], [0.0, 1.0]]
    visits=[]
    rotations=0
    sweeps=0
    for sweep in range(1,65):
        any_rotation=False
        for left in range(cols):
            for right in range(left+1,cols):
                alpha=beta=gamma=0.0
                for row in range(rows):
                    aa=b[row][left]*b[row][left]
                    bb=b[row][right]*b[row][right]
                    gg=b[row][left]*b[row][right]
                    alpha += aa; beta += bb; gamma += gg
                threshold=(2.0**-48)*math.sqrt(alpha*beta)
                rotate=abs(gamma)>threshold
                visits.append({'sweep':sweep,'left':left,'right':right,'rotated':rotate})
                if rotate:
                    tau=(beta-alpha)/(2.0*gamma)
                    hypot=math.hypot(tau,1.0)
                    divisor=abs(tau)+hypot
                    t=1.0 if tau==0.0 else math.copysign(1.0,tau)/divisor
                    c=1.0/math.hypot(t,1.0)
                    s=c*t
                    for row in range(rows):
                        x=b[row][left]; z=b[row][right]
                        b[row][left]=c*x-s*z
                        b[row][right]=s*x+c*z
                    for row in range(cols):
                        x=v[row][left]; z=v[row][right]
                        v[row][left]=c*x-s*z
                        v[row][right]=s*x+c*z
                    rotations += 1; any_rotation=True
        sweeps=sweep
        if not any_rotation: break
    sigma=[]
    for col in range(cols):
        total=0.0
        for row in range(rows): total += b[row][col]*b[row][col]
        sigma.append(math.sqrt(total))
    free_ids=[5,11]
    order=sorted(range(cols), key=lambda j:(-sigma[j],free_ids[j]))
    def step(lam):
        coeff=[0.0]*cols
        for sorted_index in range(cols):
            j=order[sorted_index]
            uty=0.0
            for row in range(rows):
                av=0.0
                for free in range(cols): av += original[row][free]*v[free][j]
                u=av/sigma[j]
                uty += u*y[row]
            denom=sigma[j]*sigma[j]+lam
            gain=sigma[j]/denom
            coeff[j]=-(gain*uty)
        out=[]
        for coordinate in range(cols):
            total=0.0
            for sorted_index in range(cols):
                j=order[sorted_index]
                total += v[coordinate][j]*coeff[j]
            out.append(total)
        return out
    def norm(p):
        total=0.0
        for x in p: total += x*x
        return math.sqrt(total)
    radius=1.0
    p0=step(0.0); n0=norm(p0)
    lo=0.0; hi=1.0; high=step(hi); nh=norm(high)
    bracket_evals=[(0.0,n0),(hi,nh)]
    multiplications=0
    for _ in range(48):
        if nh<=radius: break
        lo=hi; hi*=4.0; multiplications += 1
        high=step(hi); nh=norm(high); bracket_evals.append((hi,nh))
    initial_feasible=[lo,hi]
    mids=[]
    for _ in range(48):
        mid=(lo+hi)*0.5
        candidate=step(mid); nc=norm(candidate)
        mids.append((mid,nc,nc<=radius))
        if nc<=radius:
            hi=mid; high=candidate; nh=nc
        else:
            lo=mid
    final_norm=norm(high)
    gap=radius-final_norm
    tolerance=max((2.0**-40)*radius,64.0*math.ulp(1.0)*max(1.0,radius))
    return {
      'normalizer':tagged(normalizer),'weight':tagged(weight),
      'sweeps':sweeps,'rotations':rotations,'visits':visits,
      'singular_values':[tagged(x) for x in sigma],'sorted_from_unsorted':order,
      'lambda_zero_norm':tagged(n0),'bracket_multiplications':multiplications,
      'bracket_before_bisection':[tagged(x) for x in initial_feasible],
      'bracket_evaluations':[{'lambda':tagged(x),'norm':tagged(n)} for x,n in bracket_evals],
      'bisection_count':len(mids),
      'last_three_bisections':[{'lambda':tagged(x),'norm':tagged(n),'feasible':f} for x,n,f in mids[-3:]],
      'final_lower_lambda':tagged(lo),'final_upper_lambda':tagged(hi),
      'upper_step':[tagged(x) for x in high],'upper_norm':tagged(final_norm),
      'radius_minus_norm':tagged(gap),'tolerance':tagged(tolerance),
      'accepted_by_frozen_guard': final_norm<=radius and gap<=tolerance,
      'refusal_kind': None if final_norm<=radius and gap<=tolerance else 'TrustRegionBallAccuracy'
    }

record={
 'schema':'OPENWEPP_TRUST_CORRECTNESS_BODY09_BALL_RECONSTRUCTION_V1',
 'evidence_class':'independent non-target binary64 arithmetic reconstruction',
 'source_tree_sha256':'74f44e6972332e250f9233e4b2225a99b9b1930dc1559453485de206de55c0f4',
 'target_code_imported':False,
 'physical_target_run':False,
 'frozen_inputs':{
   'raw_residual_first_two':[2.2,2.6],
   'raw_jacobian_first_two_by_drainage_columns':[[1.0,2.0],[3.0,1.0]],
   'coordinate_scales':[1.0e-5,1.0e-5],
   'base_drainage':[2.0e-5,3.0e-5],
   'radius':1.0,
   'bisections':48,
   'ball_gap_factor':'2^-40'
 },
 'cases':[solve(1.0),solve(1.0e-4)]
}
OUT.write_text(json.dumps(record,indent=2,sort_keys=True)+'\n')
print(OUT)
for c in record['cases']:
    print(c['normalizer']['decimal'], c['final_upper_lambda']['decimal'], c['upper_norm']['decimal'], c['radius_minus_norm']['decimal'], c['tolerance']['decimal'], c['accepted_by_frozen_guard'], c['upper_step'][0]['decimal'], c['upper_step'][1]['decimal'])
