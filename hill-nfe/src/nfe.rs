use crate::bindings::NfeBindings;
use std::ffi::{CString, c_char, c_int, c_void};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AcBrNfeError {
    #[error("Erro de I/O na biblioteca ACBr: {0}")]
    Io(String),
    #[error("Arquivo não encontrado pela biblioteca ACBr: {0}")]
    FileNotFound(String),
    #[error("Diretório não encontrado pela biblioteca ACBr: {0}")]
    DirectoryNotFound(String),
    #[error("Erro ao carregar a biblioteca dinâmica: {0}")]
    LoadLibError(#[from] libloading::Error),
    #[error("Erro interno do ACBr (Código {code}): {message}")]
    AcBr { code: i32, message: String },
    #[error("Erro de conversão de string UTF-8/CString: {0}")]
    StringError(String),
}

pub struct AcBrNfe {
    bindings: NfeBindings,
    handle: *mut c_void,
}

// Permitir o envio da struct entre threads de tarefas assíncronas do Tokio
unsafe impl Send for AcBrNfe {}
unsafe impl Sync for AcBrNfe {}

impl Drop for AcBrNfe {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                (self.bindings.NFE_Finalizar)(self.handle);
            }
        }
    }
}

impl AcBrNfe {
    /// Inicializa a biblioteca ACBr NFe a partir do caminho do arquivo compartilhado (.so ou .dll).
    pub fn new(lib_path: &str, arq_config: &str, chave_crypt: &str) -> Result<Self, AcBrNfeError> {
        let bindings = unsafe { NfeBindings::load(lib_path)? };
        let mut handle: *mut c_void = std::ptr::null_mut();

        let c_config =
            CString::new(arq_config).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let c_crypt =
            CString::new(chave_crypt).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;

        let ret =
            unsafe { (bindings.NFE_Inicializar)(&mut handle, c_config.as_ptr(), c_crypt.as_ptr()) };

        if ret < 0 {
            return Err(AcBrNfeError::AcBr {
                code: ret,
                message: "Falha na inicialização do ACBr NFe".to_string(),
            });
        }

        Ok(Self { bindings, handle })
    }

    fn check_result(&self, ret: i32) -> Result<(), AcBrNfeError> {
        if ret >= 0 {
            return Ok(());
        }

        let message = self.get_ultimo_retorno();
        match ret {
            -6 => Err(AcBrNfeError::DirectoryNotFound(message)),
            -5 => Err(AcBrNfeError::FileNotFound(message)),
            _ => Err(AcBrNfeError::AcBr { code: ret, message }),
        }
    }

    /// Retorna a mensagem associada ao último erro ou retorno da biblioteca.
    pub fn get_ultimo_retorno(&self) -> String {
        let mut buffer_size: c_int = 256;
        let mut buffer = vec![0u8; buffer_size as usize];

        unsafe {
            let _ret = (self.bindings.NFE_UltimoRetorno)(
                self.handle,
                buffer.as_mut_ptr() as *mut c_char,
                &mut buffer_size,
            );
            if buffer_size > 256 {
                buffer.resize(buffer_size as usize, 0);
                let _ret2 = (self.bindings.NFE_UltimoRetorno)(
                    self.handle,
                    buffer.as_mut_ptr() as *mut c_char,
                    &mut buffer_size,
                );
            }
        }

        let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
        String::from_utf8_lossy(&buffer[..len]).into_owned()
    }

    fn process_string_ret<F>(&self, f: F) -> Result<String, AcBrNfeError>
    where
        F: Fn(*mut c_char, *mut c_int) -> c_int,
    {
        let mut buffer_size: c_int = 256;
        let mut buffer = vec![0u8; buffer_size as usize];
        let ret = f(buffer.as_mut_ptr() as *mut c_char, &mut buffer_size);
        self.check_result(ret)?;

        if buffer_size > 256 {
            buffer.resize(buffer_size as usize, 0);
            let ret = f(buffer.as_mut_ptr() as *mut c_char, &mut buffer_size);
            self.check_result(ret)?;
        }

        let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
        Ok(String::from_utf8_lossy(&buffer[..len]).into_owned())
    }

    pub fn nome(&self) -> Result<String, AcBrNfeError> {
        self.process_string_ret(|buf, size| unsafe {
            (self.bindings.NFE_Nome)(self.handle, buf, size)
        })
    }

