mod logger;
mod debugger;

extern crate slog_stdlog;

use debugger::Debugger;
use slog_scope;
use clap::{Command, Arg, ArgMatches};

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    
    let root_logger = logger::init_logger();

    let _guard = slog_scope::set_global_logger(root_logger);
    let _shutdown_logger = logger::ShutdownLogger;  

    slog_stdlog::init().unwrap();

    slog_scope::info!("DevToolchain started"; "status" => "running");

    let matches = Command::new("sllvm")
        .version("0.0")
        .author("Caleb L'Italien")
        .about("Toolchain for development and debugging SafeLLVM")
        .subcommand(Command::new("debug")
            .about("Debugging tools")
            .arg(Arg::new("tool")
                .short('t')
                .long("tool")
                .takes_value(true)
                .help("Specify the debugger tool: gdb or lldb"))
            .arg(Arg::new("executable")
                .short('e')
                .long("executable")
                .takes_value(true)
                .help("Path to the executable to debug"))
            .arg(Arg::new("file_path")
                .short('f')
                .long("file_path")
                .takes_value(true)
                .help("Specify the file path to target for breakpoints"))
            .arg(Arg::new("function")
                .short('F')
                .long("function")
                .takes_value(true)
                .help("Specify the function to target for breakpoints"))
            .arg(Arg::new("line")
                .short('l')
                .long("line")
                .takes_value(true)
                .help("Specify the line number to set a breakpoint"))
            .arg(Arg::new("all-lines")
                .short('a')
                .long("all-lines")
                .takes_value(false)
                .help("Set a breakpoint on every line of the specified file or function")))
        .get_matches();

    match_high_level_command(&matches);
}

fn match_high_level_command(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("debug", sub_m)) => handle_debugger(sub_m),
        Some(("performance", sub_m)) => handle_performance(sub_m),
        Some(("sanitize", sub_m)) => handle_sanitizer(sub_m),
        Some(("valgrind", sub_m)) => handle_valgrind(sub_m),
        _ => slog_scope::error!("No valid subcommand was provided."),
    }
}

fn handle_debugger(matches: &ArgMatches) {
    let tool = if let Some(t) = matches.value_of("tool") {
        t
    } else {
        slog_scope::error!("Debugger tool not specified");
        return;  
    };

    let executable = if let Some(e) = matches.value_of("executable") {
        e
    } else {
        slog_scope::error!("Executable path is required for debugging");
        return; 
    };

    let debugger = Debugger::new(tool, executable);
    debugger.setup();

    loop {
        slog_scope::info!("Enter command ('quit' to exit, 'run' to start debugger):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim().split_whitespace().collect::<Vec<_>>();
        let command = input.first().map(|&arg| arg);  
        
        match command {
            Some("quit") => break,
            Some("run") => {
                debugger.start();
                break;
            },
            Some("all-lines") => {
                if let Some(file_path) = matches.value_of("file_path") {
                    debugger.set_breakpoints_every_line_of_file(file_path);
                } else if let Some(function) = matches.value_of("function") {
                    debugger.set_breakpoint_every_line_of_function(function);
                } else {
                    slog_scope::error!("'all-lines' requires either a file path or a function name");
                }
            },
            Some("line") => {
                if let Some(file_path) = matches.value_of("file_path") {
                    if let Some(line_str) = matches.value_of("line") {
                        match line_str.parse::<usize>() {
                            Ok(line) => debugger.set_breakpoint_on_line(file_path, line),
                            Err(_) => slog_scope::error!("Invalid line number format"),
                        }
                    } else {
                        slog_scope::error!("Specifying a line requires a line number");
                    }
                } else {
                    slog_scope::error!("Specifying a line requires a file path");
                }
            },
            Some(cmd) => slog_scope::error!("Unknown command '{}'", cmd),
            None => slog_scope::error!("No command entered"),
        }
    }
}



fn handle_performance(matches: &ArgMatches) {
    match matches.value_of("type") {
        Some(tool_type) => slog_scope::info!("Running performance tool"; "tool_type" => tool_type),
        None => slog_scope::error!("Performance tool type not specified"),
    }
}

fn handle_sanitizer(matches: &ArgMatches) {
    match matches.value_of("type") {
        Some(sanitizer_type) => slog_scope::info!("Running sanitizer"; "sanitizer_type" => sanitizer_type),
        None => slog_scope::error!("Sanitizer type not specified"),
    }
}

fn handle_valgrind(matches: &ArgMatches) {
    match matches.value_of("tool") {
        Some(tool) => slog_scope::info!("Running Valgrind tool"; "tool" => tool),
        None => slog_scope::error!("Valgrind tool not specified"),
    }
}
