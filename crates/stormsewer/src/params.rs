// SPDX-License-Identifier: GPL-3.0-only

//! Global storm-sewer analysis parameters (hydrology, hydraulics, sizing).

use crate::design::DesignCriteria;
use crate::hydrology::IdfSet;
use crate::network::AnalysisOptions;

/// Network-level parameters for analyze / size / report passes.
#[derive(Clone, Debug, PartialEq)]
pub struct StormAnalysisParams {
    pub idf: IdfSet,
    pub hydraulics: AnalysisOptions,
    pub sizing: DesignCriteria,
    /// Default grate length (ft) for HEC-22 inlet capacity checks at inlets.
    pub inlet_grate_length_ft: f64,
    /// Assumed gutter flow depth (ft) at the curb for inlet checks.
    pub inlet_flow_depth_ft: f64,
    /// Assumed gutter longitudinal slope (ft/ft) for inlet checks.
    pub inlet_gutter_slope: f64,
}

impl Default for StormAnalysisParams {
    fn default() -> Self {
        Self {
            idf: IdfSet::municipal_default(),
            hydraulics: AnalysisOptions::default(),
            sizing: DesignCriteria::municipal(),
            inlet_grate_length_ft: 2.0,
            inlet_flow_depth_ft: 0.15,
            inlet_gutter_slope: 0.005,
        }
    }
}

impl StormAnalysisParams {
    pub fn municipal() -> Self {
        Self::default()
    }

    /// Summary for command-line / dialog display.
    pub fn summary(&self) -> String {
        let c = self.idf.design_curve();
        let tw = self
            .hydraulics
            .tailwater
            .map(|t| format!("{t:.2} ft"))
            .unwrap_or_else(|| "free".into());
        format!(
            "RP {}yr  IDF i=a/(t+b)^c  a={:.1} b={:.1} c={:.2}  tailwater={tw}  minTc={:.0}min  junctionK={:.2}  V={:.1}-{:.1} ft/s  maxFull={:.0}%",
            self.idf.design_rp,
            c.a,
            c.b,
            c.c,
            self.hydraulics.min_tc,
            self.hydraulics.junction_k,
            self.sizing.min_velocity,
            self.sizing.max_velocity,
            self.sizing.max_pct_full * 100.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_includes_design_rp() {
        let p = StormAnalysisParams::default();
        assert!(p.summary().contains("RP 10yr"));
    }
}