pub struct CrashTestHarness;
impl CrashTestHarness {
    pub fn simulate_power_loss() { std::process::exit(1); }
    pub fn inject_fsync_failure() -> std::io::Result<()> {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "simulated fsync failure"))
    }
}
