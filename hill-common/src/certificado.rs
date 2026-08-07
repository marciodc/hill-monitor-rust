use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit},
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use thiserror::Error;

const KEY_PART_A: [u8; 32] = [
    0x93, 0x17, 0xE4, 0x5A, 0xC1, 0x2D, 0x88, 0xF0, 0x34, 0x6B, 0xA9, 0x0E, 0x75, 0xD2, 0x41, 0xBC,
    0x08, 0xFE, 0x63, 0x99, 0x27, 0xD5, 0xB0, 0x4C, 0xEA, 0x31, 0x7F, 0x86, 0x12, 0xC8, 0x5D, 0xA3,
];

const KEY_PART_B: [u8; 32] = [
    0xE6, 0x79, 0x8D, 0x32, 0xA8, 0x41, 0xE5, 0x9F, 0x5A, 0x02, 0xC5, 0x62, 0x1C, 0xBE, 0x28, 0xD0,
    0x61, 0x93, 0x0A, 0xF7, 0x4E, 0xB9, 0xDC, 0x25, 0x86, 0x58, 0x16, 0xEF, 0x7B, 0xA4, 0x30, 0xCD,
];

#[derive(Debug, Error)]
pub enum CertificadoError {
    #[error("formato do certificado protegido invalido")]
    FormatoInvalido,

    #[error("base64 invalido no envelope: {0}")]
    EnvelopeBase64(#[from] base64::DecodeError),

    #[error("nonce ou tag com tamanho invalido")]
    ParametrosInvalidos,

    #[error("falha ao autenticar/descriptografar o certificado")]
    Descriptografia,

    #[error("conteudo descriptografado nao e UTF-8")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("conteudo descriptografado nao contem um PFX Base64 valido")]
    PfxBase64Invalido,
}

pub fn chave_local_certificado() -> [u8; 32] {
    let mut chave = [0_u8; 32];

    for i in 0..32 {
        chave[i] = KEY_PART_A[i] ^ KEY_PART_B[31 - i];
    }

    chave
}

pub fn descriptografar_envelope_base64(
    valor_protegido: &str,
    chave: &[u8; 32],
) -> Result<String, CertificadoError> {
    let partes: Vec<&str> = valor_protegido.split(':').collect();

    if partes.len() != 4 || partes[0] != "v1" {
        return Err(CertificadoError::FormatoInvalido);
    }

    let nonce = STANDARD.decode(partes[1])?;
    let ciphertext = STANDARD.decode(partes[2])?;
    let tag = STANDARD.decode(partes[3])?;

    if nonce.len() != 12 || tag.len() != 16 || ciphertext.is_empty() {
        return Err(CertificadoError::ParametrosInvalidos);
    }

    let mut ciphertext_com_tag = Vec::with_capacity(ciphertext.len() + tag.len());
    ciphertext_com_tag.extend_from_slice(&ciphertext);
    ciphertext_com_tag.extend_from_slice(&tag);

    let key = Key::<Aes256Gcm>::try_from(chave.as_slice())
        .map_err(|_| CertificadoError::ParametrosInvalidos)?;
    let nonce =
        Nonce::try_from(nonce.as_slice()).map_err(|_| CertificadoError::ParametrosInvalidos)?;
    let cipher = Aes256Gcm::new(&key);
    let texto_puro = cipher
        .decrypt(&nonce, ciphertext_com_tag.as_ref())
        .map_err(|_| CertificadoError::Descriptografia)?;

    Ok(String::from_utf8(texto_puro)?)
}

pub fn descriptografar_texto_utf8(valor_protegido: &str) -> Result<String, CertificadoError> {
    let chave = chave_local_certificado();
    descriptografar_envelope_base64(valor_protegido, &chave)
}

pub fn validar_pfx_base64(pfx_base64: &str) -> Result<(), CertificadoError> {
    let pfx = STANDARD
        .decode(pfx_base64)
        .map_err(|_| CertificadoError::PfxBase64Invalido)?;

    if pfx.is_empty() {
        return Err(CertificadoError::PfxBase64Invalido);
    }

    Ok(())
}

pub fn descriptografar_pfx_base64(valor_protegido: &str) -> Result<String, CertificadoError> {
    let pfx_base64 = descriptografar_texto_utf8(valor_protegido)?;
    validar_pfx_base64(&pfx_base64)?;
    Ok(pfx_base64)
}
