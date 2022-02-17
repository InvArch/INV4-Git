use std::{error::Error, path::Path, process::Command};

use crate::error::ErrorWrap;

pub fn _create_bundle(bundle: &Path, ref_name: &str) -> Result<(), Box<dyn Error>> {
    let cmd = Command::new("git")
        .args([
            "bundle",
            "create",
            bundle.to_str().ok_or(ErrorWrap("Invalid bundle path"))?,
            ref_name,
        ])
        .output()?;
    if !cmd.status.success() {
        Err(ErrorWrap("Git bundle failed").into())
    } else {
        Ok(())
    }
}

pub fn _unbundle(bundle: &Path, ref_name: &str) -> Result<(), Box<dyn Error>> {
    let cmd = Command::new("git")
        .args([
            "bundle",
            "create",
            bundle.to_str().ok_or(ErrorWrap("Invalid bundle path"))?,
            ref_name,
        ])
        .output()?;
    if !cmd.status.success() {
        Err(ErrorWrap("Git unbundle failed").into())
    } else {
        Ok(())
    }
}

pub fn _is_ancestor(base_ref: &str, remote_ref: &str) -> Result<bool, Box<dyn Error>> {
    let cmd = Command::new("git")
        .args(["merge-base", "--is-ancestor", remote_ref, base_ref])
        .output()?;
    Ok(cmd.status.success())
}

pub fn _config(setting: &str) -> Result<String, Box<dyn Error>> {
    let cmd = Command::new("git").args(["config", setting]).output()?;
    if !cmd.status.success() {
        Err(ErrorWrap("Git config failed").into())
    } else {
        Ok(String::from_utf8(cmd.stdout)?.trim().to_owned())
    }
}

pub fn _rev_parse(rev: &str) -> Result<String, Box<dyn Error>> {
    let cmd = Command::new("git").args(["rev-parse", rev]).output()?;
    if !cmd.status.success() {
        Err(ErrorWrap("Git rev-parse failed").into())
    } else {
        Ok(String::from_utf8(cmd.stdout)?.trim().to_owned())
    }
}
