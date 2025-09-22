use std::process::Command;

pub struct CompilerInvoker;

impl CompilerInvoker {
    pub fn new() -> Self {
        Self
    }

    pub fn invoke(&self, args: &[String]) -> Result<std::process::ExitStatus, std::io::Error> {
        if args.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No compiler arguments provided"
            ));
        }

        let mut cmd = Command::new(&args[0]);
        cmd.args(&args[1..]);

        cmd.status()
    }
}