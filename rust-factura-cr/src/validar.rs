// validar.rs

use anyhow::{Result, anyhow};

use libxml::parser::Parser;

use libxml::schemas::{SchemaParserContext, SchemaValidationContext};

pub fn validar_xml_contra_xsd(xml: &str, xsd_path: &str) -> Result<()> {
    // =========================================================================
    // PARSE XML
    // =========================================================================

    let parser = Parser::default();

    let document = parser
        .parse_string(xml)
        .map_err(|e| anyhow!("Error parseando XML: {:?}", e))?;

    // =========================================================================
    // PARSE XSD
    // =========================================================================

    let mut parser_context = SchemaParserContext::from_file(xsd_path);

    // =========================================================================
    // CREAR VALIDATION CONTEXT
    // =========================================================================

    let mut validation_context = SchemaValidationContext::from_parser(&mut parser_context)
        .map_err(|errors| {
            let mensaje = errors
                .iter()
                .map(|e| format!("{:?}", e))
                .collect::<Vec<String>>()
                .join("\n");

            anyhow!("Error cargando XSD:\n{}", mensaje)
        })?;

    // =========================================================================
    // VALIDAR XML
    // =========================================================================

    validation_context
        .validate_document(&document)
        .map_err(|errors| {
            let mensaje = errors
                .iter()
                .map(|e| format!("{:?}", e))
                .collect::<Vec<String>>()
                .join("\n");

            anyhow!("XML INVALIDO CONTRA XSD:\n{}", mensaje)
        })?;

    Ok(())
}
