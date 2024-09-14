use clap::Parser;
use rcli::{process_csv, process_decode, process_encode, process_genpass, process_http_serve, Base64SubCommand, HttpSubCommand, Opts, SubCommand};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => {
            match subcmd {
                Base64SubCommand::Encode(opts) => {
                    process_encode(&opts.input, opts.format)?;
                }
                Base64SubCommand::Decode(opts) => {
                    process_decode(&opts.input, opts.format)?;
                }
            }
        }
        SubCommand::Http(subcmd) => {
            match subcmd {
                HttpSubCommand::Serve(opts) => {
                    process_http_serve(opts.dir, opts.port).await?;
                }
            }
        }
    }
    Ok(())
}
