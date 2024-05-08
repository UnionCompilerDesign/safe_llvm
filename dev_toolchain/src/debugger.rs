use slog_scope;

use std::process::{Command, Stdio};
use std::backtrace::Backtrace;

pub struct Debugger {
    tool: String,
    executable: String,
}

impl Debugger {
    pub fn new(tool: &str, executable: &str) -> Debugger {
        Debugger {
            tool: tool.to_string(),
            executable: executable.to_string(),
        }
    }

    pub fn setup(&self) {
        if self.check_debugger_installed() {
            match self.tool.as_str() {
                "gdb" => self.start_gdb(),
                "lldb" => self.start_lldb(),
                _ => slog_scope::error!("Unsupported debugger"; "tool" => &self.tool, "backtrace" => format!("{:?}", Backtrace::capture())),
            }
        } else {
            slog_scope::warn!("Debugger not installed, attempting to install"; "tool" => &self.tool, "backtrace" => format!("{:?}", Backtrace::capture()));
            if !self.install_debugger() {
                slog_scope::error!("Failed to install debugger"; "tool" => &self.tool, "backtrace" => format!("{:?}", Backtrace::capture()));
            }
        }
    }

    fn check_debugger_installed(&self) -> bool {
        match Command::new(&self.tool)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status() {
            Ok(status) if status.success() => true,
            Ok(_) => {
                slog_scope::error!("Debugger installed but cannot be executed properly"; "tool" => &self.tool, "backtrace" => format!("{:?}", Backtrace::capture()));
                false
            },
            Err(e) => {
                slog_scope::error!("Error checking if debugger is installed"; "tool" => &self.tool, "error" => e.to_string(), "backtrace" => format!("{:?}", Backtrace::capture()));
                false
            },
        }
    }

    fn install_debugger(&self) -> bool {
        match self.tool.as_str() {
            "gdb" => self.install("gdb"),
            "lldb" => self.install("lldb"),
            _ => {
                slog_scope::error!("No installation routine for the specified debugger"; "tool" => &self.tool, "backtrace" => format!("{:?}", Backtrace::capture()));
                false
            }
        }
    }

    fn install(&self, package: &str) -> bool {
        let command = match std::env::consts::OS {
            "linux" => Some(("sudo", vec!["apt-get", "install", "-y", package])),
            "macos" => Some(("brew", vec!["install", package])),
            _ => {
                slog_scope::error!("Unsupported operating system for automatic installation"; "os" => std::env::consts::OS, "backtrace" => format!("{:?}", Backtrace::capture()));
                None
            }
        };

        match command {
            Some((installer, args)) => {
                match Command::new(installer).args(&args).status() {
                    Ok(status) if status.success() => true,
                    Ok(_) => {
                        slog_scope::error!("Installation command ran but failed"; "package" => package, "backtrace" => format!("{:?}", Backtrace::capture()));
                        false
                    },
                    Err(e) => {
                        slog_scope::error!("Failed to execute installation command"; "package" => package, "error" => e.to_string(), "backtrace" => format!("{:?}", Backtrace::capture()));
                        false
                    }
                }
            },
            None => false
        }
    }

    fn start_gdb(&self) {
        match Command::new("gdb").arg(&self.executable).spawn() {
            Ok(child) => slog_scope::info!("GDB started successfully"; "pid" => child.id().to_string()),
            Err(e) => slog_scope::error!("Failed to start GDB"; "error" => e.to_string(), "backtrace" => format!("{:?}", Backtrace::capture())),
        }
    }

    fn start_lldb(&self) {
        match Command::new("lldb").arg(&self.executable).spawn() {
            Ok(child) => slog_scope::info!("LLDB started successfully"; "pid" => child.id().to_string()),
            Err(e) => slog_scope::error!("Failed to start LLDB"; "error" => e.to_string(), "backtrace" => format!("{:?}", Backtrace::capture())),
        }
    }

    pub fn start(&self) {
        let cmd = match self.tool.as_str() {
            "gdb" => format!("gdb {}", self.executable),
            "lldb" => format!("lldb -- {}", self.executable),
            _ => {
                slog_scope::error!("Unsupported debugger tool"; "tool" => &self.tool);
                return;
            }
        };
    
        match Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .spawn() {
            Ok(child) => slog_scope::info!("Debugger started successfully"; "pid" => child.id().to_string()),
            Err(e) => slog_scope::error!("Failed to start debugger"; "error" => e.to_string(), "backtrace" => format!("{:?}", Backtrace::capture())),
        }
    }

    pub fn set_breakpoints_every_line_of_file(&self, file_path: &str) {
        let command = format!("{} -ex 'break {}:1' -ex run -ex 'while 1; break' -ex 'end' {}", self.tool, file_path, self.executable);
        if Command::new("sh").arg("-c").arg(&command).status().is_ok() {
            slog_scope::info!("Breakpoints set on every line"; "file" => file_path);
        } else {
            slog_scope::error!("Failed to set breakpoints on every line"; "file" => file_path);
        }
    }

    pub fn set_breakpoint_on_line(&self, file_path: &str, line: usize) {
        let command = format!("{} -ex 'break {}:{}' -ex run {}", self.tool, file_path, line, self.executable);
        if Command::new("sh").arg("-c").arg(&command).status().is_ok() {
            slog_scope::info!("Breakpoint set"; "file" => file_path, "line" => line);
        } else {
            slog_scope::error!("Failed to set breakpoint"; "file" => file_path, "line" => line);
        }
    }
    

    pub fn set_breakpoint_every_line_of_function(&self, function_name: &str) {
        let command = format!("{} -ex 'break {}' -ex run {}", self.tool, function_name, self.executable);
        if Command::new("sh").arg("-c").arg(&command).status().is_ok() {
            slog_scope::info!("Breakpoints set in function"; "function" => function_name);
        } else {
            slog_scope::error!("Failed to set breakpoints in function"; "function" => function_name);
        }
    }
    
}
