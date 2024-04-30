use std::io::Write;

use anyhow::Result;
use clap::Parser;
use pi_lib::{
    env::EvalCtx,
    parse::{handle_statement, CmdParser},
};

fn propmt() -> Result<String> {
    print!(">>> ");
    std::io::stdout().flush()?;
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    Ok(line.trim().to_string())
}

#[derive(Parser, Debug)]
#[command(name = "kvm-monitor")]
#[command(author = "Haobin Hiroki Chen. <haobchen@iu.edu>")]
#[command(version = "1.0")]
pub struct Args {
    #[clap(short, long, default_value = "", help = "The input file to parse.")]
    input: String,

    #[clap(long, default_value = "false", help = "Enter interactive mode.")]
    interactive: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.interactive {
        println!("Welcome to the Pi interpreter!");
        println!("Type 'exit' to quit.\n");

        let parser = CmdParser::new();
        let mut ctx = EvalCtx::new();
        loop {
            let input = propmt()?;

            if input.trim().to_lowercase() == "exit" {
                return Ok(());
            }

            if input.trim().to_lowercase() == "show" {
                println!("{:?}", ctx);
                continue;
            }

            let cmd = match parser.parse(input.as_str()) {
                Ok(cmd) => cmd,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            };

            match handle_statement(cmd, &mut ctx) {
                Ok(res) => println!("{:?}", res),
                Err(e) => eprintln!("{}", e),
            }
        }
    } else {
        let res = pi_lib::parse::eval_file(&args.input)?;
        println!("{:?}", res);
        Ok(())
    }
}
