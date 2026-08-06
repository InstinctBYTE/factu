use anyhow::Result;
use chrono::{FixedOffset, Utc};

pub fn crear_factura_xml(clave: &str, consecutivo: &str) -> Result<String> {
    let cr_offset = FixedOffset::west_opt(6 * 3600).unwrap();

    let fecha = Utc::now()
        .with_timezone(&cr_offset)
        .format("%Y-%m-%dT%H:%M:%S%:z")
        .to_string();

    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<FacturaElectronica
    xmlns="https://cdn.comprobanteselectronicos.go.cr/xml-schemas/v4.4/facturaElectronica"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xmlns:ds="http://www.w3.org/2000/09/xmldsig#"
    xsi:schemaLocation="https://cdn.comprobanteselectronicos.go.cr/xml-schemas/v4.4/facturaElectronica https://cdn.comprobanteselectronicos.go.cr/xml-schemas/v4.4/facturaElectronica.xsd">

    <Clave>{clave}</Clave>

    <ProveedorSistemas>111111111</ProveedorSistemas>

    <CodigoActividadEmisor>6810.0</CodigoActividadEmisor>

    <NumeroConsecutivo>{consecutivo}</NumeroConsecutivo>

    <FechaEmision>{fecha}</FechaEmision>

    <Emisor>

        <Nombre>Nombre del emisor</Nombre>

        <Identificacion>
            <Tipo>01</Tipo>
            <Numero>111111111</Numero>
        </Identificacion>

        <NombreComercial>EMPRESA DEMO</NombreComercial>

        <Ubicacion>
            <Provincia>4</Provincia>
            <Canton>06</Canton>
            <Distrito>04</Distrito>
            <Barrio>San Francisco</Barrio>
            <OtrasSenas>San Isidro Costa Rica</OtrasSenas>
        </Ubicacion>

        <Telefono>
            <CodigoPais>506</CodigoPais>
            <NumTelefono>88888888</NumTelefono>
        </Telefono>

        <CorreoElectronico>tribu.factu.lg@gmail.com</CorreoElectronico>

    </Emisor>

    <Receptor>

        <Nombre>CLIENTE DEMO</Nombre>

        <Identificacion>
            <Tipo>01</Tipo>
            <Numero>111111111</Numero>
        </Identificacion>

        <Ubicacion>
            <Provincia>1</Provincia>
            <Canton>01</Canton>
            <Distrito>01</Distrito>
            <OtrasSenas>Cliente Costa Rica</OtrasSenas>
        </Ubicacion>

        <Telefono>
            <CodigoPais>506</CodigoPais>
            <NumTelefono>88887777</NumTelefono>
        </Telefono>

        <CorreoElectronico>cliente@correo.com</CorreoElectronico>

    </Receptor>

    <CondicionVenta>01</CondicionVenta>

    <PlazoCredito>0</PlazoCredito>

    <DetalleServicio>

        <LineaDetalle>

            <NumeroLinea>1</NumeroLinea>

            <CodigoCABYS>7211100000100</CodigoCABYS>

            <CodigoComercial>
                <Tipo>01</Tipo>
                <Codigo>SERV001</Codigo>
            </CodigoComercial>

            <Cantidad>1.00</Cantidad>

            <UnidadMedida>Unid</UnidadMedida>

            <TipoTransaccion>01</TipoTransaccion>

            <Detalle>Servicio profesional desarrollo software</Detalle>

            <PrecioUnitario>100000.00</PrecioUnitario>

            <MontoTotal>100000.00</MontoTotal>

            <SubTotal>100000.00</SubTotal>

            <BaseImponible>100000.00</BaseImponible>

            <Impuesto>

                <Codigo>01</Codigo>

                <CodigoTarifaIVA>08</CodigoTarifaIVA>

                <Tarifa>13.00</Tarifa>

                <Monto>13000.00</Monto>

            </Impuesto>

            <ImpuestoAsumidoEmisorFabrica>0.00</ImpuestoAsumidoEmisorFabrica>

            <ImpuestoNeto>13000.00</ImpuestoNeto>

            <MontoTotalLinea>113000.00</MontoTotalLinea>

        </LineaDetalle>

    </DetalleServicio>

    <ResumenFactura>

        <CodigoTipoMoneda>

            <CodigoMoneda>CRC</CodigoMoneda>

            <TipoCambio>1.00</TipoCambio>

        </CodigoTipoMoneda>

        <TotalServGravados>100000.00</TotalServGravados>

        <TotalServExentos>0.00</TotalServExentos>

        <TotalServExonerado>0.00</TotalServExonerado>

        <TotalServNoSujeto>0.00</TotalServNoSujeto>

        <TotalMercanciasGravadas>0.00</TotalMercanciasGravadas>

        <TotalMercanciasExentas>0.00</TotalMercanciasExentas>

        <TotalMercExonerada>0.00</TotalMercExonerada>

        <TotalMercNoSujeta>0.00</TotalMercNoSujeta>

        <TotalGravado>100000.00</TotalGravado>

        <TotalExento>0.00</TotalExento>

        <TotalExonerado>0.00</TotalExonerado>

        <TotalVenta>100000.00</TotalVenta>

        <TotalDescuentos>0.00</TotalDescuentos>

        <TotalVentaNeta>100000.00</TotalVentaNeta>

        <TotalDesgloseImpuesto>

            <Codigo>01</Codigo>

            <CodigoTarifaIVA>08</CodigoTarifaIVA>

            <TotalMontoImpuesto>13000.00</TotalMontoImpuesto>

        </TotalDesgloseImpuesto>

        <TotalImpuesto>13000.00</TotalImpuesto>

        <TotalImpAsumEmisorFabrica>0.00</TotalImpAsumEmisorFabrica>

        <TotalIVADevuelto>0.00</TotalIVADevuelto>

        <MedioPago>
            <TipoMedioPago>01</TipoMedioPago>
        </MedioPago>

        <TotalComprobante>113000.00</TotalComprobante>

    </ResumenFactura>

    <Otros>
        <OtroTexto>Generado por Rust</OtroTexto>
    </Otros>

</FacturaElectronica>"#
    );

    Ok(xml)
}
