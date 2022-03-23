
// a mini grep clone

use clap::Parser;
use anyhow::{ Context, Result };

#[derive(Parser)]
pub struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf
}

pub fn grepit() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // plan to change read_to_string with BufReader
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read the file: {:?}", &args.path))?;

    for line in content.lines()  {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
