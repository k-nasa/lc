use anyhow::{bail, Result};
use regex::Regex;
use std::path::PathBuf;
use tokio::sync::mpsc;
use tokio::task;

use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.3.0", author = "k-nasa")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    files: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    if opts.files.len() < 2 {
        println!("Usage example: lc README.md Makefile src/");
        std::process::exit(1);
    }

    for filepath in args_to_filepaths(&opts.files) {
        println!("\x1b[01;36m=== Verify {:?} === \x1b[m", filepath);

        let text = match tokio::fs::read_to_string(&filepath).await {
            Err(e) => {
                println!("\x1b[01;31mError verify {:?}: \x1b[m{}", filepath, e);
                continue;
            }

            Ok(text) => text,
        };
        let links = find_link(&text);

        let (mut tx, mut rx) = mpsc::channel(10);
        tokio::spawn(async move {
            for link in links {
                let handler = task::spawn(async move { verify_link(link).await });

                match tx.send(handler).await {
                    Err(e) => println!("\x1b[01;31mErr \x1b[m internal error: {}", e),
                    Ok(_) => (),
                };
            }
        });

        while let Some(f) = rx.recv().await {
            let result = f.await.unwrap();
            match result {
                Err(e) => println!("\x1b[01;31mErr \x1b[m {}", e),
                Ok(v) => println!("\x1b[01;32mOk\x1b[m {}", v),
            }
        }

        println!("");
    }

    Ok(())
}

fn args_to_filepaths(files: &[String]) -> Vec<PathBuf> {
    let mut filepaths = vec![];
    for filepath in files {
        let path = std::path::PathBuf::from(filepath);
        filepaths.append(&mut walk_dir(&path));
    }

    filepaths
}

fn walk_dir(path_buf: &PathBuf) -> Vec<PathBuf> {
    let mut filepaths = vec![];

    if path_buf.is_dir() {
        for entry in std::fs::read_dir(path_buf).unwrap() {
            let path = entry.unwrap().path();
            filepaths.append(&mut walk_dir(&path));
        }
    } else {
        filepaths.push(path_buf.to_path_buf());
    }

    filepaths
}

fn find_link(text: &str) -> Vec<String> {
    let r = Regex::new(r"https?://[\w!?/\+\-_~=;\.,*&@#$%]+").unwrap();

    r.find_iter(text).map(|m| m.as_str().to_string()).collect()
}

async fn verify_link(link: String) -> Result<String> {
    let res = reqwest::get(&link).await?;

    let status_code = res.status();
    if !status_code.is_success() {
        bail!("{} -> status code {}", link, status_code);
    }

    Ok(link.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    static DUMMY_TEXT: &str = r###"
# lc

## Overview

[![Actions Status](https://github.com/k-nasa/lc/workflows/CI/badge.svg)](https://github.com/k-nasa/lc/actions)
[![crate-name at crates.io](https://img.shields.io/crates/v/lc.svg)](https://crates.io/crates/lc)

Markdown link checker"###;

    #[test]
    fn test_find_link() {
        let links = find_link(DUMMY_TEXT);

        assert_eq!(
            links,
            vec![
                "https://github.com/k-nasa/lc/workflows/CI/badge.svg",
                "https://github.com/k-nasa/lc/actions",
                "https://img.shields.io/crates/v/lc.svg",
                "https://crates.io/crates/lc",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
        )
    }
}
