extern crate execute;

use std::process::Stdio;
use std::process::Command;
use std::ptr::{null, null_mut};
use execute::{Execute, command, shell};

/// Class to make it easy to run shell commands.
///
/// # Examples
///
/// ```
/// use crate::runcmd::RunCmd;
///
/// RunCmd::new("echo \"Hello World\"").execute();
///
/// ```
#[derive(Clone)]
pub struct RunCmdOutput {
    pub cmd: String,
    pub stdout: String,
    pub stderr: String,
    pub exitcode: i32
}

pub struct RunCmd {
    retval: RunCmdOutput,
    verbose: bool,
    execute: bool,
    shell: bool
}

impl RunCmd {

    pub fn new(cmd: &str) -> RunCmd {
        RunCmd {
            retval: RunCmdOutput {
                cmd: String::from(cmd),
                stdout: String::from(""),
                stderr: String::from(""),
                exitcode: 0
            },
            verbose: false,
            execute: false,
            shell: false
        }
    }

    /// Explicitly prints out stdout, stderr, and the exit code for the command run.
    /// But it disables real time output
    #[allow(dead_code)]
    pub fn verbose(&mut self) -> &mut RunCmd {
        self.verbose = true;
        self
    }

    /// Forces the command to run in a system shell.  Can fix some issue with complex commands.
    #[allow(dead_code)]
    pub fn shell(&mut self) -> &mut RunCmd {
        self.shell = true;
        self
    }

    fn print(&self) {
        println!("cmd:\n '{}'\n", self.retval.cmd);
        println!("stdout:\n '{}'\n", self.retval.stdout);
        println!("stderr:\n '{}'\n", self.retval.stderr);
        println!("exitcode: '{}'\n\n", self.retval.exitcode);
    }

    /// Standard execution.  If it doesn't succeed it will just panic.
    pub fn execute(&mut self) {
        self.execute = true;

        let retval = self.execute_output();

        if retval.exitcode != 0 {
            panic!("Exitcode != 0")
        }
    }

    /// Execution returning a structure with the output: exitcode, stdout, stderr.
    pub fn execute_output(&mut self) -> RunCmdOutput {
        let mut executor;

        if self.shell {
            executor = shell(&self.retval.cmd)
        } else {
            executor = command(&self.retval.cmd)
        }

        if self.verbose || !self.execute {
            executor.stdout(Stdio::piped());
            executor.stderr(Stdio::piped());
        }

        let output = executor.execute_output().unwrap();

        if let Some(exit_code) = output.status.code() {
            self.retval.exitcode = exit_code;
            self.retval.stdout =  String::from_utf8(output.stdout).unwrap();
            self.retval.stderr =  String::from_utf8(output.stderr).unwrap();
        } else {
            self.retval.exitcode = -1;
            self.retval.stderr =  String::from("Interrupted! in RunCmd");
        }

        if self.verbose {
            self.print();
        }

        return self.retval.clone()
    }

    pub fn interactive(&mut self) {
       if self != "interactive" {
           return;
       }
        loop {
            let v = String::from("Install Custom Kernel to enable Reclaim? [Y/n]");
            if v.to_uppercase() == "Y" || v.to_uppercase() == "YES" {
                break;
            }
            if v.to_uppercase() == "N" || v.to_uppercase() == "NO" {
                print!("Skipping Kernel package install.");
                break;
            }
            ///println!("Sorry didn't understand - ", v);
            println!("Enter Y or n");
        }

        let mut line = String::new();
        println!("Enter your provided license key or n to skip [<license>/N]: ");
        let mut v = std::io::stdin().read_line(&mut line).unwrap();
        if v.to_string().to_uppercase() == "N" || v.to_string().to_uppercase() == "NO" {
            self.retval.stdout = False;
            print!("Skipping license key setup. See user manual to change this setting.");
        }
        else {
            self.retval.cmd = v.to_string();
        }

        let mut line = String::new();
        println!("Enter your device id [<name to identify this device>/N]: ");
        v = std::io::stdin().read_line(&mut line).unwrap();
        if v.to_string().to_uppercase() == "N" || v.to_string().to_uppercase() == "NO" {
            self.retval.stdout = False;
            print!("Skipping setting of device id, see user manual to change this setting.");
        }
        else {
            self.retval.cmd = v.to_string();
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_pass() {
        RunCmd::new("bash -c \"exit 0\"").execute();
    }

    #[test]
    #[should_panic]
    fn execute_fail() {
        RunCmd::new("bash -c \"exit -1\"").execute();
    }

    #[test]
    fn execute_verbose() {
        RunCmd::new("echo bar; exit 0")
            .verbose()
            .execute();
    }

    #[test]
    fn execute_shell() {
        RunCmd::new("echo foobar; exit 0").shell().execute();
    }

    #[test]
    fn execute_output_pass() {
        let retval = RunCmd::new("bash -c \"echo foo; >&2 echo bar; exit -1\"").execute_output();
        assert_eq!(retval.exitcode, 255);
        assert_eq!(&retval.stdout, "foo\n");
        assert_eq!(&retval.stderr, "bar\n");
        assert_eq!(&retval.cmd, "bash -c \"echo foo; >&2 echo bar; exit -1\"");
    }

    #[test]
    fn execute_output_shell_pass() {
        let retval = RunCmd::new("echo foo; >&2 echo bar; exit -1").shell().execute_output();
        assert_eq!(retval.exitcode, 255);
        assert_eq!(&retval.stdout, "foo\n");
        assert_eq!(&retval.stderr, "bar\n");
        assert_eq!(&retval.cmd, "echo foo; >&2 echo bar; exit -1");
    }

}