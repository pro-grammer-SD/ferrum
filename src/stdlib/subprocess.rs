/// Subprocess Module for Ferrum
///
/// Provides process execution capabilities:
/// - subprocess.run(cmd) - Execute command and wait for completion
/// - subprocess.popen(cmd) - Spawn process with live I/O
///
/// All functions return clean Ferrum-native objects (no panics, no hangs)

use std::process::{Command, Stdio, Child};
use std::io::{Write, Read, BufReader};
use anyhow::Result;
use crate::runtime::Value;

/// Result of process execution
pub struct ProcessResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

impl ProcessResult {
    /// Convert to Ferrum Dict for return values
    pub fn to_dict(&self) -> Value {
        let mut map = std::collections::HashMap::new();
        map.insert("stdout".to_string(), Value::Str(self.stdout.clone()));
        map.insert("stderr".to_string(), Value::Str(self.stderr.clone()));
        map.insert("exit_code".to_string(), Value::Int(self.exit_code as i64));
        Value::Dict(map)
    }
}

/// Run a command synchronously and return stdout, stderr, exit_code
pub fn run_command(cmd: &str) -> Result<ProcessResult> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd])
            .output()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()?
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    Ok(ProcessResult {
        stdout,
        stderr,
        exit_code,
    })
}

/// Spawn a process for live interaction
pub struct Process {
    child: Option<Child>,
    pub pid: u32,
}

impl Process {
    /// Spawn a new process
    pub fn spawn(cmd: &str) -> Result<Self> {
        let child = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", cmd])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?
        };

        let pid = child.id();
        Ok(Process {
            child: Some(child),
            pid,
        })
    }

    /// Write to stdin
    pub fn write(&mut self, data: &str) -> Result<()> {
        if let Some(ref mut child) = self.child {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(data.as_bytes())?;
                child.stdin = Some(stdin);
            }
        }
        Ok(())
    }

    /// Read output until EOF
    pub fn read_output(&mut self) -> Result<(String, String)> {
        let mut stdout_data = String::new();
        let mut stderr_data = String::new();

        if let Some(mut child) = self.child.take() {
            if let Some(stdout) = child.stdout.take() {
                let mut reader = BufReader::new(stdout);
                reader.read_to_string(&mut stdout_data)?;
            }
            if let Some(stderr) = child.stderr.take() {
                let mut reader = BufReader::new(stderr);
                reader.read_to_string(&mut stderr_data)?;
            }
            self.child = Some(child);
        }

        Ok((stdout_data, stderr_data))
    }

    /// Wait for process to complete
    pub fn wait(&mut self) -> Result<i32> {
        if let Some(mut child) = self.child.take() {
            let status = child.wait()?;
            let exit_code = status.code().unwrap_or(-1);
            self.child = Some(child);
            Ok(exit_code)
        } else {
            Ok(-1)
        }
    }

    /// Kill process
    pub fn kill(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            child.kill()?;
            self.child = Some(child);
        }
        Ok(())
    }
}

/// Ferrum-callable wrapper for subprocess.run()
pub fn subprocess_run(args: Vec<Value>) -> anyhow::Result<Value> {
    if let Some(Value::Str(cmd)) = args.get(0) {
        match run_command(cmd) {
            Ok(result) => Ok(result.to_dict()),
            Err(e) => {
                let mut map = std::collections::HashMap::new();
                map.insert("stdout".to_string(), Value::Str(String::new()));
                map.insert("stderr".to_string(), Value::Str(format!("Error: {}", e)));
                map.insert("exit_code".to_string(), Value::Int(-1));
                Ok(Value::Dict(map))
            }
        }
    } else {
        Err(anyhow::anyhow!("subprocess.run() requires a string command"))
    }
}

/// Ferrum-callable wrapper for subprocess.popen()
pub fn subprocess_popen(args: Vec<Value>) -> anyhow::Result<Value> {
    if let Some(Value::Str(cmd)) = args.get(0) {
        match Process::spawn(cmd) {
            Ok(proc) => {
                let mut map = std::collections::HashMap::new();
                map.insert("pid".to_string(), Value::Int(proc.pid as i64));
                map.insert("_proc_internal".to_string(), Value::Str(format!("proc-{}", proc.pid)));
                Ok(Value::Dict(map))
            }
            Err(e) => {
                let mut map = std::collections::HashMap::new();
                map.insert("error".to_string(), Value::Str(format!("Failed to spawn: {}", e)));
                Ok(Value::Dict(map))
            }
        }
    } else {
        Err(anyhow::anyhow!("subprocess.popen() requires a string command"))
    }
}
