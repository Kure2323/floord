use anyhow::Result;
use floord::floord_run;


fn main() -> Result<()> {
    floord_run()?;
    Ok(())
}