//! See <https://github.com/matklad/cargo-xtask/>.
//!
//! This binary defines various auxiliary build commands, which are not
//! expressible with just `cargo`.
//!
//! This binary is integrated into the `cargo` command line by using an alias in
//! `.cargo/config`.

mod flags;

mod init_test_db;

use std::{env, path::PathBuf};

fn main() -> eyre::Result<()> {
    let flags = flags::Xtask::from_env_or_exit();

    let project_root = project_root();

    match flags.subcommand {
        flags::XtaskCmd::InitTestDb(cmd) => cmd.run(&project_root),
    }
}

/// Returns the path to the root directory of `sblex-rs` project.
fn project_root() -> PathBuf {
    let dir =
        env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());
    PathBuf::from(dir).parent().unwrap().to_owned()
}
