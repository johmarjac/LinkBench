//! Project files (*.linkbench): all sessions and settings in one JSON file
//! that can be passed on to colleagues, plus the Windows file association.
//! Files from the time the app was called PortWerk (*.portwerk) still open.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::app::{APP_NAME, Saved};
use crate::i18n::trf;

pub const EXT: &str = "linkbench";
/// The extension project files had before the rename.
pub const OLD_EXT: &str = "portwerk";
const FORMAT: &str = "linkbench-project";
const OLD_FORMAT: &str = "portwerk-projekt";
/// The id under which Windows knows the file type.
#[cfg(windows)]
const PROG_ID: &str = "LinkBench.Project";

#[derive(Serialize, Deserialize)]
struct ProjectFile {
    format: String,
    version: u32,
    app_version: String,
    project: Saved,
}

pub fn save(path: &Path, project: &Saved) -> Result<(), String> {
    let f = ProjectFile {
        format: FORMAT.into(),
        version: 1,
        app_version: env!("CARGO_PKG_VERSION").into(),
        project: project.clone(),
    };
    let text = serde_json::to_string_pretty(&f).map_err(|e| e.to_string())?;
    std::fs::write(path, text).map_err(|e| trf("The project could not be saved: {}", &[&e]))
}

pub fn load(path: &Path) -> Result<Saved, String> {
    let text = std::fs::read_to_string(path).map_err(|e| trf("The project could not be read: {}", &[&e]))?;
    let f: ProjectFile =
        serde_json::from_str(&text).map_err(|e| trf("Not a valid {} project file: {}", &[&APP_NAME, &e]))?;
    if f.format != FORMAT && f.format != OLD_FORMAT {
        return Err(trf("Not a {} project file", &[&APP_NAME]));
    }
    Ok(f.project)
}

pub fn display_name(path: &Path) -> String {
    path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| path.display().to_string())
}

/// Registers *.linkbench (and the old *.portwerk) for the current user so a double click opens
/// the app. (The MSI installer does the same for all users.)
#[cfg(windows)]
pub fn register_extension() -> Result<String, String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe = exe.display().to_string();
    let base = r"HKCU\Software\Classes";
    let entries: [(String, String); 5] = [
        (format!(r"{base}\.{EXT}"), PROG_ID.into()),
        (format!(r"{base}\.{OLD_EXT}"), PROG_ID.into()),
        (format!(r"{base}\{PROG_ID}"), trf("{} project", &[&APP_NAME])),
        (format!(r"{base}\{PROG_ID}\DefaultIcon"), format!("\"{exe}\",0")),
        (format!(r"{base}\{PROG_ID}\shell\open\command"), format!("\"{exe}\" \"%1\"")),
    ];
    for (key, value) in entries {
        let out = std::process::Command::new("reg")
            .args(["add", &key, "/ve", "/d", &value, "/f"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .map_err(|e| trf("The registry could not be changed: {}", &[&e]))?;
        if !out.status.success() {
            return Err(trf(
                "The registry could not be changed: {}",
                &[&String::from_utf8_lossy(&out.stderr).trim()],
            ));
        }
    }
    Ok(trf(".{} files now open with {}", &[&EXT, &exe]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_roundtrip() {
        let mut p = Saved::default();
        p.tcpc_sessions[0].host = "10.0.0.7".into();
        p.modbus_sessions[0].unit = 17;
        let path = std::env::temp_dir().join("linkbench_test.linkbench");
        save(&path, &p).unwrap();
        let q = load(&path).unwrap();
        assert_eq!(q.tcpc_sessions[0].host, "10.0.0.7");
        assert_eq!(q.modbus_sessions[0].unit, 17);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn old_project_files_still_open() {
        let p = Saved::default();
        let f = ProjectFile { format: OLD_FORMAT.into(), version: 1, app_version: "0.1.0".into(), project: p };
        let path = std::env::temp_dir().join("linkbench_test_old.portwerk");
        std::fs::write(&path, serde_json::to_string(&f).unwrap()).unwrap();
        assert!(load(&path).is_ok());
        let _ = std::fs::remove_file(path);
    }
}
