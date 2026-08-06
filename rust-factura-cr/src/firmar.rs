use anyhow::{Result, anyhow};
use std::path::Path;
use std::process::Command;

pub fn firmar_xml(input_xml: &str, output_xml: &str, p12_path: &str, pin: &str) -> Result<()> {
    let output = Command::new("php")
        .args([
            "php_signer/wrapper.php",
            p12_path,
            pin,
            input_xml,
            output_xml,
        ])
        .output()?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);

        return Err(anyhow!("Error firmando XML: {}", err));
    }

    // =========================================
    // VERIFICAR ARCHIVO GENERADO
    // =========================================

    if !Path::new(output_xml).exists() {
        return Err(anyhow!("No se generó XML firmado"));
    }

    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}
