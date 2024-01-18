use std::{env, fmt::Display, str::FromStr};

use anyhow::Error;
use log::{info, warn};

pub const SETTING_HOST_MODE: &str = "host_mode";

pub const SETTING_HOST_ENVIRONMENT: &str = "host_environment";
pub const ENV_HOST_ENVIRONMENT: &str = "HOST_ENVIRONMENT";

#[derive(Clone, Copy, PartialEq, strum_macros::EnumString, strum_macros::Display)]
pub enum Environment {
    Production,
    Development,
}

#[derive(strum_macros::EnumString, strum_macros::Display)]
pub enum HostMode {
    Production,
    Development,
}

pub fn is_dev_mode() -> bool {
    if cfg!(debug_assertions) {
        return true;
    }
    false
}

pub fn is_prod_mode() -> bool {
    !is_dev_mode()
}

pub fn get_host_mode() -> HostMode {
    if is_dev_mode() {
        return HostMode::Development;
    }
    HostMode::Production
}

pub enum HostType {
    Local,
    Container,
}

pub fn is_docker_host() -> bool {
    let process_id_1 = procfs::process::all_processes()
        .expect("Can't read /proc")
        .find(|proc| {
            let stats = proc.as_ref().unwrap().stat().unwrap();
            stats.pid == 1 && !stats.comm.contains("sh")
        });

    if let Some(proc) = process_id_1 {
        let process = proc.unwrap();
        let stats = process.stat().unwrap();
        info!("Running locally. {} {}", stats.pid, stats.comm);
        false
    } else {
        info!("Running in a container.");
        true
    }
}

pub fn is_local_host() -> bool {
    !is_docker_host()
}

pub fn get_host_type() -> HostType {
    if is_docker_host() {
        return HostType::Container;
    }
    HostType::Local
}

pub fn init_environment() {
    if is_dev_mode() {
        info!("Running in development mode.");
        init_dev_environment();
    } else if is_prod_mode() {
        info!("Running in production mode.");
        init_prod_environment();
    }
}

pub fn init_dev_environment() {
    if is_local_host() {
        info!("Loading .env file");
        dotenvy::from_path(".env").expect("No .env file found!");
    } else {
        info!("Will not look for .env file")
    }

    let env = env::var("HOST_ENVIRONMENT")
        .expect("Unknown environment! Environment variable HOST_ENVIRONMENT must be set!");
    match env.to_lowercase().as_str() {
        "dev" => Environment::Development,
        "development" => Environment::Development,
        "prod" => Environment::Production,
        "production" => Environment::Production,
        val => panic!(
            "Unknown value \"{}\" for environment variable HOST_ENVIRONMENT!",
            val
        ),
    };
    info!(
        "Runnign like {} environment (determined by HOST_ENVIRONMENT)",
        env
    );
}

pub fn init_prod_environment() {
    if is_local_host() {
        info!("Will not look for .env file");
    } else {
        info!("Will not look for .env file");
    }
}

pub fn get_env_var<VarType>(name: &str, default: Option<VarType>) -> Result<VarType, Error>
where
    VarType: FromStr + Display,
    <VarType as FromStr>::Err: std::fmt::Debug,
{
    let result: VarType;
    if let Some(value) = default {
        result = env::var(name)
            .unwrap_or_else(|_| {
                warn!(
                    "Environment variable {} is not defined. Defaulting to {}",
                    name, value
                );
                value.to_string()
            })
            .parse::<VarType>()
            .unwrap_or(value);
        Ok(result)
    } else {
        let result = env::var(name)
            .unwrap_or_else(|_| {
                panic!(
                    "Environment variable {} is not defined, and a default value is not assigned",
                    name
                )
            })
            .parse()
            .unwrap_or_else(|_| {
                panic!(
                    "Environment variable {} is defined but it's type cannot be read.",
                    name
                )
            });
        Ok(result)
    }
}
