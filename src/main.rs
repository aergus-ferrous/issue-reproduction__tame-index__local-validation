use std::path::Path;

use tame_index::{
    KrateName,
    index::{FileLock, local::LocalRegistry},
};

static BAD_REGISTRY_PATH: &str = "bad-registry";
static KRATE_NAME: &str = "krate";

fn main() {
    let registry = LocalRegistry::open(BAD_REGISTRY_PATH.into(), true).unwrap();
    let krate = registry
        .cached_krate(KrateName::cargo(KRATE_NAME).unwrap(), &FileLock::unlocked())
        .unwrap()
        .unwrap()
        .versions
        .pop()
        .unwrap();
    println!(
        "name: {}, version: {}, checksum: {}",
        krate.name, krate.version, krate.checksum
    );
    let crate_file_path =
        Path::new(BAD_REGISTRY_PATH).join(format!("{}-{}.crate", krate.name, krate.version));
    println!(
        "`{}` exists: {}",
        crate_file_path.display(),
        crate_file_path.exists()
    )
}
