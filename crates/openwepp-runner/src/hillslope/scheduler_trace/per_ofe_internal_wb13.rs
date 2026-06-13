const ME4_INTERNAL_WB13_IDENTITY_TOLERANCE_MM: f64 = 1.0e-11;

#[derive(Debug, Clone)]
pub(super) struct InternalPerOfeWb13Record {
    pub(super) ofe_id: usize,
    pub(super) previous_storage_total_mm: f64,
    pub(super) row: SimulationOwnedWb13Row,
    pub(super) upstream_transfer_input: TransferInput,
    pub(super) current_transfer_output: TransferOutput,
}

#[derive(Debug, Clone)]
pub(super) struct DailyInternalPerOfeWb13Collection {
    contributor_ofe_count: usize,
    records: Vec<InternalPerOfeWb13Record>,
    transfer_identity_max_abs_mm: f64,
    per_element_identity_max_abs_mm: f64,
    aggregate_transfer_cancellation_max_abs_mm: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct PerOfeInternalWb13RunSummary {
    pub(super) day_count: usize,
    pub(super) record_count: usize,
    pub(super) expected_record_count: usize,
    pub(super) transfer_identity_max_abs_mm: f64,
    pub(super) per_element_identity_max_abs_mm: f64,
    pub(super) aggregate_transfer_cancellation_max_abs_mm: f64,
}

impl PerOfeInternalWb13RunSummary {
    pub(super) fn observe_day(
        &mut self,
        collection: &DailyInternalPerOfeWb13Collection,
    ) -> Result<(), HillslopeCliError> {
        collection.require_identity_closure()?;

        self.day_count += 1;
        self.record_count += collection.records.len();
        self.expected_record_count += collection.contributor_ofe_count;
        self.transfer_identity_max_abs_mm = self
            .transfer_identity_max_abs_mm
            .max(collection.transfer_identity_max_abs_mm);
        self.per_element_identity_max_abs_mm = self
            .per_element_identity_max_abs_mm
            .max(collection.per_element_identity_max_abs_mm);
        self.aggregate_transfer_cancellation_max_abs_mm = self
            .aggregate_transfer_cancellation_max_abs_mm
            .max(collection.aggregate_transfer_cancellation_max_abs_mm);

        Ok(())
    }
}

impl DailyInternalPerOfeWb13Collection {
    pub(super) fn append_publication_rows_to(&self, rows: &mut Vec<SimulationOwnedWb13Row>) {
        rows.extend(self.records.iter().map(|record| record.row.clone()));
    }

    pub(super) fn outlet_row(&self) -> Option<&SimulationOwnedWb13Row> {
        self.records.last().map(|record| &record.row)
    }

    fn from_sequence_report(
        sequence_report: &OfeLaneSequenceExecutionReport,
        lane_areas_m2: &[f64],
        previous_storage_totals_mm: &[f64],
        context: SchedulerLifecycleContext<'_>,
    ) -> Result<Self, HillslopeCliError> {
        if sequence_report.lane_reports.len() != lane_areas_m2.len() {
            return Err(internal_wb13_failure(format!(
                "persistent OFE sequence produced {} lane reports for {} static lane areas",
                sequence_report.lane_reports.len(),
                lane_areas_m2.len()
            )));
        }
        if sequence_report.lane_reports.len() != previous_storage_totals_mm.len() {
            return Err(internal_wb13_failure(format!(
                "persistent OFE sequence produced {} lane reports for {} previous storage snapshots",
                sequence_report.lane_reports.len(),
                previous_storage_totals_mm.len()
            )));
        }

        let mut records = Vec::with_capacity(sequence_report.lane_reports.len());
        for ((lane_report, lane_area_m2), previous_storage_total_mm) in sequence_report
            .lane_reports
            .iter()
            .zip(lane_areas_m2.iter())
            .zip(previous_storage_totals_mm.iter())
        {
            let ofe_id = u16::try_from(lane_report.ofe_id).map_err(|_| {
                internal_wb13_failure(format!(
                    "OFE id {} is outside WB13 u16 domain",
                    lane_report.ofe_id
                ))
            })?;
            let row = build_simulation_owned_wb13_row_for_ofe(
                &lane_report.kernel_report.writeback_surface,
                *lane_area_m2,
                Wb13OfePublicationContext {
                    simulation_year: context.simulation_year,
                    sim_day_index: context.sim_day_index,
                    calendar_day: context.calendar_day,
                    ofe_id,
                    upstream_runon_m: lane_report.upstream_transfer_input.upstrmq,
                    qofe_override_m: Some(lane_report.current_transfer_output.qofe),
                },
            )?;

            records.push(InternalPerOfeWb13Record {
                ofe_id: lane_report.ofe_id,
                previous_storage_total_mm: *previous_storage_total_mm,
                row,
                upstream_transfer_input: lane_report.upstream_transfer_input.clone(),
                current_transfer_output: lane_report.current_transfer_output.clone(),
            });
        }

        Self::from_records(sequence_report.lane_reports.len(), records)
    }

