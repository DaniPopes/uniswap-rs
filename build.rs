use ethers_contract_abigen::MultiAbigen;

const ABI_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/abi");
const BINDINGS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/contracts/bindings");

fn main() {
    // ignore fails, eg for docs.rs builds
    abigen().unwrap_or_else(|e| eprintln!("Failed to abigen: {e}"));
}

fn abigen() -> eyre::Result<()> {
    let gen = MultiAbigen::from_json_files(ABI_PATH)?;
    let bindings = gen.build()?;
    bindings.write_to_module(BINDINGS_PATH, false)?;
    println!("cargo:rerun-if-changed={ABI_PATH}");
    Ok(())
}
