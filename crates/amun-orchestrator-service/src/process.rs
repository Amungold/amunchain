use amun_orchestrator_core::error::OrchestratorError;
use std::process::{Child, Command, Stdio};

pub struct ProcessHandle {
    pub name: String,
    pub pid: u32,
    child: Option<Child>,
}

impl ProcessHandle {
    pub async fn spawn(
        name: &str,
        command: &str,
        args: &[String],
    ) -> Result<Self, OrchestratorError> {
        let child = Command::new(command)
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::null())
            .spawn()
            .map_err(|e| OrchestratorError::Process {
                command: command.to_string(),
                message: e.to_string(),
            })?;

        let pid = child.id();

        Ok(Self {
            name: name.to_string(),
            pid,
            child: Some(child),
        })
    }

    pub async fn is_alive(&self) -> bool {
        unsafe { libc::kill(self.pid as i32, 0) == 0 }
    }

    pub async fn kill(&mut self) -> Result<(), OrchestratorError> {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        Ok(())
    }
}
