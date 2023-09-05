use retry::future::repeat::*;
use retry::prelude::*;

use std::path::{Path, PathBuf};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[tokio::main]
pub async fn main() -> Result<()> {
    somefunc.retry::<3>("Cargo.toml").unwrap();
    add.retry::<3>(1, 2).unwrap();
    add.repeat::<3>(1, 2).unwrap();
    hello.repeat::<3>();
    let mut val = 0;
    (|| {
        val += 1;
        println!("{val}")
    })
    .repeat::<3>();

    my_fun.repeat::<3>().await?;

    let async_closure = || async {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("Hello, world!");
    };
    async_closure.repeat::<3>().await;

    sleeper.repeat::<3>(3).await?;

    Ok(())
}

pub fn somefunc<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    Ok(std::fs::canonicalize(path)?)
}

pub fn add(a: i32, b: i32) -> Result<i32> {
    Ok(a + b)
}

pub fn hello() {
    println!("hello")
}

pub async fn my_fun() -> Result<()> {
    println!("Sleepy time");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(())
}

pub async fn sleeper(sleep: u64) -> Result<u64> {
    println!("Sleepy time for {sleep}");
    tokio::time::sleep(std::time::Duration::from_secs(sleep)).await;
    Ok(sleep)
}
