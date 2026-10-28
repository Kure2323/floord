use floord::daemon_run;

fn main() -> anyhow::Result<()>{
    daemon_run()?;
    Ok(())
}
