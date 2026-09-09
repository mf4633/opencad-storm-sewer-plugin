//! `SS_LICENSE` / `SS_ACTIVATE` command text and the Pro gate message.
//!
//! Pure functions returning lines for the OCS command line, so they can be
//! unit-tested without a host.

use crate::license;

/// Commands that require an active Pro license. Everything else stays free.
pub const PRO_COMMANDS: &[&str] = &["SS_REPORT_HTML", "SS_SIZE", "SS_MULTIRP"];

pub fn is_pro_command(cmd: &str) -> bool {
    PRO_COMMANDS.contains(&cmd)
}

/// Lines pushed when a Pro command is invoked without a license.
pub fn pro_required_lines(cmd: &str) -> Vec<String> {
    let free_alt = match cmd {
        "SS_REPORT_HTML" => "  Free alternative: SS_REPORT prints the same Rational + Manning + HGL table to the command line.",
        "SS_SIZE" => "  Free alternative: SS_ANALYZE flags surcharged pipes; adjust diameters with SS_EDIT.",
        "SS_MULTIRP" => "  Free alternative: change the return period with SS_PARAMS and rerun SS_ANALYZE.",
        _ => "  Free alternative: SS_ANALYZE / SS_REPORT / SS_PROFILE.",
    };
    vec![
        format!("--- Storm Sewer: {cmd} is a Pro feature ($29/year) ---"),
        format!(
            "  Buy a key at {}  then run  SS_ACTIVATE <email> <hc_live_ss_...>",
            license::PURCHASE_URL
        ),
        free_alt.to_string(),
        format!(
            "  Or free: the standalone StormSewer desktop app does this at no cost — {}",
            license::FREE_DESKTOP_URL
        ),
    ]
}

pub fn license_lines() -> Vec<String> {
    vec![
        "=== Storm Sewer License ===".into(),
        format!("  Product: {} ({})", license::PRODUCT_LABEL, license::PRODUCT_ID),
        format!("  Status: {}", license::status_label()),
        format!("  Validation mode: {}", license::validation_mode_label()),
        format!("  Last validated: {}", license::last_validated_label()),
        format!("  Network: {}", license::online_offline_label()),
        format!("  License file: {}", license::license_file_path().display()),
        format!(
            "  Activate: SS_ACTIVATE <email> <token>  |  {}",
            license::PURCHASE_URL
        ),
        format!(
            "  Pro unlocks {}. Drawing, SS_ANALYZE, SS_REPORT, SS_PROFILE and SS_VALIDATE stay free.",
            PRO_COMMANDS.join(", ")
        ),
        "  HydroComplete (opencad) and Civil 3D keys are separate SKUs and will not activate this plugin.".into(),
    ]
}

pub fn activate_lines(args: &str) -> Vec<String> {
    let trimmed = args.trim();
    if trimmed.is_empty() {
        return vec![
            format!(
                "=== Storm Sewer Pro Activation ({}) ===",
                license::PRODUCT_LABEL
            ),
            "  Usage: SS_ACTIVATE <email> <hc_live_token>".into(),
            "  Or paste both on one line: email@domain.com hc_live_ss_...".into(),
            format!(
                "  Buy a Storm Sewer Pro key ($29/year) at {}",
                license::PURCHASE_URL
            ),
        ];
    }
    let (email, token) = if let Some((e, t)) = license::try_parse_combined_input(trimmed) {
        (e, t)
    } else {
        let mut parts = trimmed.split_whitespace();
        let email = parts.next().unwrap_or("").to_string();
        let token = parts.next().unwrap_or("").to_string();
        if email.is_empty() || token.is_empty() {
            return vec![
                "Activation failed: provide email and token.".into(),
                "  Usage: SS_ACTIVATE user@example.com hc_live_ss_...".into(),
            ];
        }
        (email, token)
    };
    let activator = license::LicenseActivator::new();
    let path = license::license_file_path();
    let result = activator.activate(&email, &token, &path);
    let mut lines = vec!["=== Storm Sewer Pro Activation ===".into()];
    if result.success {
        lines.push(format!("  {}", result.message));
        lines.push(format!("  Mode: {:?}", result.mode));
        if let Some(d) = result.expires.get(..10) {
            lines.push(format!("  Expires: {d}"));
        }
        lines.push(format!("  Status: {}", license::status_label()));
        lines.push(format!("  Unlocked: {}", PRO_COMMANDS.join(", ")));
    } else {
        lines.push(format!("  Activation failed: {}", result.message));
        lines.push(format!("  Status: {}", license::status_label()));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pro_commands_are_the_deliverables_only() {
        assert!(is_pro_command("SS_REPORT_HTML"));
        assert!(is_pro_command("SS_SIZE"));
        assert!(is_pro_command("SS_MULTIRP"));
        for free in [
            "SS_ANALYZE",
            "SS_REPORT",
            "SS_PROFILE",
            "SS_VALIDATE",
            "SS_PIPE",
            "SS_IMPORTXML",
        ] {
            assert!(!is_pro_command(free), "{free} must stay free");
        }
    }

    #[test]
    fn gate_message_names_command_and_purchase_url() {
        let lines = pro_required_lines("SS_SIZE");
        assert!(lines[0].contains("SS_SIZE"));
        assert!(lines[1].contains(license::PURCHASE_URL));
        assert!(lines[1].contains("SS_ACTIVATE"));
        assert!(lines[2].contains("Free alternative"));
    }

    #[test]
    fn activate_without_args_prints_usage() {
        let lines = activate_lines("");
        assert!(lines.iter().any(|l| l.contains("Usage: SS_ACTIVATE")));
    }

    #[test]
    fn activate_with_one_token_fails_cleanly() {
        let lines = activate_lines("onlyanemail@example.com");
        assert!(lines[0].starts_with("Activation failed"));
    }

    #[test]
    fn license_lines_mention_product_and_free_tier() {
        let lines = license_lines();
        assert!(lines[1].contains("stormsewer"));
        assert!(lines.iter().any(|l| l.contains("stay free")));
    }
}

#[cfg(test)]
mod disclosure_tests {
    use super::*;

    /// The Pro gate must name the free desktop app that does the same job.
    /// The plugin's three Pro features are all free in the standalone
    /// StormSewer app; a buyer who finds that out only after paying has been
    /// treated badly, so the gate says it up front.
    #[test]
    fn pro_gate_names_the_free_desktop_alternative() {
        for cmd in PRO_COMMANDS {
            let text = pro_required_lines(cmd).join("\n");
            assert!(
                text.contains(license::FREE_DESKTOP_URL),
                "{cmd} gate does not point at the free desktop app: {text}"
            );
            assert!(
                text.contains(license::PURCHASE_URL),
                "{cmd} gate does not say where to buy: {text}"
            );
            assert!(
                text.contains("Free alternative"),
                "{cmd} gate does not name the free in-plugin command: {text}"
            );
        }
    }
}
