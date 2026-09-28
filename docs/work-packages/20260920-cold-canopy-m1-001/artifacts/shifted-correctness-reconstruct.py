#!/usr/bin/env python3
import json, math, struct, sys
from decimal import Decimal, localcontext
from fractions import Fraction
from pathlib import Path

def f64(h): return struct.unpack('>d', int(h,16).to_bytes(8,'big'))[0]
def bits(x): return f'{struct.unpack(">Q",struct.pack(">d",x))[0]:016x}'
def vec(v): return [f64(x) for x in v]
def mat(v): return [vec(r) for r in v]
def q(x): return Fraction.from_float(x)
def normal0(x): return x == 0.0 or abs(x) >= sys.float_info.min

guards=[]
def chk(x, stage, i):
    ok=math.isfinite(x) and normal0(x)
    guards.append((stage,i,ok,bits(x)))
    if not ok: raise ValueError((stage,i,x))
    return x
def mul(a,b,i):
    chk(a,'mul-left',i); chk(b,'mul-right',i); z=a*b
    if z == 0.0 and a != 0.0 and b != 0.0: raise ValueError(('underflow',i))
    return chk(z,'mul-result',i)
def add(a,b,i):
    chk(a,'add-left',i); chk(b,'add-right',i); z=a+b
    if z == 0.0 and (a != 0.0 or b != 0.0) and bits(a) != bits(-b):
        raise ValueError(('zero-add',i,bits(a),bits(b)))
    return chk(z,'add-result',i)
def upmul(a,b,i):
    a=abs(chk(a,'upmul-left',i)); b=abs(chk(b,'upmul-right',i))
    if a==0.0 or b==0.0:return 0.0
    z=chk(a*b,'upmul-product',i)
    return chk(math.nextafter(z,math.inf),'upmul-next',i)
def upadd(a,b,i):
    a=abs(chk(a,'upadd-left',i)); b=abs(chk(b,'upadd-right',i))
    if b==0.0:return a
    z=chk(a+b,'upadd-sum',i)
    return chk(math.nextafter(z,math.inf),'upadd-next',i)

src=Path(sys.argv[1]); out=Path(sys.argv[2]); d=json.loads(src.read_text()); c=d['face_capture']; r=c['refusal']
A=mat(c['weighted_matrix']); f=vec(c['weighted_residual']); p=vec(r['p']); lam=f64(r['lambda'])
lo=vec(r['scaled_lower']); hi=vec(r['scaled_upper']); lower=r['lower']; upper=r['upper']; free=r['free']
runtime_h=vec(c['operations'][-1]['h']); runtime_res=vec(c['operations'][-1]['residual']); runtime_g=vec(c['operations'][-1]['g']); runtime_lp=vec(c['operations'][-1]['lambda_times_p'])

# Exact real arithmetic at the recorded binary64 operands.
qe=[q(x) for x in f]
for row in range(21):
    for col in range(21): qe[row]+=q(A[row][col])*q(p[col])
qg=[]; qh=[]
for col in range(21):
    z=sum((q(A[row][col])*qe[row] for row in range(21)),Fraction())
    qg.append(z); qh.append(z+q(lam)*q(p[col]))

# Frozen reached operation order.
rh=[]
for row in range(21):
    s=0.0
    for col in range(21): s=add(s,mul(A[row][col],p[col],col),col)
    rh.append(add(f[row],s,row))
gh=[]; lph=[]; hh=[]
for col in range(21):
    s=0.0
    for row in range(21): s=add(s,mul(A[row][col],rh[row],col),col)
    gh.append(s); lph.append(mul(lam,p[col],col)); hh.append(add(s,lph[-1],col))

gamma=math.nextafter(87.0/(2.0**53-87.0),math.inf)
cbar=[]; tau=[]
for i in range(21):
    cb=0.0
    for row in range(21):
        inner=0.0
        for col in range(21): inner=upadd(inner,upmul(A[row][col],p[col],i),i)
        scale=upadd(f[row],inner,i)
        cb=upadd(cb,upmul(A[row][i],scale,i),i)
    cb=upadd(cb,upmul(lam,p[i],i),i)
    cbar.append(cb); tau.append(0.0 if cb==0.0 else upmul(gamma,cb,i))

classes=[]
for i in range(21):
    fixed=lo[i] == hi[i]
    if fixed: cl='Fixed'
    elif lower[i]: cl='ReleaseLower' if hh[i] < -tau[i] else 'RetainLower'
    elif upper[i]: cl='ReleaseUpper' if hh[i] > tau[i] else 'RetainUpper'
    else: cl='FreePass' if abs(hh[i]) <= tau[i] else 'FreeRefuse'
    classes.append(cl)

