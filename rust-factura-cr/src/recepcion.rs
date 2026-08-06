use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};
use reqwest::Client;
use time::{OffsetDateTime, UtcOffset};

use crate::models::*;

fn fecha_hacienda() -> String {
    let now = OffsetDateTime::now_utc().to_offset(UtcOffset::from_hms(-6, 0, 0).unwrap());

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}-06:00",
        now.year(),
        now.month() as u8,
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    )
}

pub async fn enviar_comprobante(client: &Client, token: &str, clave: &str) -> Result<()> {
    // ============================================
    // LEER XML FIRMADO
    // ============================================

    let xml = std::fs::read("factura_firmada.xml")?;

    // ============================================
    // BASE64
    // ============================================

    let xml_base64 = general_purpose::STANDARD.encode(xml);

    // ============================================
    // FECHA HACIENDA
    // ============================================

    let fecha = fecha_hacienda();

    // ============================================
    // PAYLOAD
    // ============================================

    let payload = RecepcionRequest {
        clave: clave.to_string(),

        fecha,

        emisor: Identificacion {
            tipo_identificacion: "01".to_string(),
            numero_identificacion: "504010572".to_string(),
        },

        receptor: Identificacion {
            tipo_identificacion: "01".to_string(),
            numero_identificacion: "123456789".to_string(),
        },

        comprobante_xml: xml_base64,
    };

    println!("====================================");
    println!("JSON ENVIADO A HACIENDA");
    println!("====================================");

    println!("{}", serde_json::to_string_pretty(&payload)?);

    println!("====================================");

    // ============================================
    // REQUEST
    // ============================================

    let response = client
        .post("https://api-sandbox.comprobanteselectronicos.go.cr/recepcion/v1/recepcion")
        .header("Authorization", format!("Bearer {}", token.trim()))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    let status = response.status();

    let body = response.text().await?;

    println!("STATUS: {}", status);
    println!("BODY: {}", body);

    Ok(())
}
