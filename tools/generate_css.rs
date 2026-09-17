#[path = "../crates/ferriswatch-core/src/css/definitions.rs"]
mod definitions;

fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");

            std::process::ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();

    let check = match args.as_slice() {
        [] => false,
        [arg] if arg == "--check" => true,
        _ => {
            return Err(
                "Usage: ferriswatch-generate-css [--check] (run from the repository root)".into(),
            );
        }
    };

    let path = std::path::Path::new("crates/ferriswatch-core/src/css/default.css");

    let expected = definitions::render();

    let current = match std::fs::read(path) {
        Ok(bytes) => Some(bytes),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    };

    if current.as_deref() == Some(expected.as_bytes()) {
        return Ok(());
    }

    if check {
        return Err("Generated CSS is missing or stale. Run `cargo run --no-default-features --bin ferriswatch-generate-css`, review, and stage src/css/default.css.".into());
    }

    std::fs::write(path, expected)?;

    println!("Generated {}", path.display());

    Ok(())
}