capopt=r['optimality']['coordinates']
replay={
 'residual_bits_match_capture':[bits(x) for x in rh]==[x for x in c['operations'][-1]['residual']],
 'g_bits_match_capture':[bits(x) for x in gh]==[x for x in c['operations'][-1]['g']],
 'lambda_p_bits_match_capture':[bits(x) for x in lph]==[x for x in c['operations'][-1]['lambda_times_p']],
 'h_bits_match_capture':[bits(x) for x in hh]==[x for x in c['operations'][-1]['h']],
 'cbar_bits_match_capture':[bits(x) for x in cbar]==[x['cbar'] for x in capopt],
 'tau_bits_match_capture':[bits(x) for x in tau]==[x['tau'] for x in capopt],
 'classes_match_capture':classes==[x['class'] for x in capopt],
}
coord=[]
for i in range(21):
    exact_float=float(qh[i]); err=q(hh[i])-qh[i]
    coord.append({'coordinate':i,'mask':'fixed' if lo[i]==hi[i] else ('lower' if lower[i] else ('upper' if upper[i] else 'free')),
      'h_hat_bits':bits(hh[i]),'tau_bits':bits(tau[i]),'class':classes[i],
      'exact_h_decimal':format(float(qh[i]),'.17g'),'exact_h_numerator':str(qh[i].numerator),'exact_h_denominator':str(qh[i].denominator),
      'evaluation_error_decimal':format(float(err),'.17g'),'evaluation_error_abs_over_tau':None if tau[i]==0 else float(abs(err)/q(tau[i])),
      'exact_abs_over_tau':None if tau[i]==0 else float(abs(qh[i])/q(tau[i]))})

# Independent exact-rational fixed-face solution at the recorded lambda.
# H and rhs are formed from exact binary64 rationals, never rounded normal equations.
ids=list(r['free_ids']); active=[i for i in range(21) if i not in ids]
y=[q(f[row])+sum((q(A[row][i])*q(p[i]) for i in active),Fraction()) for row in range(21)]
H=[]; rhs=[]
for i in ids:
    H.append([sum((q(A[row][i])*q(A[row][j]) for row in range(21)),Fraction())+(q(lam) if i==j else 0) for j in ids])
    rhs.append(-sum((q(A[row][i])*y[row] for row in range(21)),Fraction()))
def exact_solve(a,b):
    a=[row[:] + [z] for row,z in zip(a,b)]; n=len(a)
    for k in range(n):
        piv=next(i for i in range(k,n) if a[i][k])
        if piv!=k:a[k],a[piv]=a[piv],a[k]
        pk=a[k][k]
        for i in range(k+1,n):
            if not a[i][k]:continue
            m=a[i][k]/pk; a[i][k]=Fraction()
            for j in range(k+1,n+1):a[i][j]-=m*a[k][j]
    x=[Fraction() for _ in range(n)]
    for i in range(n-1,-1,-1):x[i]=(a[i][n]-sum((a[i][j]*x[j] for j in range(i+1,n)),Fraction()))/a[i][i]
    return x
qref=exact_solve(H,rhs); pref=[q(x) for x in p]
for slot,i in enumerate(ids):pref[i]=qref[slot]
pref64=[float(x) for x in pref]
def exact_h(point):
    rr=[q(f[row])+sum((q(A[row][j])*point[j] for j in range(21)),Fraction()) for row in range(21)]
    return [sum((q(A[row][i])*rr[row] for row in range(21)),Fraction())+q(lam)*point[i] for i in range(21)]
href=exact_h(pref); href64=exact_h([q(x) for x in pref64])
def decimals(fr,prec):
    with localcontext() as ctx:
        ctx.prec=prec
        return str(Decimal(fr.numerator)/Decimal(fr.denominator))
reference={
 'method':'exact Fraction Gaussian elimination of exact-binary64 shifted Gram system',
 'active_coordinates':active,'free_coordinates':ids,
 'free_stationarity_exact_zero':all(href[i]==0 for i in ids),
 'decimal_100':[decimals(x,100) for x in pref], 'decimal_200':[decimals(x,200) for x in pref],
 'once_rounded_bits':[bits(x) for x in pref64],
 'once_rounded_exact_h_decimal':[format(float(x),'.17g') for x in href64],
 'box_feasible_once_rounded':all(lo[i]<=pref64[i]<=hi[i] for i in range(21)),
 'original_ball_feasible_exact':sum((x*x for x in pref),Fraction())<=q(f64(c['initial_radius']))*q(f64(c['initial_radius'])),
 'max_abs_actual_step_error':max(abs(p[i]-pref64[i]) for i in ids),
 'per_free_abs_error':[{'coordinate':i,'abs_error':abs(p[i]-pref64[i])} for i in ids]
}
# Apply the unchanged binary64 BVLS-02 evaluation once to the once-rounded reference.
rr2=[]
for row in range(21):
    s=0.0
    for col in range(21):s=add(s,mul(A[row][col],pref64[col],col),col)
    rr2.append(add(f[row],s,row))