    pub(super) fn from_records(
        contributor_ofe_count: usize,
        records: Vec<InternalPerOfeWb13Record>,
    ) -> Result<Self, HillslopeCliError> {
        if contributor_ofe_count == 0 {
            return Err(internal_wb13_failure(
                "contributor_ofe_count must be >= 1 for internal per-OFE WB13 records",
            ));
        }
        if records.len() != contributor_ofe_count {
            return Err(internal_wb13_failure(format!(
                "internal per-OFE WB13 record count {} must equal contributor count {contributor_ofe_count}",
                records.len()
            )));
        }

        let mut transfer_identity_max_abs_mm = 0.0_f64;
        let mut per_element_identity_max_abs_mm = 0.0_f64;
        let mut internal_surface_input_mm = 0.0_f64;
        let mut internal_surface_output_mm = 0.0_f64;
        let mut internal_lateral_input_mm = 0.0_f64;
        let mut internal_lateral_output_mm = 0.0_f64;

        for (index, record) in records.iter().enumerate() {
            let expected_ofe_id = index + 1;
            if record.ofe_id != expected_ofe_id {
                return Err(internal_wb13_failure(format!(
                    "internal per-OFE WB13 records must be ordered; expected OFE {expected_ofe_id}, observed {}",
                    record.ofe_id
                )));
            }
            if usize::from(record.row.wb13_row.ofe) != record.ofe_id {
                return Err(internal_wb13_failure(format!(
                    "internal WB13 row OFE {} does not match record OFE {}",
                    record.row.wb13_row.ofe, record.ofe_id
                )));
            }
            if !record.previous_storage_total_mm.is_finite() {
                return Err(internal_wb13_failure(format!(
                    "previous storage snapshot for OFE {} is non-finite ({})",
                    record.ofe_id, record.previous_storage_total_mm
                )));
            }

            let published_input_residual_mm = (record.row.wb13_row.upstrmq
                - record.upstream_transfer_input.upstrmq * 1_000.0)
                .abs()
                .max(
                    (record.row.wb13_row.subrin - record.upstream_transfer_input.subrin * 1_000.0)
                        .abs(),
                );
            if published_input_residual_mm > ME4_INTERNAL_WB13_IDENTITY_TOLERANCE_MM {
                return Err(internal_wb13_failure(format!(
                    "published upstream-input residual {published_input_residual_mm} mm exceeds tolerance {ME4_INTERNAL_WB13_IDENTITY_TOLERANCE_MM}"
                )));
            }

            let per_element_residual_mm = per_element_water_balance_residual_mm(record);
            per_element_identity_max_abs_mm =
                per_element_identity_max_abs_mm.max(per_element_residual_mm.abs());

            if index > 0 {
                internal_surface_input_mm += record.upstream_transfer_input.upstrmq * 1_000.0;
                internal_lateral_input_mm += record.upstream_transfer_input.subrin * 1_000.0;
            }
            if let Some(next_record) = records.get(index + 1) {
                let transfer_residual_mm =
                    adjacent_transfer_residual_mm(record, next_record)?;
                transfer_identity_max_abs_mm =
                    transfer_identity_max_abs_mm.max(transfer_residual_mm);

                let sent_surface_mm =
                    scaled_transfer_sum_mm(&record.current_transfer_output.surface_carry,
                        next_record.upstream_transfer_input.area_ratio);
                let sent_lateral_mm =
                    scaled_transfer_sum_mm(&record.current_transfer_output.lateral_carry,
                        next_record.upstream_transfer_input.area_ratio);
                internal_surface_output_mm += sent_surface_mm;
                internal_lateral_output_mm += sent_lateral_mm;
            }
        }

        let aggregate_transfer_cancellation_max_abs_mm =
            (internal_surface_input_mm - internal_surface_output_mm)
                .abs()
                .max((internal_lateral_input_mm - internal_lateral_output_mm).abs());

        let collection = Self {
            contributor_ofe_count,
            records,
            transfer_identity_max_abs_mm,
            per_element_identity_max_abs_mm,
            aggregate_transfer_cancellation_max_abs_mm,
        };
        collection.require_identity_closure()?;

        Ok(collection)
    }

