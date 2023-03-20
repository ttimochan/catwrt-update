/*
 * @Author: timochan
 * @Date: 2023-03-20 14:37:19
 * @LastEditors: timochan
 * @LastEditTime: 2023-03-20 16:01:45
 * @FilePath: /catwrt-update/src/main.rs
 */
use catwrt_update::{Local,ApiResponse};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let local = Local::new()?;
    println!("OK , get local info!");
    println!("arch: {}", local.arch);
    println!("version: {}", local.version);
    
    let api = ApiResponse::new()?;
    if local.version == api.version && local.hash == api.hash{
        println!("You are using the latest version!");
    } else {
        println!("You are using an old version!");
        println!("The latest version is: {}", api.version);
    }
    Ok(())
}
