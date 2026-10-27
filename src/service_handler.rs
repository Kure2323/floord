use std::env::current_exe;
use std::ffi::OsString;
use anyhow::{Context, Result};
use service_manager::{ServiceInstallCtx, ServiceLabel, ServiceLevel, ServiceManager, ServiceStartCtx, ServiceStatus, ServiceStatusCtx, ServiceStopCtx, ServiceUninstallCtx};
use crate::floord_args;

pub fn service_handler(arguments: floord_args::Args) -> Result<()> {

    let label: ServiceLabel = "service.daemon.floord".parse()?;

    let mut native_manager = <dyn ServiceManager>::native().context("getting native service manager")?;
    native_manager.set_level(ServiceLevel::User).context("making service user-level")?;

    let executable_path = current_exe().context("getting executable path")?;
    let mut args: Vec<OsString> = Vec::new();

    if let Some(inbox_path) = arguments.inbox {
        args.push("--inbox".into());
        args.push(inbox_path.into());
    }

    args.push("--delay".into());
    args.push(arguments.delay.to_string().into());
    args.push("--run".into());

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

    let status = native_manager.status(ServiceStatusCtx {label: label.clone()})
        .context("retrieving service status")?;

    match status {
        ServiceStatus::NotInstalled => {
            native_manager.install(service).context("installing service floord")?;
            println!("Floord service installed correctly")
        }
        ServiceStatus::Running => {
            if arguments.uninstall {
                native_manager.stop(ServiceStopCtx {label: label.clone()}).context("stopping service")?;
                native_manager.uninstall(ServiceUninstallCtx {label}).context("Maybe it is already uninstalled, try rebooting")?;
                println!("Uninstalling and stopping process completed");
                return Ok(())
            }
            println!("Floord service already running!");
            return Ok(())
        }
        ServiceStatus::Stopped(_) => {}
    }

    if arguments.uninstall {
        native_manager.uninstall(ServiceUninstallCtx {label}).context("Maybe it is already uninstalled, try rebooting")?;
        println!("Uninstalling process completed");
        return Ok(())
    }

    native_manager.start(ServiceStartCtx {
        label
    }).context("stating service")?;
    println!("Floord service started");

    Ok(())
}