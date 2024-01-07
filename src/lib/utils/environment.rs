use std::{env, fmt::Display, str::FromStr};

use anyhow::Error;
use log::{info, warn};

#[derive(Clone, Copy, PartialEq)]
pub enum Environment {
    Production,
    Development,
}

pub fn is_dev() -> bool {
    if cfg!(debug_assertions) {
        info!("Running in development mode.");
        return true;
    }
    info!("Running in production mode.");
    false
}

pub fn is_prod() -> bool {
    !is_dev()
}

pub fn get_environment() -> Environment {
    if is_dev() {
        return Environment::Development;
    }
    Environment::Production
}

pub enum HostType {
    Local,
    Container,
}

pub fn is_docker() -> bool {
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
        return false;
    } else {
        info!("Running in a container.");
        return true;
    }
}

pub fn is_local() -> bool {
    !is_docker()
}

pub fn get_host_type() -> HostType {
    if is_docker() {
        return HostType::Container;
    }
    HostType::Local
}

pub fn init_environment() {
    if is_dev() {
        init_dev();
    } else if is_prod() {
        init_prod();
    }
}

pub fn init_dev() {
    if is_local() {
        info!("Loading .env file");
        dotenvy::from_path(".env").expect("No .env file found!");
    } else {
        info!("Will not look for .env file")
    }

    let env = env::var("HOST_ENVIRONMENT")
        .expect("Unknown environment! Environment variable HOST_ENVIRONMENT must be set!");
    match env.as_str() {
        "dev" => Environment::Development,
        "prod" => Environment::Production,
        val => panic!(
            "Unknown value \"{}\" for environment variable HOST_ENVIRONMENT!",
            val
        ),
    };
    info!("Behaving like {} (determined by HOST_ENVIRONMENT)", env);
}

pub fn init_prod() {
    if is_local() {
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
