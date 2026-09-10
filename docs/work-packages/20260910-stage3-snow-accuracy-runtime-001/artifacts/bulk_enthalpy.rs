//! Isolated EXP-SNOW-ACCURACY-20260910-B mathematics, pending runner adoption.
//! Units: kg/m², J/m², metres, Kelvin, seconds; positive heat enters its owner.
pub const FUSION: f64 = 333_600.0;
pub const ICE_CAPACITY: f64 = 2_100.0;
// Same liquid-reference heat capacity as terminal_liquid_thermodynamics_v1.
pub const WATER_CAPACITY: f64 = 4_218.0;
pub const FREEZE: f64 = 273.15;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error { Domain, NoRepresentedIce, VaporCapacity, Phase, Conservation }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State { pub water: f64, pub enthalpy: f64, pub depth: f64 }
#[derive(Clone, Copy, Debug)]
pub struct Layer { pub ice:f64, pub liquid:f64, pub cold:f64, pub depth:f64 }
#[derive(Clone, Copy, Debug)]
pub struct Forcing {
    pub seconds:f64, pub solid:f64, pub rain:f64, pub vapor:f64,
    pub shortwave:f64, pub longwave:f64, pub sensible:f64, pub latent:f64,
    pub precipitation_sensible:f64, pub fresh_density:f64,
    pub conductance:f64, pub soil_temperature:f64, pub soil_capacity:f64,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Outcome {
    pub ending:State, pub liquid_released:f64, pub sensible_released:f64,
    pub soil_heat:f64, pub snow_heat:f64, pub vapor_material:f64,
    pub melt:f64, pub refreeze:f64,
}

fn finite(values:&[f64]) -> bool { values.iter().all(|x|x.is_finite()) }
fn close(error:f64,scale:f64,absolute:f64) -> bool {
    error.is_finite() && scale.is_finite() && error.abs()<=absolute.max(1e-12*scale)
}

pub fn select_bulk(active:bool,water:f64,threshold:f64)->Result<bool,Error> {
    if !finite(&[water,threshold]) || water<0.0 || ![0.1,1.0,5.0].contains(&threshold) { return Err(Error::Domain); }
    Ok(water>0.0 && water<=threshold*if active {1.2}else{1.0})
}

pub fn from_layers(layers:&[Layer])->Result<State,Error> {
    let mut s=State {water:0.0,enthalpy:0.0,depth:0.0};
    for l in layers {
        if !finite(&[l.ice,l.liquid,l.cold,l.depth]) || [l.ice,l.liquid,l.cold,l.depth].iter().any(|x|*x<0.0) { return Err(Error::Domain); }
        s.water+=l.ice+l.liquid;
        s.enthalpy+=FUSION*l.liquid-l.cold;
        s.depth+=l.depth;
    }
    if !finite(&[s.water,s.enthalpy,s.depth]) { return Err(Error::Domain); }
    Ok(s)
}

// Complete equilibrium phase partition; positive sensible excess belongs to water.
fn phase(water:f64,enthalpy:f64)->Result<(f64,f64,f64,f64),Error> {
    if !finite(&[water,enthalpy]) || water<0.0 { return Err(Error::Domain); }
    if water==0.0 {
        return if enthalpy==0.0 {Ok((0.0,0.0,0.0,0.0))}else{Err(Error::Phase)};
    }
    let fusion=FUSION*water;
    if !fusion.is_finite() {return Err(Error::Domain);}
    if enthalpy<0.0 {Ok((water,0.0,-enthalpy,0.0))}
    else if enthalpy<=fusion {
        let liquid=enthalpy/FUSION;
        Ok((water-liquid,liquid,0.0,0.0))
    } else {Ok((0.0,water,0.0,enthalpy-fusion))}
}

/// One analytic piecewise implicit exchange. These are branches of one law,
/// not iterative solvers or a nonconvergence fallback chain.
fn paired_heat(w:f64,h:f64,f:Forcing)->Result<f64,Error> {
    let dg=f.seconds*f.conductance;
    let delta=FREEZE-f.soil_temperature;
    let threshold=FUSION*w;
    let cold_a=ICE_CAPACITY*w;
    let warm_a=WATER_CAPACITY*w;
    let cold=dg*(cold_a*delta+h)/(cold_a+dg*(1.0+cold_a/f.soil_capacity));
    let plateau=dg*delta/(1.0+dg/f.soil_capacity);
    let warm=dg*(warm_a*delta+h-threshold)/(warm_a+dg*(1.0+warm_a/f.soil_capacity));
    if !finite(&[cold,plateau,warm,threshold]) {return Err(Error::Domain);}
    let q=if h-cold<0.0 {cold}
        else if h-plateau>=0.0 && h-plateau<=threshold {plateau}
        else if h-warm>threshold {warm}
        else {return Err(Error::Phase)};
    let ending_temperature=f.soil_temperature+q/f.soil_capacity;
    if !ending_temperature.is_finite() || ending_temperature<=0.0 {return Err(Error::Domain);}
    Ok(q)
}

pub fn advance(s:State,f:Forcing)->Result<Outcome,Error> {
    if !finite(&[s.water,s.enthalpy,s.depth,f.seconds,f.solid,f.rain,f.vapor,
        f.shortwave,f.longwave,f.sensible,f.latent,f.precipitation_sensible,
        f.fresh_density,f.conductance,f.soil_temperature,f.soil_capacity])
        || s.water<0.0 || s.depth<0.0 || f.seconds<=0.0 || f.solid<0.0 || f.rain<0.0
        || f.fresh_density<=0.0 || f.conductance<=0.0 || f.soil_temperature<=0.0 || f.soil_capacity<=0.0 {
        return Err(Error::Domain);
    }
    let (initial_ice,initial_liquid,_,_)=phase(s.water,s.enthalpy)?;
    if initial_ice==0.0 || s.depth==0.0 {return Err(Error::NoRepresentedIce);}
    if f.vapor < -(initial_ice+f.solid) {return Err(Error::VaporCapacity);}
    if (f.vapor==0.0 && f.latent!=0.0)
        || (f.vapor!=0.0 && (!((f.latent/f.vapor).is_finite()) || f.latent/f.vapor<=0.0)) {
        return Err(Error::Domain);
    }
    // No nonlinear temperature convergence: the bounded physical beginning
    // inventory determines one lagged boundary temperature.
    let lag=if s.enthalpy<0.0 {FREEZE+s.enthalpy/(ICE_CAPACITY*s.water)}else{FREEZE};
    if !lag.is_finite() || lag<=0.0 {return Err(Error::Domain);}
    let material=f.vapor*ICE_CAPACITY*(lag-FREEZE);
    let water=s.water+f.solid+f.rain+f.vapor;
    let external=f.shortwave+f.longwave+f.sensible+f.latent+f.precipitation_sensible+material;
    let h=s.enthalpy+external+FUSION*f.rain;
    if water<=0.0 {return Err(Error::Phase);}
    let q=paired_heat(water,h,f)?;
    let (ice,liquid,cold,sensible)=phase(water,h-q)?;
    // A finite energy balance alone does not establish a physical state.
    // Reject impossible cold inventories without clamping their enthalpy.
    if ice>0.0 && cold>=ICE_CAPACITY*ice*FREEZE {return Err(Error::Domain);}
    let depth=if ice==0.0 {0.0}else{s.depth+f.solid/f.fresh_density};
    let ending=State {water:ice,enthalpy:-cold,depth};
    let m_error=ending.water+liquid-s.water-f.solid-f.rain-f.vapor;
    let e_error=ending.enthalpy+FUSION*liquid+sensible+q-s.enthalpy-external-FUSION*f.rain;
    let m_scale=s.water+f.solid+f.rain+f.vapor.abs()+ending.water+liquid;
    let e_scale=ending.enthalpy.abs()+FUSION*liquid+sensible+q.abs()+s.enthalpy.abs()+external.abs()+FUSION*f.rain;
    if !finite(&[ending.water,ending.enthalpy,ending.depth,liquid,sensible,material])
        || !close(m_error,m_scale,1e-12) || !close(e_error,e_scale,1e-6) {return Err(Error::Conservation);}
    Ok(Outcome {ending,liquid_released:liquid,sensible_released:sensible,
        soil_heat:q,snow_heat:-q,vapor_material:material,
        melt:(liquid-initial_liquid-f.rain).max(0.0),refreeze:(initial_liquid+f.rain-liquid).max(0.0)})
}
