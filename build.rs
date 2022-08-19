use ethers_contract::MultiAbigen;
use std::fs::canonicalize;

fn main() {
    let input_path = canonicalize("./abi").unwrap();
    let output_path = canonicalize("./src/bindings").unwrap();

    let gen = MultiAbigen::from_json_files(&input_path).unwrap();
    let bindings = gen.build().unwrap();
    bindings.write_to_module(&output_path, false).unwrap();

    println!("cargo:rerun-if-changed={}", input_path.display())
}
