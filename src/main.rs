use anyhow::Result;

use argh::FromArgs;

#[derive(FromArgs)]
/// Arguments
struct MyArgs {
    /// a value known as "a"
    #[argh(option)]
    a: u8,
    /// a value known as "b"
    #[argh(option)]
    b: u8
}

fn main() -> Result<()> {
    let args: MyArgs = argh::from_env();

    println!(
        "I'm using the library: {:?}",
        galg::really_complicated_code(args.a, args.b)
    );
    Ok(())
}
