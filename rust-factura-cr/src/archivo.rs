use anyhow::{Context, Result};

use std::fs;

pub fn guardar_xml(ruta: &str, contenido: &str) -> Result<()> {
    fs::write(ruta, contenido).context("No se pudo guardar XML")?;

    Ok(())
}