hh2=[]; cb2=[]; t2=[]; cl2=[]
for i in range(21):
    g2=0.0
    for row in range(21):g2=add(g2,mul(A[row][i],rr2[row],i),i)
    hh2.append(add(g2,mul(lam,pref64[i],i),i))
    cb=0.0
    for row in range(21):
        inner=0.0
        for col in range(21):inner=upadd(inner,upmul(A[row][col],pref64[col],i),i)
        cb=upadd(cb,upmul(A[row][i],upadd(f[row],inner,i),i),i)
    cb=upadd(cb,upmul(lam,pref64[i],i),i); cb2.append(cb); t2.append(0.0 if cb==0 else upmul(gamma,cb,i))
    if lo[i]==hi[i]:cl='Fixed'
    elif lower[i]:cl='ReleaseLower' if hh2[i] < -t2[i] else 'RetainLower'
    elif upper[i]:cl='ReleaseUpper' if hh2[i] > t2[i] else 'RetainUpper'
    else:cl='FreePass' if abs(hh2[i])<=t2[i] else 'FreeRefuse'
    cl2.append(cl)
reference['once_rounded_bvls02']={'h_bits':[bits(x) for x in hh2],'tau_bits':[bits(x) for x in t2],'classes':cl2,'all_coordinates_pass':all(x in ('FreePass','RetainLower','RetainUpper','Fixed') for x in cl2),'summary':{x:cl2.count(x) for x in sorted(set(cl2))}}

tr=r['lambda_trace']; br=vec(tr['bracket']); bev=[[f64(a),f64(n)] for a,n in tr['bisection_evaluations']]
bev_bits={a:(f64(a),f64(n)) for a,n in tr['bisection_evaluations']}
lo_eval=bev_bits[tr['bracket'][0]]; hi_eval=bev_bits[tr['bracket'][1]]
orig_r=f64(c['initial_radius']); face_r=f64(r['radius']); norm=f64(tr['norm']); gap=f64(tr['gap']); tol=f64(tr['tolerance'])
pn2=sum((q(x)*q(x) for x in p),Fraction()); free_ids=r['free_ids']; fn2=sum((q(p[i])*q(p[i]) for i in free_ids),Fraction())
result={
 'evidence_class':'Ran: independent Python stdlib binary64 replay and exact Fraction reconstruction; no Rust/test/candidate/physical execution',
 'source_capture':str(src),'reference_precisions_predeclared':[100,200],
 'recorded':{'lambda_bits':r['lambda'],'lambda':lam,'failure_coordinate':r['coordinate'],'reason':r['reason'],'factor_identity':r['factor']['same_invocation_identity']},
 'replay':replay,'guards':{'count':len(guards),'all_normal_or_zero':all(x[2] for x in guards)},
 'coordinates':coord,
 'failure_coordinate':coord[r['coordinate']],
 'feasibility':{'box_feasible':all(lo[i]<=p[i]<=hi[i] for i in range(21)),'active_value_equal':all((not lower[i] or bits(p[i])==bits(lo[i])) and (not upper[i] or bits(p[i])==bits(hi[i])) for i in range(21)),
  'original_radius':orig_r,'face_radius':face_r,'full_norm':math.sqrt(float(pn2)),'free_norm_exact_sqrt_float':math.sqrt(float(fn2)),'recorded_free_norm':norm,
  'original_ball_feasible_exact_squared':pn2<=q(orig_r)*q(orig_r),'face_ball_feasible_exact_squared':fn2<=q(face_r)*q(face_r),
  'recorded_gap':gap,'recorded_tolerance':tol,'gap_within_tolerance':0<=gap<=tol,'lambda_positive_normal':lam>0 and normal0(lam),'lambda_times_gap':lam*gap},
 'bracket':{'lower':br[0],'upper':br[1],'selected_is_upper_bits':r['lambda']==tr['bracket'][1],
  'lower_evaluation_norm':lo_eval[1],'upper_evaluation_norm':hi_eval[1],
  'straddles_radius':lo_eval[1]>face_r and hi_eval[1]<=face_r,'bisection_count':len(bev),'endpoint_evaluation_count':len(tr['bracket_evaluations'])},
 'classification_summary':{x:classes.count(x) for x in sorted(set(classes))},
 'fixed_lambda_exact_reference':reference,
 'conclusion':{'runtime_failure_reproduced':classes[r['coordinate']]=='FreeRefuse','exact_failure_persists':abs(qh[r['coordinate']])>q(tau[r['coordinate']]),
  'evaluation_error_within_tau':abs(q(hh[r['coordinate']])-qh[r['coordinate']])<=q(tau[r['coordinate']])}
}
out.write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'replay':replay,'failure':result['failure_coordinate'],'feasibility':result['feasibility'],'bracket':result['bracket'],'summary':result['classification_summary'],'conclusion':result['conclusion']},indent=2))
