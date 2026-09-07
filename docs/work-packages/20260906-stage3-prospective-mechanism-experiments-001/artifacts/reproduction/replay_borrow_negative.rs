// Append only inside the actual private component_replay_oracle test include.
#[cfg(test)]
fn replay_negative_second_consumption(base: &ValidatedCoveredJacobianBase<'_>) {
    let h = f64::EPSILON.sqrt() * base.trial[6].abs().max(1.0);
    let pair = CoveredCanonicalProbePair::new(base, 6, h).unwrap();
    let stencil = pair.stencil;
    let cap = pair.plus.unwrap();
    let _ = cap.evaluate_for(base, 6, CoveredProbeSign::Plus, h, stencil);
    let _ = cap.evaluate_for(base, 6, CoveredProbeSign::Plus, h, stencil); // E0382
}

#[cfg(test)]
fn replay_negative_mutate_borrow(mut base: ValidatedCoveredJacobianBase<'_>) {
    let h = f64::EPSILON.sqrt() * base.trial[6].abs().max(1.0);
    let pair = CoveredCanonicalProbePair::new(&base, 6, h).unwrap();
    base.trial[6] += h; // E0502
    let _ = pair.plus.unwrap().evaluate_for(&base, 6, CoveredProbeSign::Plus, h, pair.stencil);
}

#[cfg(test)]
fn replay_negative_drop_borrow(base: ValidatedCoveredJacobianBase<'_>) {
    let h = f64::EPSILON.sqrt() * base.trial[6].abs().max(1.0);
    let pair = CoveredCanonicalProbePair::new(&base, 6, h).unwrap();
    drop(base); // E0505
    drop(pair);
}
