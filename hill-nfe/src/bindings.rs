use std::ffi::{c_char, c_int, c_void};
use libloading::Library;

#[allow(non_snake_case)]
pub struct NfeBindings {
    _lib: Library,

    pub NFE_Inicializar: unsafe extern "C" fn(handle: *mut *mut c_void, e_arq_config: *const c_char, e_chave_crypt: *const c_char) -> c_int,
    pub NFE_Finalizar: unsafe extern "C" fn(handle: *mut c_void) -> c_int,
    pub NFE_Nome: unsafe extern "C" fn(handle: *mut c_void, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int,
    pub NFE_Versao: unsafe extern "C" fn(handle: *mut c_void, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int,
    pub NFE_UltimoRetorno: unsafe extern "C" fn(handle: *mut c_void, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int,
    pub NFE_ConfigImportar: unsafe extern "C" fn(handle: *mut c_void, e_arq_config: *const c_char) -> c_int,
    pub NFE_ConfigExportar: unsafe extern "C" fn(handle: *mut c_void, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int,
    pub NFE_ConfigLer: unsafe extern "C" fn(handle: *mut c_void, e_arq_config: *const c_char) -> c_int,
    pub NFE_ConfigGravar: unsafe extern "C" fn(handle: *mut c_void, e_arq_config: *const c_char) -> c_int,
    pub NFE_ConfigLerValor: unsafe extern "C" fn(handle: *mut c_void, e_sessao: *const c_char, e_chave: *const c_char, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int,
    pub NFE_ConfigGravarValor: unsafe extern "C" fn(handle: *mut c_void, e_sessao: *const c_char, e_chave: *const c_char, valor: *const c_char) -> c_int,
    pub NFE_CarregarXML: unsafe extern "C" fn(handle: *mut c_void, e_arquivo_ou_xml: *const c_char) -> c_int,
    pub NFE_CarregarINI: unsafe extern "C" fn(handle: *mut c_void, e_arquivo_ou_ini: *const c_char) -> c_int,
    pub NFE_ObterXml: unsafe extern "C" fn(handle: *mut c_void, a_index: c_int, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int,
    pub NFE_GravarXml: unsafe extern "C" fn(handle: *mut c_void, a_index: c_int, e_nome_arquivo: *const c_char, e_path_arquivo: *const c_char) -> c_int,
    pub NFE_ObterIni: unsafe extern "C" fn(handle: *mut c_void, a_index: c_int, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int,
    pub NFE_GravarIni: unsafe extern "C" fn(handle: *mut c_void, a_index: c_int, e_nome_arquivo: *const c_char, e_path_arquivo: *const c_char) -> c_int,
    pub NFE_LimparLista: unsafe extern "C" fn(handle: *mut c_void) -> c_int,
    pub NFE_LimparListaEventos: unsafe extern "C" fn(handle: *mut c_void) -> c_int,
    pub NFE_Assinar: unsafe extern "C" fn(handle: *mut c_void) -> c_int,
    pub NFE_Validar: unsafe extern "C" fn(handle: *mut c_void) -> c_int,
    pub NFE_GerarChave: unsafe extern "C" fn(
        handle: *mut c_void,
        a_codigo_uf: c_int,
        a_codigo_numerico: c_int,
        a_modelo: c_int,
        a_serie: c_int,
        a_numero: c_int,
        a_tp_emi: c_int,
        a_emissao: *const c_char,
        cpf_cnpj: *const c_char,
        buffer: *mut c_char,
        buffer_size: *mut c_int,
    ) -> c_int,
    pub NFE_StatusServico: unsafe extern "C" fn(handle: *mut c_void, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int,
    pub NFE_Consultar: unsafe extern "C" fn(handle: *mut c_void, e_chave_ou_nfe: *const c_char, a_extrair_eventos: bool, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int,
    pub NFE_Cancelar: unsafe extern "C" fn(
        handle: *mut c_void,
        e_chave: *const c_char,
        e_justificativa: *const c_char,
        e_cnpj: *const c_char,
        a_lote: c_int,
        buffer: *mut c_char,
        buffer_size: *mut c_int,
    ) -> c_int,
    pub NFE_Inutilizar: unsafe extern "C" fn(
        handle: *mut c_void,
        acnpj: *const c_char,
        a_justificativa: *const c_char,
        ano: c_int,
        modelo: c_int,
        serie: c_int,
        numero_inicial: c_int,
        numero_final: c_int,
        buffer: *mut c_char,
        buffer_size: *mut c_int,
    ) -> c_int,
    pub NFE_Enviar: unsafe extern "C" fn(
        handle: *mut c_void,
        a_lote: c_int,
        imprimir: bool,
        sincrono: bool,
        zipado: bool,
        buffer: *mut c_char,
        buffer_size: *mut c_int,
    ) -> c_int,
    pub NFE_SalvarPDF: unsafe extern "C" fn(handle: *mut c_void, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int,
}

impl NfeBindings {
    pub unsafe fn load(path: &str) -> Result<Self, libloading::Error> {
        let lib = unsafe { Library::new(path)? };

        let nfe_inicializar = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut *mut c_void, e_arq_config: *const c_char, e_chave_crypt: *const c_char) -> c_int>(b"NFE_Inicializar")? };
        let nfe_finalizar = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void) -> c_int>(b"NFE_Finalizar")? };
        let nfe_nome = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int>(b"NFE_Nome")? };
        let nfe_versao = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int>(b"NFE_Versao")? };
        let nfe_ultimo_retorno = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int>(b"NFE_UltimoRetorno")? };
        let nfe_config_importar = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, e_arq_config: *const c_char) -> c_int>(b"NFE_ConfigImportar")? };
        let nfe_config_exportar = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int>(b"NFE_ConfigExportar")? };
        let nfe_config_ler = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, e_arq_config: *const c_char) -> c_int>(b"NFE_ConfigLer")? };
        let nfe_config_gravar = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, e_arq_config: *const c_char) -> c_int>(b"NFE_ConfigGravar")? };
        let nfe_config_ler_valor = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, e_sessao: *const c_char, e_chave: *const c_char, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int>(b"NFE_ConfigLerValor")? };
        let nfe_config_gravar_valor = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, e_sessao: *const c_char, e_chave: *const c_char, valor: *const c_char) -> c_int>(b"NFE_ConfigGravarValor")? };
        let nfe_carregar_xml = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, e_arquivo_ou_xml: *const c_char) -> c_int>(b"NFE_CarregarXML")? };
        let nfe_carregar_ini = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, e_arquivo_ou_ini: *const c_char) -> c_int>(b"NFE_CarregarINI")? };
        let nfe_obter_xml = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, a_index: c_int, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int>(b"NFE_ObterXml")? };
        let nfe_gravar_xml = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, a_index: c_int, e_nome_arquivo: *const c_char, e_path_arquivo: *const c_char) -> c_int>(b"NFE_GravarXml")? };
        let nfe_obter_ini = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, a_index: c_int, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int>(b"NFE_ObterIni")? };
        let nfe_gravar_ini = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, a_index: c_int, e_nome_arquivo: *const c_char, e_path_arquivo: *const c_char) -> c_int>(b"NFE_GravarIni")? };
        let nfe_limpar_lista = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void) -> c_int>(b"NFE_LimparLista")? };
        let nfe_limpar_lista_eventos = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void) -> c_int>(b"NFE_LimparListaEventos")? };
        let nfe_assinar = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void) -> c_int>(b"NFE_Assinar")? };
        let nfe_validar = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void) -> c_int>(b"NFE_Validar")? };
        
        let nfe_gerar_chave = unsafe { *lib.get::<unsafe extern "C" fn(
            handle: *mut c_void,
            a_codigo_uf: c_int,
            a_codigo_numerico: c_int,
            a_modelo: c_int,
            a_serie: c_int,
            a_numero: c_int,
            a_tp_emi: c_int,
            a_emissao: *const c_char,
            cpf_cnpj: *const c_char,
            buffer: *mut c_char,
            buffer_size: *mut c_int,
        ) -> c_int>(b"NFE_GerarChave")? };

        let nfe_status_servico = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int>(b"NFE_StatusServico")? };
        let nfe_consultar = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, e_chave_ou_nfe: *const c_char, a_extrair_eventos: bool, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int>(b"NFE_Consultar")? };
        
        let nfe_cancelar = unsafe { *lib.get::<unsafe extern "C" fn(
            handle: *mut c_void,
            e_chave: *const c_char,
            e_justificativa: *const c_char,
            e_cnpj: *const c_char,
            a_lote: c_int,
            buffer: *mut c_char,
            buffer_size: *mut c_int,
        ) -> c_int>(b"NFE_Cancelar")? };

        let nfe_inutilizar = unsafe { *lib.get::<unsafe extern "C" fn(
            handle: *mut c_void,
            acnpj: *const c_char,
            a_justificativa: *const c_char,
            ano: c_int,
            modelo: c_int,
            serie: c_int,
            numero_inicial: c_int,
            numero_final: c_int,
            buffer: *mut c_char,
            buffer_size: *mut c_int,
        ) -> c_int>(b"NFE_Inutilizar")? };

        let nfe_enviar = unsafe { *lib.get::<unsafe extern "C" fn(
            handle: *mut c_void,
            a_lote: c_int,
            imprimir: bool,
            sincrono: bool,
            zipado: bool,
            buffer: *mut c_char,
            buffer_size: *mut c_int,
        ) -> c_int>(b"NFE_Enviar")? };

        let nfe_salvar_pdf = unsafe { *lib.get::<unsafe extern "C" fn(handle: *mut c_void, buffer: *mut c_char, buffer_size: *mut c_int) -> c_int>(b"NFE_SalvarPDF")? };

        Ok(Self {
            _lib: lib,
            NFE_Inicializar: nfe_inicializar,
            NFE_Finalizar: nfe_finalizar,
            NFE_Nome: nfe_nome,
            NFE_Versao: nfe_versao,
            NFE_UltimoRetorno: nfe_ultimo_retorno,
            NFE_ConfigImportar: nfe_config_importar,
            NFE_ConfigExportar: nfe_config_exportar,
            NFE_ConfigLer: nfe_config_ler,
            NFE_ConfigGravar: nfe_config_gravar,
            NFE_ConfigLerValor: nfe_config_ler_valor,
            NFE_ConfigGravarValor: nfe_config_gravar_valor,
            NFE_CarregarXML: nfe_carregar_xml,
            NFE_CarregarINI: nfe_carregar_ini,
            NFE_ObterXml: nfe_obter_xml,
            NFE_GravarXml: nfe_gravar_xml,
            NFE_ObterIni: nfe_obter_ini,
            NFE_GravarIni: nfe_gravar_ini,
            NFE_LimparLista: nfe_limpar_lista,
            NFE_LimparListaEventos: nfe_limpar_lista_eventos,
            NFE_Assinar: nfe_assinar,
            NFE_Validar: nfe_validar,
            NFE_GerarChave: nfe_gerar_chave,
            NFE_StatusServico: nfe_status_servico,
            NFE_Consultar: nfe_consultar,
            NFE_Cancelar: nfe_cancelar,
            NFE_Inutilizar: nfe_inutilizar,
            NFE_Enviar: nfe_enviar,
            NFE_SalvarPDF: nfe_salvar_pdf,
        })
    }
}
