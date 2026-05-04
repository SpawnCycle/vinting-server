#![allow(clippy::unwrap_used)]

use std::{
    fs,
    ops::Not,
    process::{self, Command},
};

fn main() {
    // probably not needed in the newer rust versions
    cargo_build::rerun_if_changed("./build.rs");
    // rerun if we're on a different commit
    cargo_build::rerun_if_changed("./.git/HEAD");
    // rerun if we're on a different submodule commit
    cargo_build::rerun_if_changed("./.git/modules/vinting-web/HEAD");
    // reruns if this doesn't exist
    cargo_build::rerun_if_changed("./web/index.html");

    // exits early to not touch anything npm related,
    // good for testing
    if option_env!("NO_WEB").is_some() {
        return;
    }

    // npm gives a rather cryptic error if this happens
    if !fs::exists("./vinting-web/package.json").unwrap_or(false) {
        cargo_build::error(
            "Git repo cloned without submodules, please clone the repo with the --recursive flag",
        );
        process::exit(1);
    }

    let npm = option_env!("NPM").unwrap_or("npm");
    let rebuild = option_env!("REBUILD").is_some();

    let build_out = rebuild || !fs::exists("./web/").unwrap_or(false);
    let install_deps = rebuild || !fs::exists("./vinting-web/node_modules/").unwrap_or(false);

    // Don't run checks if not necessary, particularly useful for windows,
    // where by default npm is not in your path if you install it through scoop or choco
    if !build_out && !install_deps && !rebuild {
        return;
    }

    // check if npm is present in $PATH and is executable
    // spawning the process won't error if it's executable
    // npm will exit with an error with these args, but we don't check that
    Command::new(npm).spawn().is_err().then(|| {
        cargo_build::error(&format!("'{npm}' is not in your $PATH"));
        process::exit(1);
    });

    if install_deps {
        // download deps
        let mut i = Command::new(npm)
            .current_dir("./vinting-web/")
            .args(["install"])
            .spawn()
            .unwrap();
        i.wait()
            .unwrap()
            .success()
            .not()
            .then(|| panic!("There was an error running 'npm install'"));
    }

    // ~10s without this check as opposed with ~1.5s (on my machine) btw
    if build_out {
        // build the website
        let mut c = Command::new(npm)
            .current_dir("./vinting-web/")
            .args(["run", "build"])
            .spawn()
            .unwrap();
        c.wait()
            .unwrap()
            .success()
            .not()
            .then(|| panic!("There was an error running 'npm run build'"));
    }
}
