use flood_tide::{
    parse_simple_gnu_style,
    parse_simple_gnu_style_subcmd,
    Arg,
    HelpVersion,
    SubCommand,
    NameVal,
    Opt,
    OptNum,
    OptParseError
};

use crate::event::EventDispatcher;


// Pry Cli Layout
// +--------------+
// --path -> path for index (where to open)
// --add-watcher -> add index watcher (path, watcher script, event)
// --batch -> execute multiple executors all at once
// execute action (query, update, insert, delete)
// insert <file/directory/tag> <(file/directory) optional: tag>
// update <file/directory/tag> <new_name>
// delete <file/directory/tag>
// query <query> <optional: file/directory/tag>

const DESCRIPTION_TEXT: &str = r#"
Pry, a simple search engine
"#;

const OPTIONS_TEXT: &str  = r#"
Options:
-h, --help                      display this help and exit
-v, --version                   output version information and exit
-p, --path <path>               specify a specfic global path to open the engine index, defaults to the current directory
--add-watcher <path> <script>   add watcher to specific path in the engine index, path defaults to current directory, script  
--batch <executors>             collect multiple executor actions into a single batch call
"#;

const SUBCOMMANDS_TEXT: &str = r#"
Subcommands:
delete <name>                   delete a file, directory, or tag in the engine index
insert <name>                   insert a file, directory, or tag into the engine index
update <name> <new_name>        update a file, directory, or tag in the engine index
query <name>                    query a file, directory, or tag in the engine index, defaults to the toplevel path
"#;

const ARGUMENTS_TEXT: &str = r#"
<path>                          a relative path, defaults to the current directory
<script>
"#;

const OPTIONS: [Opt; 3] = [
    Opt { sho: b'H', lon: "help", has: Arg::No, num: CmdOptions::Help.to() },
    Opt { sho: b'V', lon: "version", has: Arg::No, num: CmdOptions::Version.to() },
    Opt { sho: b'p', lon: "path", has: Arg::Yes, num: CmdOptions::Path.to() },
    // Opt { sho: 0u8, lon: "add-watcher", has: Arg::Yes, num: CmdOptions::AddWatcher.to() },
    // Opt { sho: 0u8, lon: "batch", has: Arg::Yes, num: CmdOptions::Batch.to() },
];

const OPTIONS_SHORT_INDEX: [(u8,usize); 3] = [ // all options with short versions
    (b'H', 0), // number is index in OPTIONS
    (b'V', 1),
    (b'p', 2),
];

const SUBCOMMAND_OPTIONS: [Opt; 3] = [
    Opt { sho: b'f', lon: "file", has: Arg::Yes, num: SubcmdOptions::File.to() },
    Opt { sho: b'd', lon: "directory", has: Arg::Yes, num: SubcmdOptions::Directory.to() },
    Opt { sho: b't', lon: "tag", has: Arg::Yes, num: SubcmdOptions::Tag.to() },
];

const SUBCOMMAND_OPTIONS_SHORT_INDEX: [(u8, usize); 3] = [
    (b'f', 3),
    (b'd', 4),
    (b't', 5),
];

const SUBCOMMANDS: [&str; 4] = [
    "delete",
    "insert",
    "query",
    "update"
];

#[repr(u8)]
#[derive(Debug, PartialEq)]
enum CmdOptions {
    Help,
    Version,
    //
    Path,
    //
    File,
    Directory,
    Tag,
    //
    AddWatcher,
    Batch,
}

impl std::convert::From<OptNum> for CmdOptions {
    fn from(value: OptNum) -> Self {
        unsafe { std::mem::transmute_copy(&value) }
    }
}

impl CmdOptions {
    const fn to(self) -> OptNum {
        self as OptNum
    }
}

#[derive(Debug, Default)]
pub struct CmdOptionsConfig {
    program_name: String,
    //
    flag_help: bool,
    flag_version: bool,
    //
    pub glb_path: String,
    pub glb_watchers: Vec<String>, // TODO: replace with watcher type
    glb_batch: Vec<String>, // TODO: convert to subcmd
    //
    pub subcmd_conf: SubcmdOptionsConfig,
    program_params: Vec<String>,
}

impl HelpVersion for CmdOptionsConfig {
    fn is_help(&self) -> bool {
        self.flag_help
    }
    fn is_version(&self) -> bool {
        self.flag_version
    }
}

impl SubCommand for CmdOptionsConfig {
    fn set_subcmd(&mut self, subcmd: String) {
        match subcmd.as_str() {
            "insert" => { self.subcmd_conf.subcmd_insert = true; println!("entering insert subcmd"); },
            "query" => { self.subcmd_conf.subcmd_query = true; println!("entering query subcmd"); },
            _ => panic!("not an accepted subcommand"), // replace with something else maybe
        }
    }
}

