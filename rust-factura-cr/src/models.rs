use serde::{Deserialize, Serialize};

//
// ============================================================
// OAUTH TOKEN RESPONSE
// ============================================================
//

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,

    pub expires_in: i32,

    pub refresh_expires_in: i32,

    pub refresh_token: String,

    pub token_type: String,

    pub scope: Option<String>,

    pub session_state: Option<String>,
}

//
// ============================================================
// IDENTIFICACION
// ============================================================
//

#[derive(Debug, Serialize, Deserialize)]
pub struct Identificacion {
    #[serde(rename = "tipoIdentificacion")]
    pub tipo_identificacion: String,

    #[serde(rename = "numeroIdentificacion")]
    pub numero_identificacion: String,
}

//
// ============================================================
// HACIENDA RECEPCION REQUEST
// ============================================================
//

#[derive(Debug, Serialize)]
pub struct RecepcionRequest {
    pub clave: String,

    pub fecha: String,

    pub emisor: Identificacion,

    pub receptor: Identificacion,

    #[serde(rename = "comprobanteXml")]
    pub comprobante_xml: String,
}

//
// ============================================================
// RESPUESTA HACIENDA RECEPCION / CONSULTA
// ============================================================
//

#[derive(Debug, Deserialize)]
pub struct HaciendaRespuesta {
    #[serde(rename = "clave")]
    pub clave: Option<String>,

    #[serde(rename = "fecha")]
    pub fecha: Option<String>,

    #[serde(rename = "ind-estado")]
    pub ind_estado: Option<String>,

    #[serde(rename = "respuesta-xml")]
    pub respuesta_xml: Option<String>,
}
