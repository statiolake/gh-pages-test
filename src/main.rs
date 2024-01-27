use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fs, process::Command};
use tera::{Context, Tera};

#[derive(Serialize, Deserialize)]
pub struct Repository {
    name: String,
    url: String,
    visibility: String,
}

fn get_repositories() -> Vec<Repository> {
    let output = Command::new("gh")
        .arg("repo")
        .arg("list")
        .arg("--json")
        .arg("name,url,visibility")
        .arg("--limit")
        .arg("1000")
        .output()
        .unwrap();

    let stdout = output.stdout;
    if let Ok(repositories) = serde_json::from_slice::<Vec<Repository>>(&stdout) {
        repositories
            .into_iter()
            .filter(|repo| repo.visibility == "PUBLIC")
            .collect()
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("error: failed to parse repositories: {}", stderr);
    }
}

fn render(now: DateTime<Utc>, repositories: &[Repository]) -> String {
    let tera = Tera::new("templates/**/*.html").unwrap();
    let mut context = Context::new();
    context.insert("date", &now.to_rfc3339());
    context.insert("repositories", repositories);
    tera.render("index.html", &context).unwrap()
}

fn main() {
    let now = Utc::now();
    let repositories: Vec<_> = get_repositories();
    let page = render(now, &repositories);
    fs::create_dir_all("dist").unwrap();
    fs::write("dist/index.html", page).unwrap();
}
