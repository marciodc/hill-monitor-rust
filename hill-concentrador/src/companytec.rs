use crate::com::ConcentradorCom;
use chrono::NaiveDateTime;
use hill_common::entity::Abastecimento;
use rust_decimal::Decimal;
use std::str::FromStr;
use tracing::error;

pub struct Companytec;

impl Companytec {
    pub fn adiciona_check(st: &str) -> String {
        let mut acumulador: u8 = 0;
        for c in st.chars() {
            acumulador = acumulador.wrapping_add(c as u8);
        }
        format!("({}{:02X})", st, acumulador)
    }

    pub async fn status_bicos(com: &ConcentradorCom) -> String {
        let ret = com.send_command("(&S)", true).await;
        if ret.len() < 51 {
            return String::new();
        }
        // Remove parentheses
        ret.replace('(', "").replace(')', "")
    }

    pub async fn visualiza_abastecimento(com: &ConcentradorCom) -> String {
        let ret = com.send_command("(&V)", true).await;
        let clean = ret.replace('(', "").replace(')', "");
        if clean == "0" {
            String::new()
        } else {
            clean
        }
    }

    pub async fn consulta_encerrante(com: &ConcentradorCom, bico: &str, decimais: i32) -> Decimal {
        let cmd_body = format!("&T{}L", bico);
        let command = Self::adiciona_check(&cmd_body);
        let ret = com.send_command(&command, true).await;
        if !ret.contains(')') {
            return Decimal::ZERO;
        }

        let clean = ret.replace('(', "").replace(')', "");
        if clean.len() < 14 {
            return Decimal::ZERO;
        }

        let divisor = if decimais == 3 {
            Decimal::new(1000, 0)
        } else {
            Decimal::new(100, 0)
        };

        let bico_res = &clean[2..4];
        let valor_str = &clean[4..14];

        if bico_res == bico && !valor_str.is_empty() {
            if let Ok(encerrante) = Decimal::from_str(valor_str) {
                return encerrante / divisor;
            } else {
                error!("Erro ao converter o encerrante. Retorno: {}", ret);
            }
        }

        Decimal::ZERO
    }

    pub async fn bloqueia_bico(com: &ConcentradorCom, bico: &str) -> bool {
        let cod_bico = format!("M{}", bico);
        let cmd_body = format!("&{}B", cod_bico);
        let command = Self::adiciona_check(&cmd_body);
        let ret = com.send_command(&command, true).await;
        ret == cod_bico
    }

    pub async fn desbloqueia_bico(com: &ConcentradorCom, bico: &str) -> bool {
        let cod_bico = format!("M{}", bico);
        let cmd_body = format!("&{}L", cod_bico);
        let command = Self::adiciona_check(&cmd_body);
        let ret = com.send_command(&command, true).await;
        ret == cod_bico
    }

    pub async fn atualiza_preco(com: &ConcentradorCom, bico: &str, tipo: i32, valor: Decimal) -> bool {
        // Format to 3 decimal places
        let valor_str = format!("{:.3}", valor);
        let numero_sem_ponto = valor_str.replace('.', "");
        let pad_left_len = 4;
        let mut padded = String::new();
        if numero_sem_ponto.len() < pad_left_len {
            for _ in 0..(pad_left_len - numero_sem_ponto.len()) {
                padded.push('0');
            }
        }
        padded.push_str(&numero_sem_ponto);

        let cod_bico = format!("U{}", bico);
        let cmd_body = format!("&{}{}{}0{}", cod_bico, tipo, 0, padded);
        let command = Self::adiciona_check(&cmd_body);
        let ret = com.send_command(&command, true).await;
        ret == format!("({})", cod_bico)
    }

    pub async fn incremento(com: &ConcentradorCom) {
        com.send_command("(&I)", false).await;
    }

    pub async fn captura_abastecimento(com: &ConcentradorCom) -> Option<Abastecimento> {
        let command = Self::adiciona_check("&A2");
        let raw_ret = com.send_command(&command, true).await;
        let clean = raw_ret.replace('(', "").replace(')', "");

        if clean.is_empty() || clean == "0" || clean.len() < 119 || &clean[117..119] != "00" {
            return None;
        }

        // Parse datetime fields
        // Format from C#: $"{abast[43..45]}-{abast[41..43]}-{abast[35..37]} {abast[37..39]}:{abast[39..41]}:00"
        let yy = &clean[43..45];
        let mm = &clean[41..43];
        let dd = &clean[35..37];
        let hh = &clean[37..39];
        let min = &clean[39..41];
        let data_hora_str = format!("20{}-{}-{}T{}:{}:00", yy, mm, dd, hh, min);

        let data_hora = match NaiveDateTime::parse_from_str(&data_hora_str, "%Y-%m-%dT%H:%M:%S") {
            Ok(dt) => dt,
            Err(e) => {
                error!("Erro ao converter a data/hora do abastecimento {}: {:?}", data_hora_str, e);
                return None;
            }
        };

        // Parse hex duration
        let tempo_hex = &clean[29..33];
        let segundos_totais = u32::from_str_radix(tempo_hex, 16).ok()?;
        let segundos = segundos_totais % 60;
        let minutos = segundos_totais / 60;
        let tempo_formatado = format!("{:02}:{:02}:00", minutos, segundos);

        // Numeric fields
        let quantidade_int = clean[17..23].parse::<i64>().ok()?;
        let valor_unit_int = clean[23..27].parse::<i64>().ok()?;
        let encerrante_inicial_int = clean[61..71].parse::<i64>().ok()?;
        let encerrante_final_int = clean[51..61].parse::<i64>().ok()?;
        let total_int = clean[11..17].parse::<i64>().ok()?;

        let mut divisor = Decimal::new(100, 0);
        let cod_virgula = &clean[27..29];
        if cod_virgula == "3E" {
            divisor = Decimal::new(1000, 0);
        }

        let mut enc_ini = encerrante_inicial_int;
        if enc_ini == 0 {
            enc_ini = encerrante_final_int - quantidade_int;
        }

        let quantidade = Decimal::new(quantidade_int, 0) / divisor;
        let valor_unitario = Decimal::new(valor_unit_int, 0) / Decimal::new(1000, 0);
        let encerrante_inicial = Decimal::new(enc_ini, 0) / divisor;
        let encerrante_final = Decimal::new(encerrante_final_int, 0) / divisor;
        let total = Decimal::new(total_int, 0) / divisor;

        let rfid_frentista_raw = &clean[85..101];
        let rfid_frentista = if rfid_frentista_raw != "0000000000000000" {
            Some(rfid_frentista_raw.to_string())
        } else {
            None
        };

        let rfid_cliente_raw = &clean[101..117];
        let rfid_cliente = if rfid_cliente_raw != "0000000000000000" {
            Some(rfid_cliente_raw.to_string())
        } else {
            None
        };

        Some(Abastecimento {
            id: uuid::Uuid::new_v4(),
            pdv: None,
            status: Some("P".to_string()),
            bloqueado: Some("F".to_string()),
            bico_id: 0, // Will be resolved by the caller
            retorno: Some(clean[33..35].to_string()),
            quantidade,
            valor_unitario,
            total,
            tempo: Some(tempo_formatado),
            data_hora,
            encerrante_inicial,
            encerrante_final,
            rfid_frentista,
            rfid_cliente,
            gerado: Some("F".to_string()),
            desmembramento_id: None,
            full_string: Some(clean),
            sincronizado: Some("F".to_string()),
        })
    }
}
