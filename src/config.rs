use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use structopt::StructOpt;

/// ~/ripley/config.yaml
pub struct ToolOptions {
    pub naming: NamingOptions,
}
pub struct NamingOptions {
    pub suffix: HashMap<String, String>,
}

impl ToolOptions {
    pub fn from_file() -> Option<ToolOptions> {
        Some(ToolOptions {
            naming: NamingOptions {
                suffix: HashMap::from([
                    ("prod".to_string(), "prod".to_string()),
                    ("dev".to_string(), "dev".to_string()),
                    ("test".to_string(), "test".to_string()),
                ]),
            },
        })
    }
}

/// ./ripley.toml
#[derive(Deserialize)]
pub struct ProjectOptions {}

impl ProjectOptions {
    pub fn from_file() -> Option<ProjectOptions> {
        // look up until ripley.toml
        let mut cwd = env::current_dir().unwrap();
        cwd.push("ripley.toml");

        if let Ok(mut file) = File::open(cwd) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            // if file
            if let Ok(options) = serde_yaml::from_str(&contents) {
                return Some(options);
            }
        }

        None
    }
}

/// ripley create <name>
/// ripley reset
/// ripley pull
#[derive(Debug, StructOpt)]
#[structopt(name = "ripley", about = "Database Management Tool")]
pub struct RunOptions {
    // short and long flags (-d, --database) will be deduced from the field's name
    #[structopt(short, long)]
    database: Option<String>,

    // /// Set speed
    // // we don't want to name it "speed", need to look smart
    // #[structopt(short = "v", long = "velocity", default_value = "42")]
    // speed: f64,
    //
    // /// Input file
    // #[structopt(parse(from_os_str))]
    // input: PathBuf,
    //
    // /// Output file, stdout if not present
    // #[structopt(parse(from_os_str))]
    // output: Option<PathBuf>,
    //
    // /// Where to write the output: to `stdout` or `file`
    // #[structopt(short)]
    // out_type: String,
    //
    // /// File name: only required when `out-type` is set to `file`
    // #[structopt(name = "FILE", required_if("out-type", "file"))]
    // file_name: Option<String>,
    #[structopt(subcommand)]
    pub cmd: Option<RunSubCommand>,
}

#[derive(Debug, StructOpt)]
pub enum RunSubCommand {
    Create { name: String },
    Reset,
    Pull,
}
