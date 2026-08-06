mod archivo;
mod clave;
mod config;
mod consulta;
mod factura;
mod firmar;
mod models;
mod oauth;
mod recepcion;
mod validar;
use anyhow::Result;

use reqwest::Client;

use tracing::{error, info};

use config::HaciendaConfig;

#[tokio::main]
async fn main() -> Result<()> {
    // =========================================================================
    // LOGGER
    // =========================================================================

    tracing_subscriber::fmt::init();

    info!("==================================================");
    info!("INICIANDO SISTEMA HACIENDA CR");
    info!("==================================================");

    // =========================================================================
    // CONFIG
    // =========================================================================

    let config = HaciendaConfig::sandbox();

    info!("Ambiente sandbox configurado");

    // =========================================================================
    // HTTP CLIENT
    // =========================================================================

    let client = Client::builder().build()?;

    info!("Cliente HTTP inicializado");

    // =========================================================================
    // TOKEN
    // =========================================================================

    let token = oauth::get_token(&client, &config).await?;

    info!("TOKEN OK");
    info!("Tipo token: {}", token.token_type);
    info!("Expira en: {} segundos", token.expires_in);

    // =========================================================================
    // CONSECUTIVO
    // =========================================================================

    let consecutivo = clave::generar_consecutivo(1, 1, 1, 1);

    info!("Consecutivo generado:");
    info!("{}", consecutivo);

    // =========================================================================
    // CLAVE
    // =========================================================================

    let clave = clave::generar_clave("0504010572", &consecutivo, 1, 12345678);

    info!("Clave generada:");
    info!("{}", clave);

    info!("Longitud clave:");
    info!("{}", clave.len());

    // =========================================================================
    // VALIDACION CLAVE
    // =========================================================================

    if clave.len() != 50 {
        error!("La clave no tiene 50 caracteres");
        return Err(anyhow::anyhow!("Clave Hacienda invalida"));
    }

    info!("Clave validada correctamente");

    // =========================================================================
    // CREAR XML
    // =========================================================================

    //let xml = factura::crear_factura_xml()?;
    let xml = factura::crear_factura_xml(&clave, &consecutivo)?;

    info!("==================================================");
    info!("XML GENERADO");
    info!("==================================================");

    info!("{}", xml);

    // =========================================================================
    // GUARDAR XML
    // =========================================================================

    archivo::guardar_xml("factura.xml", &xml)?;

    info!("XML guardado en factura.xml");

    // =========================================================
    // FIRMAR XML
    // =========================================================

    firmar::firmar_xml(
        "factura.xml",
        "factura_firmada.xml",
        "/home/ga/Documents/factu_php/certificado.p12",
        "1111",
    )?;

    info!("XML FIRMADO");

    // =========================================================================
    // VALIDAR XSD HACIENDA
    // =========================================================================

    info!("==================================================");
    info!("VALIDANDO XML CONTRA XSD HACIENDA");
    info!("==================================================");

    let xml_firmado = std::fs::read_to_string("factura_firmada.xml")?;

    validar::validar_xml_contra_xsd(&xml_firmado, "xsd/FacturaElectronica_V4.4.xsd")?;

    info!("XML VALIDO CONTRA HACIENDA CR V4.4");

    // =========================================================================
    // RECEPCION
    // =========================================================================
    recepcion::enviar_comprobante(&client, &token.access_token, &clave).await?;

    // =========================================================
    // CONSULTAR ESTADO
    // =========================================================

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    consulta::consultar_comprobante(&client, &token.access_token, &clave).await?;

    // =========================================================================
    // FINAL
    // =========================================================================

    info!("==================================================");
    info!("PROCESO FINALIZADO CORRECTAMENTE");
    info!("==================================================");

    Ok(())
}
