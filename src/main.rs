use adsmacroexpand::{find_bibliography_file, update_bibliography_file};
use anyhow::{Context, Result};
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    inplace: bool,

    #[arg(short, long)]
    diff: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let root =
        fs::canonicalize(&args.path).context(format!("Cannot canonicalize path {}", args.path))?;
    let bib_file = find_bibliography_file(&root)
        .context(format!("No paper.md found in {}", root.display()))?;
    let mut reader = fs::File::open(&bib_file).context(format!(
        "Cannot open bibliography file {}",
        bib_file.display()
    ))?;

    let mut temp_buf = Vec::new();
    let diff = update_bibliography_file(&mut reader, &mut temp_buf)?;

    if args.diff {
        print!("{}", diff);
    }

    if args.inplace {
        fs::write(&bib_file, temp_buf.as_slice())
            .context(format!("Cannot write to {}", bib_file.display()))?;
    } else if let Some(path) = args.output {
        fs::write(&path, temp_buf.as_slice()).context(format!("Cannot write to {}", path))?;
    } else if !args.diff {
        println!("{}", String::from_utf8(temp_buf)?);
    }

    Ok(())
}
