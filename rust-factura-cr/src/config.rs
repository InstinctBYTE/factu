pub struct HaciendaConfig {
    pub token_url: String,
    pub client_id: String,
    pub username: String,
    pub password: String,
}

impl HaciendaConfig {
    pub fn sandbox() -> Self {
        Self {
            token_url: String::from(
                "https://idp.comprobanteselectronicos.go.cr/auth/realms/rut-stag/protocol/openid-connect/token",
            ),
            //cambiar por credenciales reales user , password
            client_id: String::from("api-stag"),

            username: String::from("cpf-05-0401-0572@stag.comprobanteselectronicos.go.cr"),

            password: String::from("F@jNiWj3i2pMLtnIZ-2U"),
        }
    }
}
