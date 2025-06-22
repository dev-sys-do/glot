// SPDX-FileCopyrightText: 2025 Polytech Montpellier.
//
// SPDX-License-Identifier: Apache-2.0

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use glot::Error;
use glot::tokenizer::GlotLine;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// glot source file
    #[arg(short, long, value_name = "FILE")]
    source: PathBuf,
}

#[derive(Debug, Clone)]
struct Glotter {
    source: PathBuf,
    lines: Vec<GlotLine>,
}

impl Glotter {
    pub fn new_from_file(source_path: &Path) -> Result<Self, Error> {
        Ok(Glotter {
            source: source_path.to_path_buf(),
            lines: Vec::new(),
        })
    }

    pub fn tokenize(&mut self) -> Result<(), Error> {
        let source_file = File::open(self.source.clone())
            .map_err(|_| Error::InvalidSourceFile(self.source.clone()))?;
        let source = BufReader::new(source_file);

        for line in source.lines() {
            let line = line.unwrap();
            self.lines.push(GlotLine::new(&line)?);
        }

        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let mut glotter = Glotter::new_from_file(&cli.source)?;
    glotter.tokenize()?;

    for line in glotter.lines {
        println!("{:?}", line);
    }

    Ok(())
}
