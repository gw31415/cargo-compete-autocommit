use anyhow::{anyhow, ensure, Ok, Result};
use std::env::args;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const SLASH: char = '/';

fn main() -> Result<()> {
    let (contest, question) = {
        // get the only-one argument
        let arg = {
            let args = args();
            if args.len() == 1 {
                println!("cargo-compete-autocommit by Amadeus_vn");
                return Ok(());
            } else if args.len() > 2 {
                return Err(anyhow!("invalid number of arguments."));
            }
            args.into_iter().nth(1).unwrap()
        };
        // count slash chars in the arg
        let mut slash_onlyone = false;
        for c in arg.chars() {
            if c == SLASH {
                if slash_onlyone {
                    slash_onlyone = false;
                    break;
                } else {
                    slash_onlyone = true;
                }
            }
        }
        if !slash_onlyone {
            return Err(anyhow!("invalid number of '/' in the argument."));
        }
        // now number of slash chars in args is only one.
        let mut sp = arg.split(SLASH);
        let c = sp.next().unwrap();
        let q = sp.next().unwrap();
        (c.to_string(), q.to_string())
    };
    // TODO: escape of contest & question
    println!("contest: {contest}; question: {question}");

    // the root directory of the cargo-compete project
    let git_root = {
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()?;
        let mut output_str = String::from_utf8(output.stdout)?;
        if output_str.ends_with('\n') {
            output_str.pop();
        }
        output_str
    };

    // create git adding args
    let mut git_adding_args = vec![String::from("add")];
    // ensure if files exist and are files.
    for file in vec![
        format!("{git_root}/{contest}/src/bin/{question}.rs"),
        format!("{git_root}/{contest}/Cargo.toml"),
        format!("{git_root}/{contest}/Cargo.lock"),
    ] {
        let source_path = Path::new(&file);
        ensure!(source_path.exists(), "'{}' does not exists.", file);
        ensure!(source_path.is_file(), "'{}' is not file.", file);
        git_adding_args.push(file);
    }
    // testcases directory
    let testcases = format!("{git_root}/{contest}/testcases");
    if Path::new(&testcases).exists() {
        git_adding_args.push(testcases);
    }

    // execute git commands
    {
        // git add
        let output = Command::new("git")
            .current_dir(&git_root)
            .args(git_adding_args)
            .output()?;
        std::io::stdout().write_all(&output.stdout).unwrap();
        std::io::stderr().write_all(&output.stderr).unwrap();
    }
    {
        // git commit
        let output = Command::new("git")
            .current_dir(&git_root)
            .args([
                "commit",
                "-m",
                &format!("Done question {question} of contest:{contest}"),
            ])
            .output()?;
        std::io::stdout().write_all(&output.stdout).unwrap();
        std::io::stderr().write_all(&output.stderr).unwrap();
    }
    Ok(())
}
