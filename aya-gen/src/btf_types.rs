use std::{io, path::Path, process::Command, str::from_utf8};

use thiserror::Error;

use crate::bindgen;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error executing bpftool")]
    BpfTool(#[source] io::Error),

    #[error("{stderr}\nbpftool failed with exit code {code}")]
    BpfToolExit { code: i32, stderr: String },

    #[error("bindgen failed")]
    Bindgen,

    #[error("rustfmt failed")]
    Rustfmt(#[source] io::Error),
}

pub fn generate<T: AsRef<str>>(btf_file: &Path, types: &[T]) -> Result<String, Error> {
    let mut bindgen = bindgen::bpf_builder();

    let c_header = c_header_from_btf(btf_file)?;
    bindgen = bindgen.header_contents("kernel_types.h", &c_header);

    for ty in types {
        bindgen = bindgen.allowlist_type(ty);
    }

    let bindings = bindgen.generate().or(Err(Error::Bindgen))?.to_string();

    Ok(bindings)
}

fn c_header_from_btf(path: &Path) -> Result<String, Error> {
    let output = Command::new("bpftool")
        .args(&["btf", "dump", "file"])
        .arg(path)
        .args(&["format", "c"])
        .output()
        .map_err(Error::BpfTool)?;

    if !output.status.success() {
        return Err(Error::BpfToolExit {
            code: output.status.code().unwrap(),
            stderr: from_utf8(&output.stderr).unwrap().to_owned(),
        });
    }

    Ok(from_utf8(&output.stdout).unwrap().to_owned())
}
