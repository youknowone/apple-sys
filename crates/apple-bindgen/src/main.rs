use apple_bindgen::{Builder, DependencyAnalyzer, SdkPath, SdkPathError};
use apple_sdk::Platform;
use clap::{Args, Parser, Subcommand};
use std::str::FromStr;

#[derive(Debug, Clone)]
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
            _ => Sdk::Path(std::path::PathBuf::from(s).try_into()?),
        })
    }
}

/// Generate Rust bindings for Apple frameworks
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate bindings for a framework (default command)
    #[clap(alias = "gen")]
    Generate(GenerateArgs),

    /// Analyze framework dependencies from umbrella header
    AnalyzeDeps(AnalyzeDepsArgs),
}

#[derive(Args, Debug)]
struct GenerateArgs {
    /// e.g. CoreFoundation, UIKit, etc.
    framework: String,

    /// SDK name or path. One of: macosx, iphoneos, iphonesimulator, watchos, appletvos, driverkit
    #[clap(long)]
    sdk: Sdk,

    /// Specify target architecture
    #[clap(long)]
    target: Option<String>,

    /// Print build details
    #[clap(short, long, num_args = 0)]
    verbose: bool,
}

#[derive(Args, Debug)]
struct AnalyzeDepsArgs {
    /// Framework to analyze
    framework: String,

    /// SDK name or path
    #[clap(long)]
    sdk: Sdk,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate(args) => cmd_generate(args),
        Command::AnalyzeDeps(args) => cmd_analyze_deps(args),
    }
}

fn cmd_generate(args: GenerateArgs) {
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

fn cmd_analyze_deps(args: AnalyzeDepsArgs) {
    let sdk_path: SdkPath = args.sdk.try_into().expect("Invalid SDK");
    let analyzer = DependencyAnalyzer::new(sdk_path.path());

    match analyzer.analyze(&args.framework) {
        Ok(deps) => {
            if deps.is_empty() {
                eprintln!("{} has no external dependencies", args.framework);
            } else {
                println!("{} depends on:", args.framework);
                for dep in deps {
                    println!("  - {}", dep);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to analyze {}: {}", args.framework, e);
            std::process::exit(1);
        }
    }
}