// impl CmdOptionsConfig {
//     fn get_subcmd(&self) -> &str {
//         
//     }
// }

#[repr(u8)]
#[derive(Debug, PartialEq)]
enum SubcmdOptions {
    File,
    Directory,
    Tag,
}

impl std::convert::From<OptNum> for SubcmdOptions {
    fn from(value: OptNum) -> Self {
        unsafe { std::mem::transmute_copy(&value) }
    }
}

impl SubcmdOptions {
    const fn to(self) -> OptNum {
        self as OptNum
    }
}

#[derive(Debug, Default)]
pub struct SubcmdOptionsConfig {
    flag_help: bool,
    flag_version: bool,
    //
    pub subcmd_delete: bool,
    pub subcmd_insert: bool,
    pub subcmd_query: bool,
    pub subcmd_update: bool,
    //
    pub flag_dir: String,
    pub flag_file: String,
    pub flag_query: String,
    pub flag_tag: String,
    //
    subcmd_params: Vec<String>,
}

impl HelpVersion for SubcmdOptionsConfig {
    fn is_help(&self) -> bool {
        self.flag_help
    }
    fn is_version(&self) -> bool {
        self.flag_version
    }
}

fn version_mess(program: &str) -> String {
    format!("{} {} ({program})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

// addon to with subcmds later
fn usage_mess(_program: &str) -> String {
    format!("Usage:\n {} {}", env!("CARGO_PKG_NAME"), "[options]")
}

fn help_mess(program: &str) -> String {
    [&version_mess(program), "", &usage_mess(program), DESCRIPTION_TEXT, OPTIONS_TEXT, SUBCOMMANDS_TEXT, /* EXAMPLES_TEXT */].join("\n")
}

fn print_help(conf: &CmdOptionsConfig) {
    print!("{}", help_mess(&conf.program_name));
    std::process::exit(0);
}

fn print_version(conf: &CmdOptionsConfig) {
    print!("{}", version_mess(&conf.program_name));
    std::process::exit(0);
}

// evaluates the option argument
fn value_to_string(nv: &NameVal<'_>) -> Result<String, OptParseError> {
    match nv.val {
        Some(val) => Ok(val.to_string()),
        None => {
            Err(OptParseError::missing_option_argument(&(nv.opt.sho as char).to_string()))
        }
    }
}

fn parse_option_match(conf: &mut CmdOptionsConfig, nv: &NameVal<'_>) -> Result<(), OptParseError> {
    match CmdOptions::from(nv.opt.num) {
        CmdOptions::Help => print_help(conf),
        CmdOptions::Version => print_version(conf),
        _ => {}
    }
    Ok(())
}

fn parse_subcmd_option_match(conf: &mut SubcmdOptionsConfig, nv: &NameVal<'_>) -> Result<(), OptParseError> {
    match SubcmdOptions::from(nv.opt.num) {
        SubcmdOptions::File => conf.flag_file = value_to_string(nv)?,
        _ if conf.subcmd_query => { println!("query value: {nv:?}"); conf.flag_query = value_to_string(nv)? },
        _ => {}
    }
    Ok(())
}

fn parse_cmdopts(program: &str, args: Vec<&str>) -> Result<CmdOptionsConfig, OptParseError> {
    let mut conf = CmdOptionsConfig {
        program_name: program.to_string(),
        ..Default::default()
    };
    let (free_opt, err_r) = parse_simple_gnu_style_subcmd(
        &mut conf,
        &OPTIONS,
        &OPTIONS_SHORT_INDEX,
        &args,
        parse_option_match,
        &SUBCOMMANDS
        );

    // if let Err(err) = err_r {
    //     return Err(err);
    // }
    if let Some(free) = free_opt {
        if !free.is_empty() {
            // parse subcmd options
            let free_args = free.iter().map(std::string::String::as_str).collect::<Vec<_>>();
            let (free_opt, err_r) = parse_simple_gnu_style(
                &mut conf.subcmd_conf,
                &SUBCOMMAND_OPTIONS,
                &SUBCOMMAND_OPTIONS_SHORT_INDEX,
                &free_args.as_slice(),
                parse_subcmd_option_match
                );

            if let Some(free) = free_opt {
                if !free.is_empty() {
                    println!("free args: {free:?}");
                    conf.subcmd_conf.subcmd_params = free.clone();
                    conf.program_params = free;

                }
            }
        }
    }

    Ok(conf)
}

pub fn create_cli_conf() -> Result<CmdOptionsConfig, OptParseError> {
    let mut args: Vec<String> = std::env::args().collect();
    let program = args.remove(0);
    let args: Vec<&str> = args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, args)
}

pub fn test_run(conf: &CmdOptionsConfig) {
    let dispatcher = EventDispatcher::new();
    if conf.subcmd_conf.subcmd_insert {

    }
}
