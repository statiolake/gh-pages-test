use chrono::Local;

fn main() {
    let now = Local::now();

    let page = include_str!("template.html");
    let page = page.replace("{{ date }}", &now.format("%Y-%m-%d %H:%M:%S").to_string());
    println!("{}", page);
}
