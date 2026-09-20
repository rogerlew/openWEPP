"""Independent high-precision two-layer wet heat/reservoir column controls.
Not a solver: all trial coordinates are prescribed and generally off solution.
"""
from decimal import Decimal as D, localcontext
from pathlib import Path
import hashlib
import json


def generate():
    with localcontext() as ctx:
        ctx.prec=60
        tf,ci,lf,rv,e0=map(D,('273.15','2106','333700','461.52','611.213476'))
        sigma,p,tcan,qcan,cp=map(D,('5.670374419e-8','90000','268','.0015','1004.64'))
        plants=list(map(D,('1.35','.9')))
        caps=list(map(D,('.297','.198')))
        gb=list(map(D,('.0186','.0184')))
        dry=[[D(x) for x in values] for values in [('267','266','268'),('269','267','270')]]
        sun=list(map(D,('.2','.1')));shade=list(map(D,('.8','.6')));stem=list(map(D,('.35','.2')))
        sw=list(map(D,('14','9'))); clump=list(map(D,('.8','.7')))
        state=list(map(D,('.018','-6385.68','.023','-7917.29')))
        rho=p/(D('287.05')*tcan)
        tau=[(-D('.8')*clump[i]*plants[i]).exp() for i in range(2)]
        def evaluate(x):
            wet=[];temp=[];emission=[];ev=[]
            for i in range(2):
                m,h=x[2*i:2*i+2]
                t=tf+(h/m+lf)/ci
                assert h < -lf*m
                f=((m/caps[i]).ln()*D(2)/3).exp()
                wet.append(f); temp.append(t)
                dry_emission=sum(area*sigma*tt**4 for area,tt in zip([sun[i],shade[i],stem[i]],dry[i]))/plants[i]
                emission.append((1-f)*dry_emission+f*sigma*t**4)
                a=ci-D('1849')
                ep=e0*((D('2834700')+a*tf)/rv*(1/tf-1/t)-a/rv*(t/tf).ln()).exp()
                qs=D('.622')*ep/(p-D('.378')*ep)
                ev.append(rho*gb[i]*plants[i]*f*(qs-qcan))
            down0=D(270)
            # Explicit two-layer boundary expansion, independent of Rust loops.
            down1=tau[0]*down0+(1-tau[0])*emission[0]
            ground=sigma*D('265.15')**4
            up1=tau[1]*ground+(1-tau[1])*emission[1]
            lw=[wet[0]*(1-tau[0])*(down0+up1-2*sigma*temp[0]**4),
                wet[1]*(1-tau[1])*(down1+ground-2*sigma*temp[1]**4)]
            sensible=[rho*cp*gb[i]*plants[i]*wet[i]*(temp[i]-tcan) for i in range(2)]
            heat=[wet[i]*sw[i]+lw[i]-sensible[i] for i in range(2)]
            r=[]
            for i in range(2):
                hv=D('2501000')+D('1849')*(temp[i]-tf)
                r.extend([x[2*i]-state[2*i]+60*ev[i],x[2*i+1]-state[2*i+1]-60*(heat[i]-ev[i]*hv)])
            return dict(temperature=temp,wet_fraction=wet,emission=emission,wet_longwave=lw,wet_sensible=sensible,q_nonvapor=heat,e=ev,residual=r)
        base=evaluate(state);columns=[]
        for k in range(4):
            step=D('1e-10') if k%2==0 else D('1e-6')
            approximations=[]
            for h in [step,step/2]:
                plus=state.copy();minus=state.copy();plus[k]+=h;minus[k]-=h
                yp=evaluate(plus);ym=evaluate(minus)
                approximations.append({key:[(a-b)/(2*h) for a,b in zip(yp[key],ym[key])] for key in ['q_nonvapor','residual']})
            for key in approximations[0]:
                assert all(abs(a-b)<D('1e-9')*max(abs(a),D(1)) for a,b in zip(approximations[0][key],approximations[1][key]))
            columns.append(dict(coordinate=['upper_M','upper_H','lower_M','lower_H'][k],step=str(step/2),**{key:list(map(str,value)) for key,value in approximations[1].items()}))
        return dict(evidence_class='Independent prescribed-trial component values, not a coupled solve',
            inputs=dict(state=list(map(str,state)),plant_area=list(map(str,plants)),capacity=list(map(str,caps)),gb=list(map(str,gb)),
                dry_temperatures=[[str(x) for x in row] for row in dry],sun_area=list(map(str,sun)),shade_area=list(map(str,shade)),stem_area=list(map(str,stem)),
                shortwave_total=list(map(str,sw)),clumping=list(map(str,clump)),p=str(p),tcan=str(tcan),qcan=str(qcan),cp_air=str(cp),lw_down='270',tground='265.15',dt='60',i='0',d='0'),
            expected={k:list(map(str,v)) for k,v in base.items()},columns=columns,relative_derivative_tolerance='1e-6',
            required_additional_check='All rows of the assembled coupled evaluator, including dry/shared-air/ground/soil, must be checked in Rust; these vectors cover both reservoirs with full wet Q dependencies.')


if __name__=='__main__':
    out=generate();out['generator_sha256']=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    path=Path(__file__).with_name('radiation-reference-m1.json');path.write_text(json.dumps(out,indent=2)+'\n');print(path)
