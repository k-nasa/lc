use anyhow::{bail, Result};
use regex::Regex;
use std::path::PathBuf;
use tokio::task;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage example: {} README.md Makefile src/", args[0]);
        std::process::exit(1);
    }

    for filepath in args_to_filepaths(&args) {
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

fn args_to_filepaths(args: &[String]) -> Vec<PathBuf> {
    let mut filepaths = vec![];
    for filepath in &args[1..] {
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
