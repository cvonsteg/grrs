use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let path = &args.path;
    // TODO: replace read with std::io::BufReader implementation
    let content = std::fs::read_to_string(path)
        .with_context(|_| format!("cannae read file `{}`", path))?;
    grrs::find_matches(content, &args.pattern, &mut std::io::stdout()); 
    Ok(())
    
}


