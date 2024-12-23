use std::process::Command;
use std::io;
use std::io::Write;
use std::fs;
use std::fs::OpenOptions;

fn create_yew_app(app_name : &str) {
   create_new_project(app_name);
   update_cargo_toml(app_name).expect("Failed to update Cargo.toml");
   add_html_file(app_name).expect("Failed to add HTML file");
   update_main_rs(app_name).expect("Failed to update main.rs");
}

fn create_new_project(name: &str) {
    Command::new("cargo")
        .arg("new")
        .arg(name)
        .output()
        .expect("Failed to create new Cargo project");
}

fn update_cargo_toml(name: &str) -> io::Result<()> {
    let path = format!("{}/Cargo.toml", name);
    let mut file = OpenOptions::new()
        .append(true)
        .open(path)?;

    // Append or modify entries in the Cargo.toml here
    writeln!(file, "yew = {{ version = \"0.21.0\", features = [\"csr\"] }}")?;
    writeln!(file, "wasm-bindgen = \"0.2\"")?;
    writeln!(file, "web-sys = \"0.3\"")?;

    Ok(())
}

fn add_html_file(name: &str) -> std::io::Result<()> {
    let path = format!("{}/index.html", name);

    let content = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Yew App</title>
</head>
<body>
    <div id="app"></div>
    <script type="module" src="/pkg/your_app_name.js"></script>
</body>
</html>
"#;

    fs::write(path, content)?;

    Ok(())
}


fn update_main_rs(name: &str) -> std::io::Result<()> {
    let path = format!("{}/src/main.rs", name);

    let content = r#"
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>{ "Hello, Yew!" }</div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();}
"#;

    fs::write(path, content)?;

    Ok(())
}

fn main() {
    // Collect user input
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");


    // Trim the input to remove newline or whitespace
    let input = input.trim();

    match input {
        cmd if cmd.starts_with("create-yew-app") => {
            if let Some(app_name) = cmd.strip_prefix("create-yew-app ").map(str::trim) {
                if !app_name.is_empty() {
                    println!("Creating Yew app named: {}", app_name);
                    create_yew_app(app_name);


                } else {
                    println!("Error: No app name provided. Usage: create-yew-app <app-name>");
                }
            } else {
                println!("Error: No app name provided. Usage: create-yew-app <app-name>");
            }
        }
        _ => {
            println!("Unknown command: {}", input);
        }
    }
}