use anyhow::{bail, Result};
use regex::Regex;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage {} <filepath>", args[0]);
        std::process::exit(1);
    }

    let filepath = &args[1];

    let text = std::fs::read_to_string(filepath)?;
    let links = find_link(&text);

    let mut futures = vec![];
    for link in links {
        let handler = tokio::spawn(async move { verify_link(link).await });
        futures.push(handler)
    }

    for f in futures {
        let result = f.await.unwrap();
        match result {
            Err(e) => println!("\x1b[01;31mErr \x1b[m {}", e),
            Ok(v) => println!("\x1b[01;32mOk\x1b[m {}", v),
        }
    }

    Ok(())
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

    static dummy_text: &str = r###"
# lc

## Overview

[![Actions Status](https://github.com/k-nasa/lc/workflows/CI/badge.svg)](https://github.com/k-nasa/lc/actions)
[![crate-name at crates.io](https://img.shields.io/crates/v/lc.svg)](https://crates.io/crates/lc)

Markdown link checker"###;

    #[test]
    fn test_find_link() {
        let links = find_link(dummy_text);

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
