/*
 * @Author: timochan
 * @Date: 2023-03-20 14:40:29
 * @LastEditors: timochan
 * @LastEditTime: 2023-03-20 16:04:37
 * @FilePath: /catwrt-update/src/lib.rs
 */
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub struct Local {
    pub arch: String,
    pub version: String,
    pub hash: String,
}
impl Local {
    pub fn new() -> Result<Local, Box<dyn Error>> {
        let arch = get_arch();
        let version = get_version();
        let hash = get_hash();
        Ok(Local {
            arch,
            version,
            hash,
        })
    }
}

fn get_arch() -> String {
    let arch = env::consts::ARCH;
    let arch = match arch {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        _ => "unknown",
    };
    arch.to_string()
}
fn get_version() -> String {
    let os_release = fs::read_to_string("/etc/catwrt-release").unwrap_or_else(|_| {
        eprintln!("This file not found! Please check your system!");
        process::exit(1)
    });
    let os_release = os_release.split("\n").collect::<Vec<&str>>();
    let mut version = String::new();
    for line in os_release {
        if line.starts_with("version=") {
            version = line.replace("version=", "");
            version = version.replace("\"", "");
            break;
        }
    }
    version.to_string()
}
fn get_hash() -> String {
    let os_release = fs::read_to_string("/etc/catwrt-release").unwrap_or_else(|err| {
        eprintln!("This file not found! {}", err);
        process::exit(1)
    });
    let os_release = os_release.split("\n").collect::<Vec<&str>>();
    let mut hash = String::new();
    for line in os_release {
        if line.starts_with("hash=") {
            hash = line.replace("hash=", "");
            hash = hash.replace("\"", "");
            break;
        }
    }
    hash.to_string()
}
#[derive(Debug, serde::Deserialize)]
pub struct ApiResponse {
    pub version: String,
    pub hash: String,
}
impl ApiResponse {
    pub fn new() -> Result<ApiResponse, Box<dyn Error>> {
        let arch = get_arch();
        let response = fetch_api_data(arch)?;
        Ok(ApiResponse {
            version: response.version,
            hash: response.hash,
        })
    }
}
fn fetch_api_data(arch: String) -> Result<ApiResponse, Box<dyn Error>> {
    let response =
        reqwest::blocking::get("https://api.miaoer.xyz/api/v2/snippets/catwrt/check_update")?
            .json::<HashMap<String, String>>()?;

    let version = response
        .get("version")
        .ok_or("API response does not contain version field")?
        .to_owned();
    let mut hash = String::new();

    if arch == "amd64" {
        hash = response
            .get("hash_amd64")
            .ok_or("API response does not contain hash field")?
            .to_owned();
    } else if arch == "arm64" {
        hash = response
            .get("hash_arm")
            .ok_or("API response does not contain hash field")?
            .to_owned();
    }

    Ok(ApiResponse { version, hash })
}
