/*
 * @Author: timochan
 * @Date: 2023-03-20 14:40:29
 * @LastEditors: timochan
 * @LastEditTime: 2023-05-21 10:35:50
 * @FilePath: /catwrt-update/src/lib.rs
 */
use std::collections::HashMap;
use std::error::Error;
use std::{process,fs,env};

const API_URL: &str = "https://api.miaoer.xyz/api/v2/snippets/catwrt/check_update";
const VERSION_FILE: &str = "/etc/catwrt-release";

#[derive(Debug)]
pub struct Local {
    pub arch: String,
    pub version: String,
    pub hash: String,
}
impl Local {
    pub fn new() -> Result<Local, Box<dyn Error>> {
        let arch = get_local_arch();
        let version = get_local_version()?;
        let hash = get_local_hash()?;
        Ok(Local {
            arch,
            version,
            hash,
        })
    }
}
fn get_local_arch() -> String {
    let arch = env::consts::ARCH;
    let arch = match arch {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        "mips" => "mips",
        _ => "unknown",
    };
    arch.to_string()
}
fn get_local_version() -> Result<String, Box<dyn Error>> {
    let os_release = fs::read_to_string(VERSION_FILE).map_err(|e| {
        eprintln!("Error reading file: {}", e);
        e
    })?;
    let version = os_release
        .lines()
        .find(|line| line.starts_with("version="))
        .map(|line| {
            line.trim_start_matches("version=")
                .trim_matches('"')
                .to_string()
        })
        .ok_or_else(|| "version not found in file".to_string())?;
    Ok(version)
}
fn get_local_hash() -> Result<String, Box<dyn Error>> {
    let os_release = fs::read_to_string(VERSION_FILE).map_err(|e| {
        eprintln!("Error reading file: {}", e);
        e
    })?;
    let hash = os_release
        .lines()
        .find(|line| line.starts_with("hash="))
        .map(|line| {
            line.trim_start_matches("hash=")
                .trim_matches('"')
                .to_string()
        })
        .ok_or_else(|| "hash not found in file".to_string())?;
    Ok(hash)
}
#[derive(Debug, serde::Deserialize)]
pub struct ApiResponse {
    pub version: String,
    pub hash: String,
}
impl ApiResponse {
    pub fn new(arch: &str) -> Result<ApiResponse, Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            "catwrt-update/0.1.0".parse().unwrap(),
        );
        let response = client
            .get(API_URL)
            .headers(headers)
            .send()
            .map_err(|e| {
                eprintln!("Error sending request: {}", e);
                e
            })?
            .json::<HashMap<String, String>>()
            .map_err(|e| {
                eprintln!("Error parsing response: {}", e);
                e
            })?;

        let version = response
            .get("version")
            .ok_or_else(|| "API response does not contain version field")?
            .to_owned();

        let hash_key = match arch {
            "amd64" => "hash_amd64",
            "arm64" => "hash_arm",
            "mips" => "hash_wireless_mt7986a",
            _ => {
                eprintln!("This arch is not supported!");
                process::exit(1);
            }
        };
        let hash = response
            .get(hash_key)
            .ok_or_else(|| "API response does not contain hash field")?
            .to_owned();

        Ok(ApiResponse { version, hash })
    }
}
