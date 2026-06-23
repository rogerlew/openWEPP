const ME4_INTERNAL_WB13_IDENTITY_TOLERANCE_MM: f64 = 1.0e-11;
const MI_HILLSLOPE_TOTAL_IDENTITY_TOLERANCE_MM: f64 = 1.0e-9;

#[derive(Debug, Clone)]
pub(super) struct InternalPerOfeWb13Record {
    pub(super) ofe_id: usize,
    pub(super) area_m2: f64,
    pub(super) previous_storage_total_mm: f64,
    pub(super) physical_surface_outflow_mm: f64,
    pub(super) frost_upper_overflow_mm: f64,
    pub(super) frost_internal_adjustment_mm: f64,
    pub(super) storage_reconciliation_detail: String,
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
    per_element_identity_max_abs_detail: Option<String>,
    aggregate_transfer_cancellation_max_abs_mm: f64,
    hillslope_total_identity_max_abs_mm: f64,
    hillslope_total_identity_max_abs_detail: Option<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct PerOfeInternalWb13RunSummary {
    pub(super) day_count: usize,
    pub(super) record_count: usize,
    pub(super) expected_record_count: usize,
    pub(super) transfer_identity_max_abs_mm: f64,
    pub(super) per_element_identity_max_abs_mm: f64,
    pub(super) aggregate_transfer_cancellation_max_abs_mm: f64,
    pub(super) hillslope_total_identity_max_abs_mm: f64,
}

struct InternalIdentityScan {
    transfer_identity_max_abs_mm: f64,
    per_element_identity_max_abs_mm: f64,
    per_element_identity_max_abs_detail: Option<String>,
    internal_surface_input_mm: f64,
    internal_surface_output_mm: f64,
    internal_lateral_input_mm: f64,
    internal_lateral_output_mm: f64,
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
        self.hillslope_total_identity_max_abs_mm = self
            .hillslope_total_identity_max_abs_mm
            .max(collection.hillslope_total_identity_max_abs_mm);

        Ok(())
    }
}

impl DailyInternalPerOfeWb13Collection {
    pub(super) fn append_publication_rows_to(&self, rows: &mut Vec<SimulationOwnedWb13Row>) {
        rows.extend(self.records.iter().map(|record| record.row.clone()));
    }

    pub(super) fn append_runoff_delivery_rows_to(
        &self,
        wepp_id: i32,
        publication_area_m2: f64,
        publication_scalars: HillslopePassPublicationScalars,
        rows: &mut Vec<HillslopePassRow>,
    ) -> Result<(), HillslopeCliError> {
        if wepp_id <= 0 {
            return Err(internal_wb13_failure(format!(
                "outputs.pass_parquet wepp_id must be > 0, observed {wepp_id}"
            )));
        }
        if !publication_area_m2.is_finite() || publication_area_m2 <= 0.0 {
            return Err(internal_wb13_failure(format!(
                "outputs.pass_parquet publication area must be finite and > 0.0, observed {publication_area_m2}"
            )));
        }

        let outlet = self.records.last().ok_or_else(|| {
            internal_wb13_failure("outputs.pass_parquet has no outlet per-OFE record".to_string())
        })?;
        rows.push(build_hillslope_pass_row_from_outlet_delivery(
            wepp_id,
            outlet,
            publication_scalars,
        )?);
        Ok(())
    }

    pub(super) fn outlet_row(&self) -> Option<&SimulationOwnedWb13Row> {
        self.records.last().map(|record| &record.row)
    }

