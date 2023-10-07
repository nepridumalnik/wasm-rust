use std::env;
use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::current_dir()?;
    let pkg_dir = format!("{}/web/static/pkg", path.display());

    let status = Command::new("wasm-pack")
        .args(&[
            "build",
            "src/wasm_core",
            "--target",
            "web",
            "--out-dir",
            &pkg_dir,
        ])
        .status()
        .expect("Не удалось выполнить команду 'wasm-pack build'");

    if !status.success() {
        panic!("Команда 'wasm-pack build' завершилась с ошибкой");
    }

    Ok(())
}
