extern crate vergen;
use vergen::{Config, vergen};
use vergen::{ShaKind};

fn main() {
    let mut config = Config::default();
    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    *config.build_mut().semver_mut() = true;
    assert!(vergen(config).is_ok());
}