    fn from_sequence_report(
        sequence_report: &OfeLaneSequenceExecutionReport,
        lane_areas_m2: &[f64],
        runoff_publication_geometries: &[Wb13RunoffPublicationGeometry],
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
        if sequence_report.lane_reports.len() != runoff_publication_geometries.len() {
            return Err(internal_wb13_failure(format!(
                "persistent OFE sequence produced {} lane reports for {} runoff-publication geometry records",
                sequence_report.lane_reports.len(),
                runoff_publication_geometries.len()
            )));
        }

        let mut records = Vec::with_capacity(sequence_report.lane_reports.len());
        for (((lane_report, lane_area_m2), runoff_geometry), previous_storage_total_mm) in sequence_report
            .lane_reports
            .iter()
            .zip(lane_areas_m2.iter())
            .zip(runoff_publication_geometries.iter())
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
                    routed_runoff_m: Some(lane_report.current_transfer_output.qofe),
                    runoff_geometry: Some(*runoff_geometry),
                },
            )?;
            let frost_upper_overflow_m = runtime_surface_symbol_value(
                &lane_report.kernel_report.writeback_surface,
                "frost.runtime_watpdg_m",
            )
            .unwrap_or(0.0);
            if !frost_upper_overflow_m.is_finite() || frost_upper_overflow_m < 0.0 {
                return Err(internal_wb13_failure(format!(
                    "frost.runtime_watpdg_m must be finite and >= 0.0 for OFE {}, observed {}",
                    lane_report.ofe_id, frost_upper_overflow_m
                )));
            }
            let frost_internal_adjustment_m = internal_wb13_frost_internal_adjustment_m(
                &lane_report.kernel_report.writeback_surface,
            )?;
            records.push(InternalPerOfeWb13Record {
                ofe_id: lane_report.ofe_id,
                area_m2: *lane_area_m2,
                previous_storage_total_mm: *previous_storage_total_mm,
                physical_surface_outflow_mm: lane_report.current_transfer_output.qofe * 1_000.0,
                frost_upper_overflow_mm: frost_upper_overflow_m * 1_000.0,
                frost_internal_adjustment_mm: frost_internal_adjustment_m * 1_000.0,
                storage_reconciliation_detail: format_wb12_storage_terms(
                    &lane_report.kernel_report.writeback_surface,
                ),
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

        let identity_scan = scan_internal_identity_terms(&records)?;

        let aggregate_transfer_cancellation_max_abs_mm =
            (identity_scan.internal_surface_input_mm - identity_scan.internal_surface_output_mm)
                .abs()
                .max(
                    (identity_scan.internal_lateral_input_mm
                        - identity_scan.internal_lateral_output_mm)
                        .abs(),
                );
        let hillslope_total_identity = hillslope_total_identity_residual_mm(&records)?;
        let hillslope_total_identity_max_abs_mm = hillslope_total_identity.residual_mm.abs();

        let collection = Self {
            contributor_ofe_count,
            records,
            transfer_identity_max_abs_mm: identity_scan.transfer_identity_max_abs_mm,
            per_element_identity_max_abs_mm: identity_scan.per_element_identity_max_abs_mm,
            per_element_identity_max_abs_detail: identity_scan.per_element_identity_max_abs_detail,
            aggregate_transfer_cancellation_max_abs_mm,
            hillslope_total_identity_max_abs_mm,
            hillslope_total_identity_max_abs_detail: Some(hillslope_total_identity.detail),
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
            let detail = self
                .per_element_identity_max_abs_detail
                .as_deref()
                .unwrap_or("no per-element residual detail recorded");
            return Err(internal_wb13_failure(format!(
                "per-element storage identity residual {} mm exceeds tolerance {ME4_INTERNAL_WB13_IDENTITY_TOLERANCE_MM}; {detail}",
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
        if self.hillslope_total_identity_max_abs_mm > MI_HILLSLOPE_TOTAL_IDENTITY_TOLERANCE_MM {
            let detail = self
                .hillslope_total_identity_max_abs_detail
                .as_deref()
                .unwrap_or("no hillslope-total residual detail recorded");
            return Err(internal_wb13_failure(format!(
                "hillslope-total identity residual {} mm exceeds tolerance {MI_HILLSLOPE_TOTAL_IDENTITY_TOLERANCE_MM}; {detail}",
                self.hillslope_total_identity_max_abs_mm
            )));
        }

        Ok(())
    }
}

fn scan_internal_identity_terms(
    records: &[InternalPerOfeWb13Record],
) -> Result<InternalIdentityScan, HillslopeCliError> {
    let mut scan = InternalIdentityScan {
        transfer_identity_max_abs_mm: 0.0,
        per_element_identity_max_abs_mm: 0.0,
        per_element_identity_max_abs_detail: None,
        internal_surface_input_mm: 0.0,
        internal_surface_output_mm: 0.0,
        internal_lateral_input_mm: 0.0,
        internal_lateral_output_mm: 0.0,
    };

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
        if !record.area_m2.is_finite() || record.area_m2 <= 0.0 {
            return Err(internal_wb13_failure(format!(
                "OFE {} area_m2 must be finite and > 0.0 for hillslope-total identity, observed {}",
                record.ofe_id, record.area_m2
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

        let per_element_terms = per_element_water_balance_terms(record);
        if per_element_terms.residual_mm.abs() > scan.per_element_identity_max_abs_mm {
            scan.per_element_identity_max_abs_mm = per_element_terms.residual_mm.abs();
            scan.per_element_identity_max_abs_detail = Some(format!(
                "{}; wb12_terms={}",
                per_element_terms.describe(record.ofe_id),
                record.storage_reconciliation_detail
            ));
        }

        if index > 0 {
            scan.internal_surface_input_mm += record.upstream_transfer_input.upstrmq * 1_000.0;
            scan.internal_lateral_input_mm += record.upstream_transfer_input.subrin * 1_000.0;
        }
        if let Some(next_record) = records.get(index + 1) {
            let transfer_residual_mm = adjacent_transfer_residual_mm(record, next_record)?;
            scan.transfer_identity_max_abs_mm = scan
                .transfer_identity_max_abs_mm
                .max(transfer_residual_mm);

            scan.internal_surface_output_mm += scaled_transfer_sum_mm(
                &record.current_transfer_output.surface_carry,
                next_record.upstream_transfer_input.area_ratio,
            );
            scan.internal_lateral_output_mm += scaled_transfer_sum_mm(
                &record.current_transfer_output.lateral_carry,
                next_record.upstream_transfer_input.area_ratio,
            );
        }
    }

    Ok(scan)
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

fn internal_wb13_frost_internal_adjustment_m(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<f64, HillslopeCliError> {
    let net_liquid_delta =
        runtime_surface_symbol_value(runtime_surface, "frost.runtime_frwatc_net_liquid_delta_m")
            .unwrap_or(0.0);
    let frozen_before =
        runtime_surface_symbol_value(runtime_surface, "frost.runtime_frwatc_frozen_water_before_m")
            .unwrap_or(0.0);
    let frozen_after =
        runtime_surface_symbol_value(runtime_surface, "frost.runtime_frwatc_frozen_water_after_m")
            .unwrap_or(0.0);
    let watpdg = runtime_surface_symbol_value(runtime_surface, "frost.runtime_watpdg_m")
        .unwrap_or(0.0);
    let watbtm = runtime_surface_symbol_value(runtime_surface, "frost.runtime_watbtm_m")
        .unwrap_or(0.0);

    for (symbol, value) in [
        (
            "frost.runtime_frwatc_net_liquid_delta_m",
            net_liquid_delta,
        ),
        ("frost.runtime_frwatc_frozen_water_before_m", frozen_before),
        ("frost.runtime_frwatc_frozen_water_after_m", frozen_after),
        ("frost.runtime_watpdg_m", watpdg),
        ("frost.runtime_watbtm_m", watbtm),
    ] {
        if !value.is_finite() {
            return Err(internal_wb13_failure(format!(
                "{symbol} must be finite for internal WB13 frost adjustment, observed {value}"
            )));
        }
    }
    if frozen_before < 0.0 || frozen_after < 0.0 || watpdg < 0.0 || watbtm < 0.0 {
        return Err(internal_wb13_failure(format!(
            "frost storage and overflow diagnostics must be non-negative; frozen_before={frozen_before}, frozen_after={frozen_after}, watpdg={watpdg}, watbtm={watbtm}"
        )));
    }

    // `watbtm` (lower overflow) is intentionally excluded from this inflow-side
    // adjustment: it is owned by the WB13 `Dp` deep-percolation outflow lineage
    // (SC-WATBAL-001 M-E4-REDO item 2; SC-SNOWFREEZE-001 "enters WB13 `Dp`").
    // Adding it here double-counts it against `Dp` and reopens the per-element
    // residual by exactly `watbtm` on frost-overflow days (FARPOINT01 F-B, H2637
    // OFE5 1996-02-06). It remains read and bounds-checked above as a valid term.
    Ok(net_liquid_delta + frozen_after - frozen_before + watpdg)
}

#[derive(Debug, Clone, Copy)]
#[allow(clippy::struct_field_names)]
struct PerElementWaterBalanceTerms {
    residual_mm: f64,
    local_liquid_input_mm: f64,
    inflow_mm: f64,
    interception_mm: f64,
    physical_q_mm: f64,
    published_q_mm: f64,
    published_qofe_mm: f64,
    ep_mm: f64,
    es_mm: f64,
    er_mm: f64,
    dp_mm: f64,
    latqcc_mm: f64,
    tile_mm: f64,
    frost_upper_overflow_mm: f64,
    frost_internal_adjustment_mm: f64,
    outflow_mm: f64,
    previous_storage_total_mm: f64,
    current_storage_total_mm: f64,
    storage_delta_mm: f64,
}

impl PerElementWaterBalanceTerms {
    fn describe(self, ofe_id: usize) -> String {
        format!(
            "ofe={ofe_id}, residual_mm={}, inflow_mm={} (RM/local={} UpStrmQ+SubRIn={} frost_adjustment={}), outflow_mm={} (I={} physical_Q={} published_Q={} published_QOFE={} Ep={} Es={} Er={} Dp={} latqcc={} Tile={} watpdg={}), storage_delta_mm={} (previous={} current={})",
            self.residual_mm,
            self.inflow_mm,
            self.local_liquid_input_mm,
            self.inflow_mm - self.local_liquid_input_mm - self.frost_internal_adjustment_mm,
            self.frost_internal_adjustment_mm,
            self.outflow_mm,
            self.interception_mm,
            self.physical_q_mm,
            self.published_q_mm,
            self.published_qofe_mm,
            self.ep_mm,
            self.es_mm,
            self.er_mm,
            self.dp_mm,
            self.latqcc_mm,
            self.tile_mm,
            self.frost_upper_overflow_mm,
            self.storage_delta_mm,
            self.previous_storage_total_mm,
            self.current_storage_total_mm,
        )
    }
}

fn per_element_water_balance_terms(
    record: &InternalPerOfeWb13Record,
) -> PerElementWaterBalanceTerms {
    let row = &record.row.wb13_row;
    let current_storage_total_mm = row.total_soil + row.frozwt;
    let storage_delta_mm = current_storage_total_mm - record.previous_storage_total_mm;
    // WB13 `RM` is the local liquid-input publication term in this runner; it
    // already includes irrigation, while `Irr` is retained as a diagnostic row
    // column and must not be added a second time here.
    let local_liquid_input_mm = row.rm;
    let inflow_mm = local_liquid_input_mm
        + row.upstrmq
        + row.subrin
        + record.frost_internal_adjustment_mm;
    let interception_mm = record.row.interception_mm;
    let outflow_mm = interception_mm
        + record.physical_surface_outflow_mm
        + row.ep
        + row.es
        + row.er
        + row.dp
        + row.latqcc
        + row.tile
        + record.frost_upper_overflow_mm;

    PerElementWaterBalanceTerms {
        residual_mm: inflow_mm - outflow_mm - storage_delta_mm,
        local_liquid_input_mm,
        inflow_mm,
        interception_mm,
        physical_q_mm: record.physical_surface_outflow_mm,
        published_q_mm: row.q,
        published_qofe_mm: row.qofe,
        ep_mm: row.ep,
        es_mm: row.es,
        er_mm: row.er,
        dp_mm: row.dp,
        latqcc_mm: row.latqcc,
        tile_mm: row.tile,
        frost_upper_overflow_mm: record.frost_upper_overflow_mm,
        frost_internal_adjustment_mm: record.frost_internal_adjustment_mm,
        outflow_mm,
        previous_storage_total_mm: record.previous_storage_total_mm,
        current_storage_total_mm,
        storage_delta_mm,
    }
}

struct HillslopeTotalIdentityResidual {
    residual_mm: f64,
    detail: String,
}

fn hillslope_total_identity_residual_mm(
    records: &[InternalPerOfeWb13Record],
) -> Result<HillslopeTotalIdentityResidual, HillslopeCliError> {
    if records.is_empty() {
        return Err(internal_wb13_failure(
            "hillslope-total identity requires at least one internal WB13 record",
        ));
    }

    let outlet_index = records.len() - 1;
    let mut weighted_residual_mm_m2 = 0.0_f64;
    let mut total_area_m2 = 0.0_f64;
    let mut detail_parts = Vec::with_capacity(records.len());

    for (index, record) in records.iter().enumerate() {
        if !record.area_m2.is_finite() || record.area_m2 <= 0.0 {
            return Err(internal_wb13_failure(format!(
                "OFE {} area_m2 must be finite and > 0.0 for hillslope-total identity, observed {}",
                record.ofe_id, record.area_m2
            )));
        }

        let terms = per_element_water_balance_terms(record);
        let external_in_mm =
            terms.local_liquid_input_mm + terms.frost_internal_adjustment_mm;
        let mut external_out_mm = terms.interception_mm
            + terms.ep_mm
            + terms.es_mm
            + terms.er_mm
            + terms.dp_mm
            + terms.tile_mm
            + terms.frost_upper_overflow_mm;
        if index == outlet_index {
            external_out_mm += terms.physical_q_mm + terms.latqcc_mm;
        }

        let local_residual_mm = external_in_mm - external_out_mm - terms.storage_delta_mm;
        let weighted_residual = local_residual_mm * record.area_m2;
        weighted_residual_mm_m2 += weighted_residual;
        total_area_m2 += record.area_m2;
        detail_parts.push(format!(
            "ofe={} area_m2={} local_residual_mm={} weighted_mm_m2={} external_in_mm={} external_out_mm={} storage_delta_mm={} physical_q_mm={} latqcc_mm={} upstream_in_mm={} transfer_surface_out_mm={} transfer_lateral_out_mm={}",
            record.ofe_id,
            record.area_m2,
            local_residual_mm,
            weighted_residual,
            external_in_mm,
            external_out_mm,
            terms.storage_delta_mm,
            terms.physical_q_mm,
            terms.latqcc_mm,
            record.upstream_transfer_input.upstrmq * 1_000.0,
            scaled_transfer_sum_mm(&record.current_transfer_output.surface_carry, 1.0),
            scaled_transfer_sum_mm(&record.current_transfer_output.lateral_carry, 1.0),
        ));
    }

    if !total_area_m2.is_finite() || total_area_m2 <= 0.0 {
        return Err(internal_wb13_failure(format!(
            "total hillslope area must be finite and > 0.0 for hillslope-total identity, observed {total_area_m2}"
        )));
    }

    let residual_mm = weighted_residual_mm_m2 / total_area_m2;
    Ok(HillslopeTotalIdentityResidual {
        residual_mm,
        detail: format!(
            "weighted_residual_mm_m2={weighted_residual_mm_m2}, total_area_m2={total_area_m2}, {}",
            detail_parts.join("; ")
        ),
    })
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

#[cfg(test)]
mod farpoint01_frost_overflow_double_count_tests {
    use super::*;

    fn seed_frost_overflow_surface(
        net_liquid_delta_m: f64,
        frozen_before_m: f64,
        frozen_after_m: f64,
        watpdg_m: f64,
        watbtm_m: f64,
    ) -> HillslopeWritebackSurface {
        let mut surface = HillslopeWritebackSurface::default();
        for (symbol, value) in [
            ("frost.runtime_frwatc_net_liquid_delta_m", net_liquid_delta_m),
            ("frost.runtime_frwatc_frozen_water_before_m", frozen_before_m),
            ("frost.runtime_frwatc_frozen_water_after_m", frozen_after_m),
            ("frost.runtime_watpdg_m", watpdg_m),
            ("frost.runtime_watbtm_m", watbtm_m),
        ] {
            surface
                .state_surface
                .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
        }
        surface
    }

    /// FARPOINT01 F-B regression: the named internal frost adjustment must
    /// **exclude** the lower-overflow `watbtm` term. `watbtm` is owned by the
    /// WB13 `Dp` deep-percolation outflow lineage (`SC-SNOWFREEZE-001`); counting
    /// it on the per-element inflow side double-counts it against `Dp` and
    /// reopens the per-element/hillslope-total residual by exactly `watbtm` on
    /// frost-overflow days (reproduced on the H2637 19-OFE substrate, OFE5,
    /// 1996-02-06). Fails before the line-432 fix (returns the double-counted
    /// value); passes after.
    #[test]
    fn farpoint01_internal_frost_adjustment_excludes_watbtm_lower_overflow() {
        // Frost-overflow day terms (H2637 OFE5, 1996-02-06), in metres.
        let net_liquid_delta_m = 0.008_987_645_4;
        let frozen_before_m = 0.005_292_571_9;
        let frozen_after_m = 0.004_980_476_6;
        let watpdg_m = 0.0;
        let watbtm_m = 0.008_231_171_5; // > 0: the frost lower-overflow signature

        let surface = seed_frost_overflow_surface(
            net_liquid_delta_m,
            frozen_before_m,
            frozen_after_m,
            watpdg_m,
            watbtm_m,
        );

        let adjustment_m = internal_wb13_frost_internal_adjustment_m(&surface)
            .expect("frost adjustment must compute for a valid overflow surface");

        let expected_m = net_liquid_delta_m + frozen_after_m - frozen_before_m + watpdg_m;
        assert!(
            (adjustment_m - expected_m).abs() <= 1.0e-15,
            "adjustment must be net_liquid_delta + frozen_after - frozen_before + watpdg \
             (watbtm excluded); expected {expected_m}, observed {adjustment_m}"
        );

        // Anti-tautology: the pre-fix formula (including watbtm) is a strictly
        // different value; the corrected adjustment must not equal it.
        let double_counted_m = expected_m + watbtm_m;
        assert!(
            (adjustment_m - double_counted_m).abs() > 1.0e-9,
            "adjustment must not include watbtm; watbtm is owned by the Dp outflow lineage"
        );
    }
}
