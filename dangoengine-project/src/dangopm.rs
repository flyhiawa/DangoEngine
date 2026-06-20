use std::fs::{create_dir_all, File};
use std::io::{self, Write,};
use std::path::PathBuf;
use clap::{Parser, Subcommand, Args};
use flate2::read::GzDecoder;
use tar::Archive;
#[derive(Parser)]
#[command(name="dangopm")]
#[command(about="Pack Manager of DangoEngine", long_about=None)]
#[command(disable_version_flag = true)]
struct CLI{
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, help="Returns the current version of the 'DangoEngine' project")]
    version: bool,
}
#[derive(Subcommand)]
enum Commands{
    #[command(about="Create a structured dangopm project.")]
    New(NewArgs),
}

#[derive(Args, Debug)]
struct NewArgs{
    name: String,
    #[arg(long="search_tar_gz", help="WARNING: Use this flag with care, only place compressed files whose origin is trustworthy | Transfers all folders and files from a .tar.gz to includes. is the method of officially transferring libraries in DangoEngine")]
    search_tar_gz: Option<PathBuf>,
}
pub fn exec() -> io::Result<()>{
    let cli = CLI::parse();
    if cli.version{
        println!("DangoEngine | \x1B[31m{}\x1B[0m.\x1B[32m{}\x1B[0m.\x1B[034m{}\x1B[0m", env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"), env!("CARGO_PKG_VERSION_PATCH"));
        return Ok(());
    }
    match &cli.command{
        Some(Commands::New(args)) =>{
            println!("New DangoEngine Project | Name: \x1B[31m{}\x1B[0m", args.name);
            create_dir_all(format!("{}/src", args.name))?;
            create_dir_all(format!("{}/includes", args.name))?;
            let mut _main_lgc = File::create(format!("{}/src/main.dango", args.name))?;
            let mut configf_toml = File::create(format!("{}/Dangoc.toml", args.name))?;
            write!(&mut configf_toml,
                r#"[project]
name = "{project_name}"
version = "{version}"

[libraries]

# This is a TOML file of any lgcpm project.
                "#,
                project_name=args.name,
                version=env!("CARGO_PKG_VERSION")
            )?;
            if let Some(tar_gz) = &args.search_tar_gz{
                let fileo = File::open(tar_gz)?;
                let decod = GzDecoder::new(fileo);
                let mut tarf = Archive::new(decod);
                tarf.unpack(format!("{}/includes", args.name))?;
            }
        }
        None =>{
            println!("\x1B[31mERROR\x1B[0m: Use a subcommand or type '--help' to see options.");
        }
    }
    Ok(())
}

fn main() -> io::Result<()>{
    exec()
}
