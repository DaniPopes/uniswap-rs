use ethers_contract_abigen::MultiAbigen;

const ABI_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/abi");
const BINDINGS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/contracts/bindings");
const ABIGEN_CHECK: &str = "ABIGEN_CHECK";

fn main() {
    match std::env::var(ABIGEN_CHECK) {
        Ok(x) if !x.is_empty() => {
            abigen(true).unwrap();
        }
        _ => {
            // ignore fails, eg for docs.rs builds
            abigen(false).unwrap_or_else(|e| eprintln!("Failed to abigen: {e}"));
        }
    }
}

fn abigen(check: bool) -> eyre::Result<()> {
    let gen = MultiAbigen::from_json_files(ABI_PATH)?;
    let bindings = gen.build()?;
    if check {
        bindings.ensure_consistent_module(BINDINGS_PATH, false)?;
    } else {
        bindings.write_to_module(BINDINGS_PATH, false)?;
    }
    println!("cargo:rerun-if-changed={ABI_PATH}");
    println!("cargo:rerun-if-env-changed={ABIGEN_CHECK}");
    Ok(())
}
