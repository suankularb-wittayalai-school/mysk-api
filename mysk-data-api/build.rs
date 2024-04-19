use chrono::offset::Local;
use std::{env, process::Command};

fn main() {
    let target_triple = env::var("TARGET").unwrap();

    println!("cargo::rustc-env=TARGET_TRIPLE={}", target_triple);

    let is_git_up_to_date = String::from_utf8(
        Command::new("git")
            .arg("status")
            .arg("--porcelain")
            .output()
            .expect("Failed to execute git-status")
            .stdout,
    )
    .expect("Failed to parse output of git-status")
    .is_empty();
    if is_git_up_to_date {
        let git_commit_short_hash = String::from_utf8(
            Command::new("git")
                .arg("rev-parse")
                .arg("--short")
                .arg("HEAD")
                .output()
                .expect("Failed to execute git-rev-parse")
                .stdout,
        )
        .expect("Failed to parse output of git-rev-parse");
        let git_commit_date = String::from_utf8(
            Command::new("git")
                .arg("show")
                .arg("--no-patch")
                .arg("--format=%ci")
                .arg("HEAD")
                .output()
                .expect("Failed to execute git-show")
                .stdout,
        )
        .expect("Failed to parse output of git-show");
        // Get only the date part of the string
        let git_commit_date = git_commit_date.split_at(10).0;

        println!(
            "cargo::rustc-env=COMMIT_SHORT_HASH={}",
            git_commit_short_hash,
        );
        println!("cargo::rustc-env=COMMIT_DATE{}", git_commit_date);
    } else {
        let build_date = Local::now().format("%F");

        println!("cargo::rustc-env=COMMIT_SHORT_HASH=untracked");
        println!("cargo::rustc-env=COMMIT_DATE={}", build_date);
    }
}