    pub fn versao(&self) -> Result<String, AcBrNfeError> {
        self.process_string_ret(|buf, size| unsafe {
            (self.bindings.NFE_Versao)(self.handle, buf, size)
        })
    }

    pub fn config_importar(&self, arq_config: &str) -> Result<(), AcBrNfeError> {
        let c_config =
            CString::new(arq_config).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let ret = unsafe { (self.bindings.NFE_ConfigImportar)(self.handle, c_config.as_ptr()) };
        self.check_result(ret)
    }

    pub fn config_exportar(&self) -> Result<String, AcBrNfeError> {
        self.process_string_ret(|buf, size| unsafe {
            (self.bindings.NFE_ConfigExportar)(self.handle, buf, size)
        })
    }

    pub fn config_ler(&self, arq_config: &str) -> Result<(), AcBrNfeError> {
        let c_config =
            CString::new(arq_config).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let ret = unsafe { (self.bindings.NFE_ConfigLer)(self.handle, c_config.as_ptr()) };
        self.check_result(ret)
    }

    pub fn config_gravar(&self, arq_config: &str) -> Result<(), AcBrNfeError> {
        let c_config =
            CString::new(arq_config).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let ret = unsafe { (self.bindings.NFE_ConfigGravar)(self.handle, c_config.as_ptr()) };
        self.check_result(ret)
    }

    pub fn config_ler_val(&self, sessao: &str, chave: &str) -> Result<String, AcBrNfeError> {
        let c_sessao =
            CString::new(sessao).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let c_chave = CString::new(chave).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        self.process_string_ret(|buf, size| unsafe {
            (self.bindings.NFE_ConfigLerValor)(
                self.handle,
                c_sessao.as_ptr(),
                c_chave.as_ptr(),
                buf,
                size,
            )
        })
    }

    pub fn config_gravar_val(
        &self,
        sessao: &str,
        chave: &str,
        valor: &str,
    ) -> Result<(), AcBrNfeError> {
        let c_sessao =
            CString::new(sessao).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let c_chave = CString::new(chave).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let c_valor = CString::new(valor).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let ret = unsafe {
            (self.bindings.NFE_ConfigGravarValor)(
                self.handle,
                c_sessao.as_ptr(),
                c_chave.as_ptr(),
                c_valor.as_ptr(),
            )
        };
        self.check_result(ret)
    }

    pub fn carregar_xml(&self, arquivo_ou_xml: &str) -> Result<(), AcBrNfeError> {
        let c_xml =
            CString::new(arquivo_ou_xml).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let ret = unsafe { (self.bindings.NFE_CarregarXML)(self.handle, c_xml.as_ptr()) };
        self.check_result(ret)
    }

    pub fn carregar_ini(&self, arquivo_ou_ini: &str) -> Result<(), AcBrNfeError> {
        let c_ini =
            CString::new(arquivo_ou_ini).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let ret = unsafe { (self.bindings.NFE_CarregarINI)(self.handle, c_ini.as_ptr()) };
        self.check_result(ret)
    }

    pub fn obter_xml(&self, index: i32) -> Result<String, AcBrNfeError> {
        self.process_string_ret(|buf, size| unsafe {
            (self.bindings.NFE_ObterXml)(self.handle, index as c_int, buf, size)
        })
    }

    pub fn gravar_xml(
        &self,
        index: i32,
        nome_arquivo: &str,
        path_arquivo: &str,
    ) -> Result<(), AcBrNfeError> {
        let c_nome =
            CString::new(nome_arquivo).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let c_path =
            CString::new(path_arquivo).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let ret = unsafe {
            (self.bindings.NFE_GravarXml)(
                self.handle,
                index as c_int,
                c_nome.as_ptr(),
                c_path.as_ptr(),
            )
        };
        self.check_result(ret)
    }

    pub fn obter_ini(&self, index: i32) -> Result<String, AcBrNfeError> {
        self.process_string_ret(|buf, size| unsafe {
            (self.bindings.NFE_ObterIni)(self.handle, index as c_int, buf, size)
        })
    }

    pub fn gravar_ini(
        &self,
        index: i32,
        nome_arquivo: &str,
        path_arquivo: &str,
    ) -> Result<(), AcBrNfeError> {
        let c_nome =
            CString::new(nome_arquivo).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let c_path =
            CString::new(path_arquivo).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let ret = unsafe {
            (self.bindings.NFE_GravarIni)(
                self.handle,
                index as c_int,
                c_nome.as_ptr(),
                c_path.as_ptr(),
            )
        };
        self.check_result(ret)
    }

