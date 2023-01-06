// do not run on Windows
#![cfg(not(windows))]

use ethers_contract_abigen::{Abigen, MultiAbigen};
use eyre::Result;
use std::{env, path::PathBuf, process::Command};

const ABI_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/abi");
const BINDINGS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/contracts/bindings");
const ABIGEN_CHECK: &str = "ABIGEN_CHECK";

// Don't print anything if we're not in "check" mode
macro_rules! warn {
    ($s:expr) => {
        println!(concat!("cargo:warning=uniswap-rs/build.rs: ", $s))
    };
    ($s:expr, $($args:tt)*) => {
        println!(concat!("cargo:warning=uniswap-rs/build.rs: ", $s), $($args)+)
    };
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed={ABI_PATH}");
    println!("cargo:rerun-if-env-changed={ABIGEN_CHECK}");
    match env::var_os(ABIGEN_CHECK) {
        Some(x) if !x.is_empty() && (x == "1" || x == "true") => abigen(true),
        _ => {
            // ignore fails, eg for docs.rs builds
            abigen(false).unwrap_or_else(|e| warn!("Failed to build bindings: {}", e));
            Ok(())
        }
    }
}

fn abigen(check: bool) -> Result<()> {
    // current rustfmt.toml contains nightly-only config
    let rustfmt = match rustfmt_version() {
        Ok(rustfmt) => rustfmt.contains("nightly"),
        Err(e) => {
            if check {
                warn!("Could not get rustfmt version: {}", e)
            }
            false
        }
    };
    if !rustfmt {
        if check {
            warn!("Cannot build bindings without rustfmt nightly.");
        }
        return Ok(())
    }

    // temp: manually build abigens to set rustfmt value
    let abigens = json_files(ABI_PATH)
        .map(|path| Abigen::from_file(path).map(|abigen| abigen.rustfmt(rustfmt)))
        .collect::<Result<Vec<_>>>()?;

    let multi_abigen = MultiAbigen::from_abigens(abigens);
    let bindings = multi_abigen.build()?;
    if check {
        bindings.ensure_consistent_module(BINDINGS_PATH, false)?;
    } else {
        bindings.write_to_module(BINDINGS_PATH, false)?;
    }
    Ok(())
}

fn rustfmt_version() -> Result<String> {
    let rustfmt = env::var_os("RUSTFMT").unwrap_or_else(|| "rustfmt".into());
    let output = Command::new(rustfmt).arg("--version").output()?;
    let version = String::from_utf8(output.stdout)?;
    Ok(version)
}

/// Returns a list of absolute paths to all the json files under the root.
///
/// Modified from: ethers_contract_abigen::util::json_files
pub fn json_files(root: impl AsRef<std::path::Path>) -> impl Iterator<Item = PathBuf> {
    walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map(|ext| ext == "json").unwrap_or_default())
        .map(|e| e.path().into())
}
