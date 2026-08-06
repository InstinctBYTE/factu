use anyhow::Result;
use reqwest::Client;

use crate::models::HaciendaRespuesta;

pub async fn consultar_comprobante(client: &Client, token: &str, clave: &str) -> Result<()> {
    let url = format!(
        "https://api-sandbox.comprobanteselectronicos.go.cr/recepcion/v1/recepcion/{}",
        clave
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token.trim()))
        .send()
        .await?;

    let status = response.status();

    println!("STATUS CONSULTA => {}", status);

    let body = response.text().await?;

    println!("BODY CONSULTA => {}", body);

    // intentar parsear JSON
    let parsed: Result<HaciendaRespuesta, _> = serde_json::from_str(&body);

    match parsed {
        Ok(data) => {
            println!("==============================");
            println!("ESTADO HACIENDA");
            println!("==============================");

            println!("CLAVE => {}", data.clave.unwrap_or("SIN CLAVE".to_string()));

            println!(
                "ESTADO => {}",
                data.ind_estado.unwrap_or("SIN ESTADO".to_string())
            );

            if let Some(xml) = data.respuesta_xml {
                println!("XML RESPUESTA => {}", xml);
            }
        }

        Err(_) => {
            println!("No se pudo parsear respuesta JSON");
        }
    }

    Ok(())
}
