use chrono::Local;
use serde::Deserialize;
use std::{fs, process::Command};

#[derive(Deserialize)]
pub struct Repository {
    name: String,
    url: String,
    visibility: String,
}

fn get_repositories() -> Vec<Repository> {
    let stdout = Command::new("gh")
        .arg("repo")
        .arg("list")
        .arg("--json")
        .arg("name,url,visibility")
        .arg("--limit")
        .arg("1000")
        .output()
        .unwrap()
        .stdout;

    serde_json::from_slice::<Vec<Repository>>(&stdout)
        .unwrap()
        .into_iter()
        .filter(|repo| repo.visibility == "PUBLIC")
        .collect()
}

fn main() {
    let now = Local::now();

    let page = include_str!("template.html");
    let page = page.replace("{{ date }}", &now.format("%Y-%m-%d %H:%M:%S").to_string());

    let repositories: Vec<_> = get_repositories()
        .into_iter()
        .map(|repo| format!("<li><a href=\"{}\">{}</a></li>", repo.url, repo.name))
        .collect();
    let repositories = format!("<ul>\n{}\n</ul>", repositories.join("\n"));

    let page = page.replace("{{ repositories }}", &repositories);

    fs::create_dir_all("dist").unwrap();
    fs::write("dist/index.html", page).unwrap();
}