    fn require_identity_closure(&self) -> Result<(), HillslopeCliError> {
        if self.transfer_identity_max_abs_mm > ME4_INTERNAL_WB13_IDENTITY_TOLERANCE_MM {
            return Err(internal_wb13_failure(format!(
                "transfer identity residual {} mm exceeds tolerance {ME4_INTERNAL_WB13_IDENTITY_TOLERANCE_MM}",
                self.transfer_identity_max_abs_mm
            )));
        }
        if self.per_element_identity_max_abs_mm > ME4_INTERNAL_WB13_IDENTITY_TOLERANCE_MM {
            return Err(internal_wb13_failure(format!(
                "per-element storage identity residual {} mm exceeds tolerance {ME4_INTERNAL_WB13_IDENTITY_TOLERANCE_MM}",
                self.per_element_identity_max_abs_mm
            )));
        }
        if self.aggregate_transfer_cancellation_max_abs_mm
            > ME4_INTERNAL_WB13_IDENTITY_TOLERANCE_MM
        {
            return Err(internal_wb13_failure(format!(
                "aggregate internal-transfer cancellation residual {} mm exceeds tolerance {ME4_INTERNAL_WB13_IDENTITY_TOLERANCE_MM}",
                self.aggregate_transfer_cancellation_max_abs_mm
            )));
        }

        Ok(())
    }
}

pub(super) fn internal_wb13_storage_total_mm_from_surface(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<f64, HillslopeCliError> {
    let soil_water_m = require_runtime_surface_scalar(runtime_surface, "wb11_soil_water")?;
    let frozen_water_m = require_runtime_surface_scalar(
        runtime_surface,
        "frost.runtime_frwatc_frozen_water_after_m",
    )?;
    if soil_water_m < 0.0 {
        return Err(internal_wb13_failure(format!(
            "wb11_soil_water must be >= 0.0 for previous storage snapshot, observed {soil_water_m}"
        )));
    }
    if frozen_water_m < 0.0 {
        return Err(internal_wb13_failure(format!(
            "frost.runtime_frwatc_frozen_water_after_m must be >= 0.0 for previous storage snapshot, observed {frozen_water_m}"
        )));
    }

    Ok((soil_water_m + frozen_water_m) * 1_000.0)
}

fn per_element_water_balance_residual_mm(record: &InternalPerOfeWb13Record) -> f64 {
    let row = &record.row.wb13_row;
    let current_storage_total_mm = row.total_soil + row.frozwt;
    let storage_delta_mm = current_storage_total_mm - record.previous_storage_total_mm;
    // WB13 `RM` is the local liquid-input publication term in this runner; it
    // already includes irrigation, while `Irr` is retained as a diagnostic row
    // column and must not be added a second time here.
    let local_liquid_input_mm = row.rm;
    let inflow_mm = local_liquid_input_mm + row.upstrmq + row.subrin;
    let outflow_mm = record.row.interception_mm
        + row.q
        + row.ep
        + row.es
        + row.er
        + row.dp
        + row.latqcc
        + row.tile;

    inflow_mm - outflow_mm - storage_delta_mm
}

fn adjacent_transfer_residual_mm(
    upstream_record: &InternalPerOfeWb13Record,
    downstream_record: &InternalPerOfeWb13Record,
) -> Result<f64, HillslopeCliError> {
    let expected_recipient = upstream_record.ofe_id + 1;
    if downstream_record.ofe_id != expected_recipient {
        return Err(internal_wb13_failure(format!(
            "adjacent transfer comparison expected downstream OFE {expected_recipient}, observed {}",
            downstream_record.ofe_id
        )));
    }
    if upstream_record.current_transfer_output.source_ofe_id != upstream_record.ofe_id {
        return Err(internal_wb13_failure(format!(
            "transfer output source OFE {} does not match upstream record OFE {}",
            upstream_record.current_transfer_output.source_ofe_id,
            upstream_record.ofe_id
        )));
    }
    if upstream_record.current_transfer_output.recipient_ofe_id != Some(downstream_record.ofe_id) {
        return Err(internal_wb13_failure(format!(
            "transfer output from OFE {} targets {:?}; expected Some({})",
            upstream_record.ofe_id,
            upstream_record.current_transfer_output.recipient_ofe_id,
            downstream_record.ofe_id
        )));
    }
    if downstream_record.upstream_transfer_input.source_ofe_id != Some(upstream_record.ofe_id) {
        return Err(internal_wb13_failure(format!(
            "downstream transfer input source {:?} does not match upstream OFE {}",
            downstream_record.upstream_transfer_input.source_ofe_id,
            upstream_record.ofe_id
        )));
    }
    if downstream_record.upstream_transfer_input.recipient_ofe_id != downstream_record.ofe_id {
        return Err(internal_wb13_failure(format!(
            "downstream transfer input recipient {} does not match record OFE {}",
            downstream_record.upstream_transfer_input.recipient_ofe_id,
            downstream_record.ofe_id
        )));
    }

    let surface_sent_mm = scaled_transfer_sum_mm(
        &upstream_record.current_transfer_output.surface_carry,
        downstream_record.upstream_transfer_input.area_ratio,
    );
    let lateral_sent_mm = scaled_transfer_sum_mm(
        &upstream_record.current_transfer_output.lateral_carry,
        downstream_record.upstream_transfer_input.area_ratio,
    );
    let surface_received_mm = downstream_record.upstream_transfer_input.upstrmq * 1_000.0;
    let lateral_received_mm = downstream_record.upstream_transfer_input.subrin * 1_000.0;

    Ok((surface_sent_mm - surface_received_mm)
        .abs()
        .max((lateral_sent_mm - lateral_received_mm).abs()))
}

fn scaled_transfer_sum_mm(values_m: &[f64], area_ratio: f64) -> f64 {
    values_m.iter().sum::<f64>() * area_ratio * 1_000.0
}

fn internal_wb13_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "per_ofe_internal_wb13",
        detail: format!("{SIMPIPE_GUARD_ID} {}", detail.into()),
    }
}
