use adsmacroexpand::{find_bibliography_file, update_bibliography_file};
use anyhow::{Context, Result};
use clap::Parser;
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    inplace: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let bib_file = find_bibliography_file(&args.path)
        .context(format!("No paper.md found in {}", args.path))?;
    let mut reader = fs::File::open(&bib_file).context(format!(
        "Cannot open bibliography file {}",
        bib_file.display()
    ))?;

    let mut temp_buf = Vec::new();

    {
        let mut writer: Box<dyn io::Write> = if args.inplace {
            Box::new(&mut temp_buf)
        } else {
            match args.output {
                None => Box::new(io::stdout()),
                Some(path) => Box::new(
                    fs::File::create(&path)
                        .context(format!("Cannot create output file {}", path))?,
                ),
            }
        };

        update_bibliography_file(&mut reader, &mut writer);
    }

    if args.inplace {
        fs::write(&bib_file, temp_buf.as_slice())
            .context(format!("Cannot write to {}", bib_file.display()))?;
    }

    Ok(())
}
