use std::{env, fs, path::Path};

use specbind::schema::generate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let check = match env::args().nth(1).as_deref() {
        None => false,
        Some("--check") => true,
        Some(argument) => return Err(format!("unknown argument: {argument}").into()),
    };

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    write_or_check(
        &root.join("schemas/spec/v1.schema.json"),
        &generate::to_pretty_json(&generate::spec_v1())?,
        check,
    )?;
    write_or_check(
        &root.join("schemas/tasks/v1.schema.json"),
        &generate::to_pretty_json(&generate::tasks_v1())?,
        check,
    )?;

    Ok(())
}

fn write_or_check(
    path: &Path,
    generated: &str,
    check: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if check {
        let checked_in = fs::read_to_string(path)?;
        if checked_in != generated {
            return Err(format!("generated schema differs: {}", path.display()).into());
        }
    } else {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, generated)?;
    }

    Ok(())
}
