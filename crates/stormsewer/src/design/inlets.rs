// SPDX-License-Identifier: GPL-3.0-only

//! Inlet interception capacity (HEC-22 style grate on grade, simplified).

/// Grate-on-grade interception capacity (cfs).
///
/// Uses the HEC-22 composite gutter approximation for a depressed grate:
/// `Q = Cw * L * d^1.5 * sqrt(S)` with `Cw ≈ 3.0` (US customary calibration).
pub fn grate_capacity_cfs(grate_length_ft: f64, flow_depth_ft: f64, gutter_slope: f64) -> f64 {
    if grate_length_ft <= 0.0 || flow_depth_ft <= 0.0 || gutter_slope <= 0.0 {
        return 0.0;
    }
    const CW: f64 = 3.0;
    CW * grate_length_ft * flow_depth_ft.powf(1.5) * gutter_slope.sqrt()
}

/// Check whether an inlet can capture the approach design flow.
#[derive(Clone, Debug, PartialEq)]
pub struct InletCheck {
    pub design_q_cfs: f64,
    pub capacity_cfs: f64,
    pub ok: bool,
}

pub fn check_inlet(design_q_cfs: f64, grate_length_ft: f64, flow_depth_ft: f64, gutter_slope: f64) -> InletCheck {
    let cap = grate_capacity_cfs(grate_length_ft, flow_depth_ft, gutter_slope);
    InletCheck {
        design_q_cfs,
        capacity_cfs: cap,
        ok: cap >= design_q_cfs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longer_grate_carries_more() {
        let a = grate_capacity_cfs(2.0, 0.15, 0.005);
        let b = grate_capacity_cfs(5.0, 0.15, 0.005);
        assert!(b > a);
    }
}