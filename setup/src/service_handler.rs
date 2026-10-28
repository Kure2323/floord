use crate::setup_args;
use anyhow::{Context, Result};
use service_manager::{ServiceInstallCtx, ServiceLabel, ServiceLevel, ServiceManager, ServiceStartCtx, ServiceStatus, ServiceStatusCtx, ServiceStopCtx, ServiceUninstallCtx};
use std::env::consts::OS;
use std::env::current_exe;
use std::ffi::OsString;

pub fn run(arguments: setup_args::Args) -> Result<()> {
    let label: ServiceLabel = "service.daemon.setup".parse()?;

    let mut native_manager = <dyn ServiceManager>::native().context("getting native service setup")?;

    if OS != "windows" { // Windows does not support user service level
        native_manager.set_level(ServiceLevel::User).context("making service user-level")?;
    }

    let executable_path = current_exe()?.parent().unwrap().join("floord");
    let mut args: Vec<OsString> = Vec::new();

    if let Some(inbox_path) = arguments.inbox {
        args.push("--inbox".into());
        args.push(inbox_path.into());
    }

    args.push("--delay".into());
    args.push(arguments.delay.to_string().into());

    let service = ServiceInstallCtx {
        label: label.clone(),
        program: executable_path,
        args,
        contents: None,
        username: None,
        working_directory: None,
        environment: None,
        autostart: true,
        restart_policy: Default::default(),
    };

    let status = native_manager.status(ServiceStatusCtx { label: label.clone() })
        .context("retrieving service status")?;

    match status {
        ServiceStatus::NotInstalled => {
            if !arguments.uninstall {
                native_manager.install(service).context("installing service")?;
                println!("Floord service installed correctly")
            }
        }
        ServiceStatus::Running => {
            if arguments.uninstall {
                native_manager.stop(ServiceStopCtx { label: label.clone() }).context("stopping service")?;
                native_manager.uninstall(ServiceUninstallCtx { label }).context("Maybe it is already uninstalled, try rebooting")?;
                println!("Uninstalling and stopping process completed");
                return Ok(())
            }
            println!("Floord service already running!");
            return Ok(())
        }
        ServiceStatus::Stopped(_) => {}
    }

    if arguments.uninstall {
        native_manager.uninstall(ServiceUninstallCtx { label }).context("probably already uninstalled")?;
        println!("Uninstalling process completed");
        return Ok(())
    }

    native_manager.start(ServiceStartCtx {
        label
    }).context("stating service")?;
    println!("Floord service started");

    Ok(())
}