    pub fn limpar_lista(&self) -> Result<(), AcBrNfeError> {
        let ret = unsafe { (self.bindings.NFE_LimparLista)(self.handle) };
        self.check_result(ret)
    }

    pub fn limpar_lista_eventos(&self) -> Result<(), AcBrNfeError> {
        let ret = unsafe { (self.bindings.NFE_LimparListaEventos)(self.handle) };
        self.check_result(ret)
    }

    pub fn assinar(&self) -> Result<(), AcBrNfeError> {
        let ret = unsafe { (self.bindings.NFE_Assinar)(self.handle) };
        self.check_result(ret)
    }

    pub fn validar(&self) -> Result<(), AcBrNfeError> {
        let ret = unsafe { (self.bindings.NFE_Validar)(self.handle) };
        self.check_result(ret)
    }

    pub fn gerar_chave(
        &self,
        codigo_uf: i32,
        codigo_numerico: i32,
        modelo: i32,
        serie: i32,
        numero: i32,
        tp_emi: i32,
        emissao: &str,
        cpf_cnpj: &str,
    ) -> Result<String, AcBrNfeError> {
        let c_emissao =
            CString::new(emissao).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let c_cpf_cnpj =
            CString::new(cpf_cnpj).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;

        self.process_string_ret(|buf, size| unsafe {
            (self.bindings.NFE_GerarChave)(
                self.handle,
                codigo_uf as c_int,
                codigo_numerico as c_int,
                modelo as c_int,
                serie as c_int,
                numero as c_int,
                tp_emi as c_int,
                c_emissao.as_ptr(),
                c_cpf_cnpj.as_ptr(),
                buf,
                size,
            )
        })
    }

    pub fn status_servico(&self) -> Result<String, AcBrNfeError> {
        self.process_string_ret(|buf, size| unsafe {
            (self.bindings.NFE_StatusServico)(self.handle, buf, size)
        })
    }

    pub fn consultar(
        &self,
        chave_ou_nfe: &str,
        extrair_eventos: bool,
    ) -> Result<String, AcBrNfeError> {
        let c_chave =
            CString::new(chave_ou_nfe).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        self.process_string_ret(|buf, size| unsafe {
            (self.bindings.NFE_Consultar)(self.handle, c_chave.as_ptr(), extrair_eventos, buf, size)
        })
    }

    pub fn cancelar(
        &self,
        chave: &str,
        justificativa: &str,
        cnpj: &str,
        lote: i32,
    ) -> Result<String, AcBrNfeError> {
        let c_chave = CString::new(chave).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let c_just =
            CString::new(justificativa).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let c_cnpj = CString::new(cnpj).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;

        self.process_string_ret(|buf, size| unsafe {
            (self.bindings.NFE_Cancelar)(
                self.handle,
                c_chave.as_ptr(),
                c_just.as_ptr(),
                c_cnpj.as_ptr(),
                lote as c_int,
                buf,
                size,
            )
        })
    }

    pub fn inutilizar(
        &self,
        cnpj: &str,
        justificativa: &str,
        ano: i32,
        modelo: i32,
        serie: i32,
        numero_inicial: i32,
        numero_final: i32,
    ) -> Result<String, AcBrNfeError> {
        let c_cnpj = CString::new(cnpj).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;
        let c_just =
            CString::new(justificativa).map_err(|e| AcBrNfeError::StringError(e.to_string()))?;

        self.process_string_ret(|buf, size| unsafe {
            (self.bindings.NFE_Inutilizar)(
                self.handle,
                c_cnpj.as_ptr(),
                c_just.as_ptr(),
                ano as c_int,
                modelo as c_int,
                serie as c_int,
                numero_inicial as c_int,
                numero_final as c_int,
                buf,
                size,
            )
        })
    }

    pub fn enviar(
        &self,
        lote: i32,
        imprimir: bool,
        sincrono: bool,
        zipado: bool,
    ) -> Result<String, AcBrNfeError> {
        self.process_string_ret(|buf, size| unsafe {
            (self.bindings.NFE_Enviar)(
                self.handle,
                lote as c_int,
                imprimir,
                sincrono,
                zipado,
                buf,
                size,
            )
        })
    }

    pub fn salvar_pdf(&self) -> Result<String, AcBrNfeError> {
        self.process_string_ret(|buf, size| unsafe {
            (self.bindings.NFE_SalvarPDF)(self.handle, buf, size)
        })
    }
}
