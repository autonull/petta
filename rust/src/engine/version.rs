//! SWI-Prolog version checking

use std::path::Path;
use std::process::Command;

/// Minimum SWI-Prolog version
pub const MIN_SWIPL_VERSION: (u32, u32) = (9, 3);

pub fn check_swipl_version(path: &Path, min: (u32, u32)) -> Result<(), super::PeTTaError> {
    let output = Command::new(path).arg("--version").output().map_err(|_| {
        super::PeTTaError::SwiplVersion(format!("swipl not found at {}", path.display()))
    })?;
    
    if !output.status.success() {
        return Err(super::PeTTaError::SwiplVersion("version check failed".into()));
    }
    
    let vs = String::from_utf8_lossy(&output.stdout);
    for part in vs.split_whitespace() {
        let p: Vec<&str> = part.split('.').collect();
        if p.len() >= 2 {
            if let (Ok(ma), Ok(mi)) = (p[0].parse::<u32>(), p[1].parse::<u32>()) {
                if ma > min.0 || (ma == min.0 && mi >= min.1) {
                    return Ok(());
                }
                return Err(super::PeTTaError::SwiplVersion(
                    format!("need {}.{} found {}.{}", min.0, min.1, ma, mi)
                ));
            }
        }
    }
    Ok(())
}

pub fn swipl_available(path: &Path) -> bool {
    check_swipl_version(path, MIN_SWIPL_VERSION).is_ok()
}
