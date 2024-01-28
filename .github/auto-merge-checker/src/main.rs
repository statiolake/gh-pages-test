use std::{env, process::Command};

fn get_changed_files() -> Vec<String> {
    let github_base_ref = env::var("GITHUB_BASE_REF").unwrap();
    let github_head_ref = env::var("GITHUB_HEAD_REF").unwrap();

    String::from_utf8_lossy(
        &Command::new("git")
            .arg("diff")
            .arg("--name-only")
            .arg(format!(
                "origin/{github_base_ref}...origin/{github_head_ref}"
            ))
            .output()
            .unwrap()
            .stdout,
    )
    .lines()
    .map(|s| s.to_string())
    .collect()
}

fn main() {
    let changed_files = get_changed_files();
    assert_eq!(changed_files, vec!["README.md"]);
}
