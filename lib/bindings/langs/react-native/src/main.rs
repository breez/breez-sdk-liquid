mod gen_kotlin;
mod gen_swift;
mod gen_typescript;
mod generator;
use camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;
use generator::ReactNativeBindingGenerator;

#[derive(Parser, Debug)]
pub(crate) struct Cli {
    /// Path to the UDL file
    #[clap(name = "binding_dir", short = 'b', long = "binding_dir")]
    pub(crate) binding_dir: Option<String>,

    /// Directory in which to write generated files. Default is same folder as .udl file.
    #[clap(name = "out_dir", short = 'o', long = "out_dir")]
    pub(crate) out_dir: Option<String>,

    /// Extract proc-macro metadata from a native lib (cdylib or staticlib) for this crate.
    #[clap(long, short)]
    lib_file: Option<Utf8PathBuf>,

    /// This as the crate name instead of attempting to locate and parse Cargo.toml.
    #[clap(long = "crate")]
    crate_name: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let cli_binding_dir = cli.binding_dir.unwrap_or("../../".into());
    let cli_out_dir = cli.out_dir.unwrap_or("./".into());
    let binding_dir = Utf8Path::new(cli_binding_dir.as_str());
    let udl_file = binding_dir.join(Utf8Path::new("src/breez_sdk_liquid.udl"));
    let config = binding_dir.join(Utf8Path::new("uniffi.toml"));
    let out_dir = Utf8Path::new(cli_out_dir.as_str());

    // React Native generator
    uniffi_bindgen::generate_external_bindings(
        ReactNativeBindingGenerator {},
        udl_file,
        Some(config),
        Some(out_dir),
        cli.lib_file,
        cli.crate_name.as_deref(),
    )
    .unwrap();
}
