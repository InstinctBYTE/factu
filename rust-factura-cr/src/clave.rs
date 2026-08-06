use chrono::Local;

pub fn generar_consecutivo(
    sucursal: u32,
    terminal: u32,
    tipo_documento: u32,
    numero: u32,
) -> String {
    format!(
        "{:03}{:05}{:02}{:010}",
        sucursal, terminal, tipo_documento, numero
    )
}

pub fn generar_clave(
    cedula: &str,
    consecutivo: &str,
    situacion: u8,
    codigo_seguridad: u32,
) -> String {
    let now = Local::now();

    // DDMMYY obligatorio
    let fecha = now.format("%d%m%y").to_string();

    let pais = "506";

    // SOLO números (evita basura silenciosa)
    let cedula = cedula
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>();
    let cedula = format!("{:012}", cedula.parse::<u64>().unwrap_or(0));

    let situacion = situacion.to_string();

    let seguridad = format!("{:08}", codigo_seguridad);

    format!(
        "{}{}{}{}{}{}",
        pais, fecha, cedula, consecutivo, situacion, seguridad
    )
}
