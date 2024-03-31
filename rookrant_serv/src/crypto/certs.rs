use rcgen::generate_simple_self_signed;

use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use std::path::Path;

use crate::AppResult;

pub fn generate_self_signed_cert_files(
    pub_file: impl AsRef<str>,
    key_file: impl AsRef<str>)
    -> AppResult<()>
{
    let pub_path = Path::new(pub_file.as_ref());
    let key_path = Path::new(key_file.as_ref());

    // Don't create if the files already exist.
    if pub_path.is_file() && key_path.is_file() {
        return Ok(())
    }

    log::info!("Generating a self signed certificate.");

    let subject_alt_names = vec![ "localhost".to_string()];
    let cert_key = generate_simple_self_signed(subject_alt_names)?;

    write_to_file(pub_path, cert_key.cert.pem())?;
    write_to_file(key_path, cert_key.key_pair.serialize_pem())?;

    Ok(())
}

fn write_to_file(file_path: &Path, content: String) -> AppResult<()> {
    if let Some(p_path) = file_path.parent() {
        if !p_path.is_dir() {
            create_dir_all(p_path)?;
        }
    }

    let mut fh = File::create(file_path)?;
    writeln!(fh, "{}", content)?;

    Ok(())
}
