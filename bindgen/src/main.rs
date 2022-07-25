use apple_bindgen::{Builder, SdkPath, SdkPathError};
use apple_sdk::Platform;
use clap::Parser;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug)]
enum Sdk {
    Name(Platform),
    Path(SdkPath),
}

impl TryFrom<Sdk> for SdkPath {
    type Error = SdkPathError;

    fn try_from(sdk: Sdk) -> Result<Self, Self::Error> {
        Ok(match sdk {
            Sdk::Name(ref platform) => platform.try_into()?,
            Sdk::Path(path) => path,
        })
    }
}

impl FromStr for Sdk {
    type Err = SdkPathError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match Platform::from_str(s) {
            Ok(name) if !matches!(name, Platform::Unknown(_)) => Sdk::Name(name),
            _ => Sdk::Path(PathBuf::from(s).try_into()?),
        })
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// e.g. CoreFoundation, UIKit, etc.
    #[clap(value_parser, index(1))]
    framework: String,

    /// SDK name. One of MacOSX, iPhoneOS, iPhoneSimulator, WatchOS, AppleTVOS, AppleTVSimulator, DriverKit.
    #[clap(long)]
    sdk: Sdk,

    /// Specify target architecture. Defaults to system architecture.
    #[clap(long)]
    target: Option<String>,

    /// Print build details
    #[clap(short, long, takes_value = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let mut builder =
        Builder::with_builtin_config(&args.framework, args.sdk).expect("sdk lookup failed");
    if args.verbose {
        eprintln!("config: {:?}", builder);
        eprintln!(
            "bindgen {:?}",
            builder.bindgen_builder().command_line_flags().join(" ")
        );
    }
    if let Some(target) = args.target {
        builder = builder.target(target);
    }
    let out = builder.generate().expect("binding generation failed");
    println!("{}", out);
}
