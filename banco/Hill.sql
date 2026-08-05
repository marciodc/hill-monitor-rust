--
-- PostgreSQL database dump
--

-- Dumped from database version 15.4
-- Dumped by pg_dump version 15.4

-- Started on 2026-08-04 23:12:34

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- TOC entry 2 (class 3079 OID 16747)
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;


--
-- TOC entry 3777 (class 0 OID 0)
-- Dependencies: 2
-- Name: EXTENSION "uuid-ossp"; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';


--
-- TOC entry 291 (class 1255 OID 16746)
-- Name: iif(boolean, anyelement, anyelement); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.iif(condition boolean, true_result anyelement, false_result anyelement) RETURNS anyelement
    LANGUAGE sql IMMUTABLE
    AS $$
  SELECT CASE WHEN condition THEN true_result ELSE false_result END
$$;


ALTER FUNCTION public.iif(condition boolean, true_result anyelement, false_result anyelement) OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 218 (class 1259 OID 33496)
-- Name: abastecimentos; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.abastecimentos (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    id_serial integer NOT NULL,
    status character(1),
    bloqueado character(1),
    bico_id integer,
    retorno character varying(3),
    quantidade numeric(15,4),
    valor_unitario numeric(15,2),
    total numeric(15,2),
    tempo character(8),
    encerrante_inicial numeric(15,4),
    encerrante_final numeric(15,4),
    data_hora timestamp without time zone,
    rfid_frentista character varying(16),
    rfid_cliente character varying(16),
    pdv uuid,
    gerado character(1) DEFAULT 'F'::bpchar,
    full_string character varying(250),
    data_alteracao timestamp without time zone,
    desmembramento_id uuid,
    sincronizado character(1) DEFAULT 'F'::bpchar
);


ALTER TABLE public.abastecimentos OWNER TO postgres;

--
-- TOC entry 217 (class 1259 OID 33495)
-- Name: abastecimentos_id_serial_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.abastecimentos_id_serial_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.abastecimentos_id_serial_seq OWNER TO postgres;

--
-- TOC entry 3778 (class 0 OID 0)
-- Dependencies: 217
-- Name: abastecimentos_id_serial_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.abastecimentos_id_serial_seq OWNED BY public.abastecimentos.id_serial;


--
-- TOC entry 219 (class 1259 OID 33503)
-- Name: administradoras; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.administradoras (
    id integer NOT NULL,
    descricao character varying(20),
    cnpj character varying(14),
    bandeira character varying(20)
);


ALTER TABLE public.administradoras OWNER TO postgres;

--
-- TOC entry 221 (class 1259 OID 33507)
-- Name: afericoes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.afericoes (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    id_serial integer NOT NULL,
    pdv uuid,
    setor_id integer,
    turno_id uuid,
    turno_posto_id uuid,
    data_hora timestamp without time zone,
    abastecimento_id uuid,
    bico_id integer,
    quantidade numeric(15,3),
    usuario_id integer,
    sincronizado character(1)
);


ALTER TABLE public.afericoes OWNER TO postgres;

--
-- TOC entry 220 (class 1259 OID 33506)
-- Name: afericoes_id_serial_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.afericoes_id_serial_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.afericoes_id_serial_seq OWNER TO postgres;

--
-- TOC entry 3779 (class 0 OID 0)
-- Dependencies: 220
-- Name: afericoes_id_serial_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.afericoes_id_serial_seq OWNED BY public.afericoes.id_serial;


--
-- TOC entry 222 (class 1259 OID 33512)
-- Name: alteracoes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.alteracoes (
    tabela character varying(40),
    alteracao character varying(32)
);


ALTER TABLE public.alteracoes OWNER TO postgres;

--
-- TOC entry 223 (class 1259 OID 33515)
-- Name: bicos; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.bicos (
    id integer NOT NULL,
    status character(1),
    retorno character varying(3),
    numero integer,
    bomba integer,
    tanque_id integer,
    produto_id integer,
    gtin character varying(14),
    combustivel character varying(40),
    tipo_combustivel character(1),
    altera_preco character(1),
    valor_unitario numeric(15,3),
    abastecimento_manual character(1),
    bloqueado character(1) DEFAULT 'F'::bpchar,
    setor_id integer,
    tabela_preco_id integer,
    cesna_master integer,
    cesna_slave integer,
    cesna_bomba_logica integer,
    cesna_bico_logico integer,
    bloqueio_quantidade numeric(15,3),
    valor_unitario_debito numeric(15,2),
    valor_unitario_credito numeric(15,2),
    sincroniza_preco_alterado character(1) DEFAULT 'F'::bpchar,
    sincroniza_preco_data_hora timestamp without time zone
);


ALTER TABLE public.bicos OWNER TO postgres;

--
-- TOC entry 225 (class 1259 OID 33520)
-- Name: bicos_encerrantes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.bicos_encerrantes (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    id_serial integer NOT NULL,
    turno_posto_id uuid,
    bico_id integer,
    status character(1),
    encerrante_inicial numeric(15,3) DEFAULT 0,
    encerrante_final numeric(15,3) DEFAULT 0,
    quantidade_vendida numeric(15,3) DEFAULT 0,
    afericao numeric(15,3) DEFAULT 0
);


ALTER TABLE public.bicos_encerrantes OWNER TO postgres;

--
-- TOC entry 224 (class 1259 OID 33519)
-- Name: bicos_encerrantes_id_serial_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.bicos_encerrantes_id_serial_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.bicos_encerrantes_id_serial_seq OWNER TO postgres;

--
-- TOC entry 3780 (class 0 OID 0)
-- Dependencies: 224
-- Name: bicos_encerrantes_id_serial_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.bicos_encerrantes_id_serial_seq OWNED BY public.bicos_encerrantes.id_serial;


--
-- TOC entry 227 (class 1259 OID 33530)
-- Name: caixa; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.caixa (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    id_serial integer NOT NULL,
    pdv uuid,
    tipo character(1),
    turno_id uuid,
    turno_posto_id uuid,
    venda_id uuid,
    sangria_suprimento_id uuid,
    forma_pagamento_id integer,
    valor numeric(15,2),
    data_hora timestamp without time zone,
    usuario_id integer,
    historico character varying(80),
    sincronizado character(1)
);


ALTER TABLE public.caixa OWNER TO postgres;

--
-- TOC entry 226 (class 1259 OID 33529)
-- Name: caixa_id_serial_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.caixa_id_serial_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.caixa_id_serial_seq OWNER TO postgres;

--
-- TOC entry 3781 (class 0 OID 0)
-- Dependencies: 226
-- Name: caixa_id_serial_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.caixa_id_serial_seq OWNED BY public.caixa.id_serial;


--
-- TOC entry 278 (class 1259 OID 41702)
-- Name: configuracoes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.configuracoes (
    id uuid,
    pdv_numero integer,
    empresa integer,
    setor integer,
    razao_social character varying(60),
    nome_fantasia character varying(60),
    cnpj character varying(14),
    inscricao_estadual character varying(14),
    inscricao_municipal character varying(14),
    cnae character varying(7),
    codigo_regime_tributacao integer,
    logradouro character varying(60),
    complemento character varying(60),
    numero character varying(10),
    bairro character varying(60),
    municipio character varying(60),
    cod_municipio integer,
    uf character(2),
    cep character varying(9),
    fone character varying(14),
    mensagem_venda character varying(250),
    exibir_valor_fechamento_caixa character(1),
    exibir_valor_sangria character(1),
    solicita_senha_venda character(1),
    identifica_vendedor character(1),
    diferenca_abastecimento numeric(15,2),
    quantidade_maxima_gerada numeric(15,2),
    quantidade_maxima_abastecimento integer,
    tipo_estabelecimento character(1),
    tipo_busca_abastecimento integer,
    tipo_identificacao_cliente integer,
    tipo_identificacao_fidelidade integer,
    tipo_identificacao_usuario integer,
    desconto_fechamento character(1),
    imprime_gerencial_fidelidade character(1),
    imprime_gerencial_promocao character(1),
    imprime_espelho_completo character(1),
    imprime_espelho_vencimento character(1),
    imprime_recibo_espelho character(1),
    imprime_rel_fechamento_caixa character(1),
    imprime_rel_fechamento_turno character(1),
    imprime_descricao_grade character(1),
    imprime_espelho_sangria character(1),
    imprime_espelho_suprimento character(1),
    abre_venda_consulta_produto character(1),
    codigo_balanca character(1),
    vias_espelho integer,
    pedido_agrupado character(1),
    pre_venda_pagamento character(1),
    alterar_pre_venda character(1),
    atualizacao timestamp without time zone,
    versao_retaguarda character varying(20),
    senha_usuario_ativo character(1),
    efetuar_sangria_usuario character(1),
    vlr_max_nfce numeric(15,2),
    exibir_limite_cliente character(1),
    emissao_direta_nf_pj character(1),
    lista_todos_abastecimentos_pdv character(1),
    id_token character varying(40),
    token_csc text,
    controle_estoque_combustivel character(1)
);


ALTER TABLE public.configuracoes OWNER TO postgres;

--
-- TOC entry 228 (class 1259 OID 33540)
-- Name: fidelidade_tabelas; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.fidelidade_tabelas (
    id integer NOT NULL,
    tabela_id integer
);


ALTER TABLE public.fidelidade_tabelas OWNER TO postgres;

--
-- TOC entry 229 (class 1259 OID 33543)
-- Name: fidelidades; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.fidelidades (
    id integer NOT NULL,
    status character(1),
    cpf_cnpj character varying(14),
    inscricao_estadual character varying(14),
    inscricao_municipal character varying(14),
    nome_fantasia character varying(80),
    razao_social character varying(80),
    logradouro character varying(60),
    complemento character varying(60),
    numero character varying(10),
    bairro character varying(60),
    municipio character varying(60),
    cod_municipio integer,
    uf character(2),
    cep character varying(9),
    identificacao character varying(20),
    desconto_venda numeric(15,2)
);


ALTER TABLE public.fidelidades OWNER TO postgres;

--
-- TOC entry 230 (class 1259 OID 33548)
-- Name: formas_pagamento; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.formas_pagamento (
    id integer NOT NULL,
    numero integer,
    tipo_pagamento integer,
    descricao character varying(60),
    valor_aviso_sangria numeric(15,2),
    somente_cadastrados character(1),
    permite_troco character(1),
    permite_desconto character(1),
    permite_acrescimo character(1),
    dados_cheque character(1),
    dados_tef character(1),
    maximo_parcelas integer,
    tef_rede character varying(20),
    tef_operacao integer,
    voucher character(1),
    ignora_limite_troco character(1),
    solicita_vencimento character(1),
    valida_limite_credito character(1),
    espelho character(1),
    dias_vencimento character(1),
    tipo_venda character(2),
    tabela_id integer,
    permite_cheque_troco character(1),
    permite_deposito_troco character(1),
    percentual_maximo_troco numeric(15,3),
    percentual_desconto numeric(15,2),
    percentual_maximo_desconto numeric(15,2),
    venda_mobile character(1),
    troco_em_deposito character(1),
    vendas_com_juros_mobile character(1)
);


ALTER TABLE public.formas_pagamento OWNER TO postgres;

--
-- TOC entry 231 (class 1259 OID 33551)
-- Name: grades_itens; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.grades_itens (
    id integer,
    grade_id integer,
    codigo character varying(20),
    descricao character varying(40)
);


ALTER TABLE public.grades_itens OWNER TO postgres;

--
-- TOC entry 232 (class 1259 OID 33554)
-- Name: inutilizacao; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.inutilizacao (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    setor_id integer,
    data_hora timestamp without time zone,
    modelo integer,
    numero integer,
    serie integer,
    protocolo character varying(40),
    motivo character varying(200),
    sincronizado character(1)
);


ALTER TABLE public.inutilizacao OWNER TO postgres;

--
-- TOC entry 233 (class 1259 OID 33558)
-- Name: logs; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.logs (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    pdv uuid,
    usuario_id integer,
    data_hora timestamp without time zone,
    tipo integer,
    historico character varying(255),
    sincronizado character(1)
);


ALTER TABLE public.logs OWNER TO postgres;

--
-- TOC entry 234 (class 1259 OID 33562)
-- Name: lotes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.lotes (
    id integer,
    numero character varying(40)
);


ALTER TABLE public.lotes OWNER TO postgres;

--
-- TOC entry 235 (class 1259 OID 33565)
-- Name: municipios; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.municipios (
    codigo integer,
    uf integer,
    descricao character varying(60)
);


ALTER TABLE public.municipios OWNER TO postgres;

--
-- TOC entry 215 (class 1259 OID 16830)
-- Name: numeracao_nfce; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.numeracao_nfce (
    pdv uuid,
    numero integer
);


ALTER TABLE public.numeracao_nfce OWNER TO postgres;

--
-- TOC entry 216 (class 1259 OID 16833)
-- Name: numeracao_nfe; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.numeracao_nfe (
    pdv uuid,
    numero integer
);


ALTER TABLE public.numeracao_nfe OWNER TO postgres;

--
-- TOC entry 236 (class 1259 OID 33568)
-- Name: parametros; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.parametros (
    pdv uuid,
    chave character varying(80),
    valor text,
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL
);


ALTER TABLE public.parametros OWNER TO postgres;

--
-- TOC entry 237 (class 1259 OID 33573)
-- Name: parceiro_dependentes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.parceiro_dependentes (
    id integer NOT NULL,
    status character(1),
    parceiro_id integer,
    nome character varying(80),
    rfid character varying(20),
    limite_disponivel numeric(15,2)
);


ALTER TABLE public.parceiro_dependentes OWNER TO postgres;

--
-- TOC entry 240 (class 1259 OID 33582)
-- Name: parceiro_formas_pagameto; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.parceiro_formas_pagameto (
    id integer NOT NULL,
    parceiro_id integer,
    forma_pagamento_id integer,
    tabela_id integer
);


ALTER TABLE public.parceiro_formas_pagameto OWNER TO postgres;

--
-- TOC entry 238 (class 1259 OID 33576)
-- Name: parceiro_frotas; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.parceiro_frotas (
    id integer,
    status character(1),
    parceiro_id integer,
    veiculo character varying(20),
    placa character varying(8)
);


ALTER TABLE public.parceiro_frotas OWNER TO postgres;

--
-- TOC entry 239 (class 1259 OID 33579)
-- Name: parceiro_tabelas; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.parceiro_tabelas (
    id integer NOT NULL,
    status character(1),
    parceiro_id integer,
    tabela_id integer
);


ALTER TABLE public.parceiro_tabelas OWNER TO postgres;

--
-- TOC entry 241 (class 1259 OID 33585)
-- Name: parceiro_tabelas_formas_pagamento; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.parceiro_tabelas_formas_pagamento (
    id integer NOT NULL,
    status character(1),
    parceiro_id integer,
    forma_pagamento_id integer,
    tabela_id integer
);


ALTER TABLE public.parceiro_tabelas_formas_pagamento OWNER TO postgres;

--
-- TOC entry 242 (class 1259 OID 33588)
-- Name: parceiros; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.parceiros (
    id integer NOT NULL,
    status character(1),
    cpf_cnpj character varying(14),
    inscricao_estadual character varying(14),
    inscricao_municipal character varying(14),
    nome_fantasia character varying(80),
    razao_social character varying(80),
    logradouro character varying(60),
    complemento character varying(60),
    numero character varying(10),
    bairro character varying(60),
    municipio character varying(60),
    cod_municipio integer,
    uf character(2),
    cep character varying(9),
    identificacao character varying(20),
    requer_placa character(1),
    requer_km character(1),
    requer_condutor character(1),
    desconto_venda numeric(15,2),
    limite_disponivel numeric(15,2),
    email character varying(60),
    rfid character varying(20),
    ie_situacao character(1)
);


ALTER TABLE public.parceiros OWNER TO postgres;

--
-- TOC entry 243 (class 1259 OID 33593)
-- Name: pos; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.pos (
    id integer NOT NULL,
    serial character varying(30),
    pdv uuid
);


ALTER TABLE public.pos OWNER TO postgres;

--
-- TOC entry 244 (class 1259 OID 33596)
-- Name: pre_venda_pagamentos; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.pre_venda_pagamentos (
    id integer NOT NULL,
    pre_venda_id uuid,
    processado character(1),
    forma_pagamento_id integer,
    vencimento date,
    desconto numeric(15,3),
    acrescimo numeric(15,3),
    total numeric(15,3)
);


ALTER TABLE public.pre_venda_pagamentos OWNER TO postgres;

--
-- TOC entry 245 (class 1259 OID 33599)
-- Name: produtos; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.produtos (
    id integer NOT NULL,
    tipo character(1),
    categoria character(1),
    codigo character varying(20),
    tipo_codigo integer,
    descricao character varying(60),
    descricao_resumida character varying(25),
    gtin_comercial character varying(14),
    gtin_tributacao character varying(14),
    codigo_auxiliar character varying(20),
    unidade_comercial character varying(6),
    unidade_tributacao character varying(6),
    quantidade_comercial numeric(15,3),
    quantidade_tributacao numeric(15,3),
    indicador_arredondamento character(1),
    indicador_producao character(1),
    fracionado character(1),
    pesado_caixa character(1),
    etiqueta_balanca character(1),
    cst character(3),
    cfop integer,
    aliquota numeric(15,2),
    cod_ticket character(5),
    ncm character varying(10),
    ncm_excecao character(3),
    cest character varying(7),
    imposto_chave character varying(6),
    imposto_aliquota_importacao numeric(15,3),
    imposto_aliquota_federal numeric(15,3),
    imposto_aliquota_estadual numeric(15,3),
    imposto_aliquota_municipal numeric(15,3),
    codigo_anp integer,
    controla_numero_serie character(1),
    controla_lote character(1),
    solicita_vendedor character(1),
    grade_id integer,
    setor_impressao_1 integer,
    setor_impressao_2 integer,
    setor_impressao_3 integer,
    setor_impressao_4 integer,
    exclusivo_kit character(1),
    descricao_anp character varying(100),
    cst_pis character(3),
    cst_cofins character(3),
    aliquota_pis numeric(15,3),
    aliquota_cofins numeric(15,3),
    tipo_combustivel integer,
    predbcefet numeric(15,3),
    picmsefet numeric(15,3),
    pfcpstret numeric(15,3),
    pfcpst numeric(15,3),
    pfcp numeric(15,3),
    modbc numeric(15,3),
    modbcst numeric(15,3),
    pmvast numeric(15,3),
    predbcst numeric(15,3),
    picmsst numeric(15,3),
    predbc numeric(15,3),
    pglp numeric(15,3),
    pgnn numeric(15,3),
    pgni numeric(15,3),
    vpart numeric(15,3),
    observacao text
);


ALTER TABLE public.produtos OWNER TO postgres;

--
-- TOC entry 246 (class 1259 OID 33604)
-- Name: produtos_codigos; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.produtos_codigos (
    id integer,
    status character(1),
    produto_id integer,
    codigo character varying(40)
);


ALTER TABLE public.produtos_codigos OWNER TO postgres;

--
-- TOC entry 247 (class 1259 OID 33607)
-- Name: produtos_kits; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.produtos_kits (
    id integer,
    kit_id integer,
    produto_id integer,
    quantidade numeric(15,3),
    tabela_preco_id integer
);


ALTER TABLE public.produtos_kits OWNER TO postgres;

--
-- TOC entry 248 (class 1259 OID 33610)
-- Name: produtos_promocoes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.produtos_promocoes (
    produto_id integer,
    valor numeric(15,3),
    validade date
);


ALTER TABLE public.produtos_promocoes OWNER TO postgres;

--
-- TOC entry 249 (class 1259 OID 33613)
-- Name: produtos_series; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.produtos_series (
    id integer NOT NULL,
    produto_id integer,
    serie character varying(40),
    venda_id uuid
);


ALTER TABLE public.produtos_series OWNER TO postgres;

--
-- TOC entry 250 (class 1259 OID 33616)
-- Name: produtos_setores; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.produtos_setores (
    id integer NOT NULL,
    setor_id integer,
    produto_id integer
);


ALTER TABLE public.produtos_setores OWNER TO postgres;

--
-- TOC entry 251 (class 1259 OID 33619)
-- Name: promocoes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.promocoes (
    id integer NOT NULL,
    status character(1),
    produto_id integer,
    quantidade numeric(15,3),
    a_partir numeric(15,3),
    total numeric(15,2),
    desconto numeric(15,2),
    tipo character(2),
    forma_pagamento_id integer,
    conteudo text
);


ALTER TABLE public.promocoes OWNER TO postgres;

--
-- TOC entry 253 (class 1259 OID 33625)
-- Name: sangria_suprimento; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.sangria_suprimento (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    id_serial integer NOT NULL,
    pdv uuid,
    setor_id integer,
    turno_id uuid,
    turno_posto_id uuid,
    tipo character(2),
    forma_pagamento_id integer,
    valor numeric(15,2),
    data_hora timestamp without time zone,
    usuario_id integer,
    historico character varying(80),
    sincronizado character(1)
);


ALTER TABLE public.sangria_suprimento OWNER TO postgres;

--
-- TOC entry 252 (class 1259 OID 33624)
-- Name: sangria_suprimento_id_serial_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.sangria_suprimento_id_serial_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.sangria_suprimento_id_serial_seq OWNER TO postgres;

--
-- TOC entry 3782 (class 0 OID 0)
-- Dependencies: 252
-- Name: sangria_suprimento_id_serial_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.sangria_suprimento_id_serial_seq OWNED BY public.sangria_suprimento.id_serial;


--
-- TOC entry 254 (class 1259 OID 33630)
-- Name: setores; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.setores (
    id integer NOT NULL,
    descricao character varying(40),
    impressora character varying(80)
);


ALTER TABLE public.setores OWNER TO postgres;

--
-- TOC entry 255 (class 1259 OID 33633)
-- Name: tabela_preco_itens; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tabela_preco_itens (
    id integer NOT NULL,
    tabela_preco_id integer,
    produto_id integer,
    valor_comercial numeric(15,3),
    valor_tributacao numeric(15,3)
);


ALTER TABLE public.tabela_preco_itens OWNER TO postgres;

--
-- TOC entry 256 (class 1259 OID 33636)
-- Name: tabela_precos; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tabela_precos (
    id integer NOT NULL,
    padrao character(1),
    status character(1),
    descricao character varying(40),
    exclusiva_cliente character(1)
);


ALTER TABLE public.tabela_precos OWNER TO postgres;

--
-- TOC entry 257 (class 1259 OID 33639)
-- Name: tanques; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tanques (
    id integer NOT NULL,
    numero integer,
    gtin character varying(14),
    descricao character varying(60),
    capacidade numeric(15,3),
    estoque numeric(15,3)
);


ALTER TABLE public.tanques OWNER TO postgres;

--
-- TOC entry 258 (class 1259 OID 33642)
-- Name: tanques_medicoes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tanques_medicoes (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    tanque_numero integer,
    descricao character varying(60),
    volume_expansao numeric(15,3),
    estoque_atual numeric(15,3),
    data_hora timestamp without time zone,
    sincronizado character(1) DEFAULT 'F'::bpchar
);


ALTER TABLE public.tanques_medicoes OWNER TO postgres;

--
-- TOC entry 262 (class 1259 OID 33654)
-- Name: turno_postos; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.turno_postos (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    id_serial integer NOT NULL,
    pdv uuid,
    setor_id integer,
    status character(1),
    numero integer NOT NULL,
    abertura_usuario_id integer NOT NULL,
    fechamento_usuario_id integer,
    data_hora_abertura timestamp without time zone,
    data_hora_fechamento timestamp without time zone,
    sincronizado_abertura character(1),
    sincronizado_fechamento character(1)
);


ALTER TABLE public.turno_postos OWNER TO postgres;

--
-- TOC entry 261 (class 1259 OID 33653)
-- Name: turno_postos_id_serial_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.turno_postos_id_serial_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.turno_postos_id_serial_seq OWNER TO postgres;

--
-- TOC entry 3783 (class 0 OID 0)
-- Dependencies: 261
-- Name: turno_postos_id_serial_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.turno_postos_id_serial_seq OWNED BY public.turno_postos.id_serial;


--
-- TOC entry 260 (class 1259 OID 33648)
-- Name: turnos; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.turnos (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    id_serial integer NOT NULL,
    pdv uuid,
    setor_id integer,
    status character(1),
    numero integer NOT NULL,
    suprimento numeric(15,2),
    data_hora_abertura timestamp without time zone,
    data_hora_reabertura timestamp without time zone,
    data_hora_fechamento timestamp without time zone,
    tipo character(1),
    venda_bruta numeric(15,2),
    cancelamentos numeric(15,2),
    descontos numeric(15,2),
    acrescimos numeric(15,2),
    abertura_usuario_id integer NOT NULL,
    fechamento_usuario_id integer,
    sincronizado_abertura character(1),
    sincronizado_fechamento character(1)
);


ALTER TABLE public.turnos OWNER TO postgres;

--
-- TOC entry 259 (class 1259 OID 33647)
-- Name: turnos_id_serial_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.turnos_id_serial_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.turnos_id_serial_seq OWNER TO postgres;

--
-- TOC entry 3784 (class 0 OID 0)
-- Dependencies: 259
-- Name: turnos_id_serial_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.turnos_id_serial_seq OWNED BY public.turnos.id_serial;


--
-- TOC entry 263 (class 1259 OID 33659)
-- Name: usuario_permissoes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.usuario_permissoes (
    id integer NOT NULL,
    usuario_id integer,
    cancela_venda_aberta character(1),
    cancela_venda_fechada character(1),
    cancela_item character(1),
    desconto_item character(1),
    desconto_fechamento character(1),
    desconto_fechamento_pv character(1),
    acrescimo_item character(1),
    acrescimo_fechamento character(1),
    acrescimo_fechamento_pv character(1),
    cliente_limite character(1),
    cliente_bloqueado character(1),
    cliente_forma_pagamento character(1),
    sangria character(1),
    suprimento character(1),
    abertura_turno character(1),
    fechamento_turno character(1),
    reabertura_turno character(1),
    afericao character(1),
    lista_todos_abastecimentos character(1),
    operacoes_tef character(1),
    limite_desconto_acrescimo character(1),
    sangria_lancamento_saida character(1),
    desmembramento character(1),
    libera_troco_maximo character(1)
);


ALTER TABLE public.usuario_permissoes OWNER TO postgres;

--
-- TOC entry 264 (class 1259 OID 33662)
-- Name: usuarios; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.usuarios (
    id integer NOT NULL,
    status character(1),
    nome character varying(80),
    tentativas_invalidas integer,
    login character varying(60),
    senha text,
    rfid character varying(16),
    rfid_debito character varying(16),
    rfid_credito character varying(16),
    digital text,
    cartao_magnetico character varying(60),
    perc_max_desc_acres_item numeric(15,3) DEFAULT 0,
    valor_max_desc_acres_item numeric(15,3) DEFAULT 0,
    perc_max_desc_acres_subtotal numeric(15,3) DEFAULT 0,
    valor_max_desc_acres_subtotal numeric(15,3) DEFAULT 0
);


ALTER TABLE public.usuarios OWNER TO postgres;

--
-- TOC entry 266 (class 1259 OID 33672)
-- Name: venda_cheque_trocos; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.venda_cheque_trocos (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    id_serial integer NOT NULL,
    status character(1),
    empresa_id integer,
    caixa_id integer,
    venda_id uuid NOT NULL,
    forma_pagamento_id integer,
    sequencia smallint,
    cliente_id integer,
    cheque_troco_id integer,
    agencia character varying(30),
    conta character varying(30),
    data_emissao date,
    numero_cheque integer,
    valor numeric(15,2),
    liberado_venda character(1)
);


ALTER TABLE public.venda_cheque_trocos OWNER TO postgres;

--
-- TOC entry 265 (class 1259 OID 33671)
-- Name: venda_cheque_trocos_id_serial_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.venda_cheque_trocos_id_serial_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.venda_cheque_trocos_id_serial_seq OWNER TO postgres;

--
-- TOC entry 3785 (class 0 OID 0)
-- Dependencies: 265
-- Name: venda_cheque_trocos_id_serial_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.venda_cheque_trocos_id_serial_seq OWNED BY public.venda_cheque_trocos.id_serial;


--
-- TOC entry 268 (class 1259 OID 33678)
-- Name: venda_cheques; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.venda_cheques (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    id_serial integer NOT NULL,
    venda_id uuid NOT NULL,
    forma_pagamento_id integer,
    sequencia integer,
    compensacao character varying(3),
    banco integer,
    agencia integer,
    agencia_digito character(1),
    conta character varying(20),
    conta_digito character(1),
    numero integer,
    numero_digito character(1),
    valor numeric(15,2),
    vencimento date,
    tipo_pessoa character(1),
    titular character varying(60),
    cpf_cnpj character varying(14),
    telefone character varying(40)
);


ALTER TABLE public.venda_cheques OWNER TO postgres;

--
-- TOC entry 267 (class 1259 OID 33677)
-- Name: venda_cheques_id_serial_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.venda_cheques_id_serial_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.venda_cheques_id_serial_seq OWNER TO postgres;

--
-- TOC entry 3786 (class 0 OID 0)
-- Dependencies: 267
-- Name: venda_cheques_id_serial_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.venda_cheques_id_serial_seq OWNED BY public.venda_cheques.id_serial;


--
-- TOC entry 270 (class 1259 OID 33684)
-- Name: venda_deposito_trocos; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.venda_deposito_trocos (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    id_serial integer NOT NULL,
    forma_pagamento_id integer,
    cliente_id integer,
    venda_id uuid NOT NULL,
    banco character varying(40),
    agencia character varying(4),
    agencia_digito character varying(1),
    conta character varying(20),
    conta_digito character varying(1),
    tipo_conta character varying(1),
    operacao character varying(10),
    favorecido character varying(80),
    cpf_cnpj character varying(14),
    telefone character varying(20),
    depositante character varying(80),
    valor numeric(15,2),
    observacao character varying(80),
    data date
);


ALTER TABLE public.venda_deposito_trocos OWNER TO postgres;

--
-- TOC entry 269 (class 1259 OID 33683)
-- Name: venda_deposito_trocos_id_serial_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.venda_deposito_trocos_id_serial_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.venda_deposito_trocos_id_serial_seq OWNER TO postgres;

--
-- TOC entry 3787 (class 0 OID 0)
-- Dependencies: 269
-- Name: venda_deposito_trocos_id_serial_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.venda_deposito_trocos_id_serial_seq OWNED BY public.venda_deposito_trocos.id_serial;


--
-- TOC entry 272 (class 1259 OID 33712)
-- Name: venda_itens; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.venda_itens (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    id_serial integer NOT NULL,
    status character(1),
    venda_id uuid NOT NULL,
    sequencia integer,
    pre_venda character(1),
    produto_id integer,
    produto_gtin character varying(20),
    quantidade numeric(15,4),
    valor_comercial numeric(15,2),
    valor_tributacao numeric(15,2),
    subtotal numeric(15,2),
    desconto numeric(15,2),
    acrescimo numeric(15,2),
    total numeric(15,2),
    desconto_fechamento numeric(15,2) DEFAULT 0,
    acrescimo_fechamento numeric(15,2) DEFAULT 0,
    total_fechamento numeric(15,2) DEFAULT 0,
    cfop integer,
    cst character(3),
    icms_aliquota numeric(15,3),
    icms_valor numeric(15,3),
    total_tributos numeric(15,2) DEFAULT 0,
    total_tributos_importacao numeric(15,3) DEFAULT 0,
    total_tributos_federal numeric(15,3) DEFAULT 0,
    total_tributos_estadual numeric(15,3) DEFAULT 0,
    total_tributos_municipal numeric(15,3) DEFAULT 0,
    abastecimento_id uuid,
    bico_id integer,
    encerrante_inicial numeric(15,3) DEFAULT 0,
    encerrante_final numeric(15,3) DEFAULT 0,
    rfid_vendedor character varying(16),
    rfid_cliente character varying(16),
    setor_id integer,
    grade_item_id integer,
    grade_codigo character varying(20),
    grade_descricao character varying(40),
    lote_id integer,
    produto_serie_id integer,
    tabela_preco_id integer,
    tabela_padrao character(1),
    vendedor_id integer,
    icmsst_valor numeric(15,3),
    icmsst_aliquota numeric(15,3),
    predbcefet numeric(15,3),
    picmsefet numeric(15,3),
    vicmsefet numeric(15,3),
    pfcpstret numeric(15,3),
    vfcpstret numeric(15,3),
    pfcpst numeric(15,3),
    vfcpst numeric(15,3),
    pfcp numeric(15,3),
    vfcp numeric(15,3),
    modbc numeric(15,3),
    modbcst numeric(15,3),
    pmvast numeric(15,3),
    predbcst numeric(15,3),
    predbc numeric(15,3),
    cst_pis character(2),
    pis_valor numeric(15,3),
    pis_aliquota numeric(15,3),
    cst_cofins character(2),
    cofins_valor numeric(15,3),
    cofins_aliquota numeric(15,3)
);


ALTER TABLE public.venda_itens OWNER TO postgres;

--
-- TOC entry 271 (class 1259 OID 33711)
-- Name: venda_itens_id_serial_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.venda_itens_id_serial_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.venda_itens_id_serial_seq OWNER TO postgres;

--
-- TOC entry 3788 (class 0 OID 0)
-- Dependencies: 271
-- Name: venda_itens_id_serial_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.venda_itens_id_serial_seq OWNED BY public.venda_itens.id_serial;


--
-- TOC entry 274 (class 1259 OID 33728)
-- Name: venda_pagamentos; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.venda_pagamentos (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    id_serial integer NOT NULL,
    sequencia integer,
    venda_id uuid NOT NULL,
    forma_pagamento_id integer,
    subtotal numeric(15,2),
    desconto numeric(15,2),
    acrescimo numeric(15,2),
    total numeric(15,2),
    recebido numeric(15,2),
    troco numeric(15,2),
    vinculado character(1),
    data_vencimento date,
    vendedor_id integer,
    voucher_id integer,
    tef character(1),
    tef_pos character(1),
    tef_terminal character varying(10),
    tef_cnpj character varying(14),
    tef_nsu character varying(20),
    tef_rede character varying(20),
    tef_bandeira character varying(20),
    tef_operacao character(1),
    tef_parcelas integer,
    tef_tipo_parcelamento character(1),
    tef_tipo_transacao integer,
    tef_desconto numeric(15,2),
    tef_saque numeric(15,2),
    tef_sitef_instituicao character varying(5),
    tef_sitef_bandeira character varying(5),
    tef_via_estabelecimento text,
    tef_via_cliente text
);


ALTER TABLE public.venda_pagamentos OWNER TO postgres;

--
-- TOC entry 273 (class 1259 OID 33727)
-- Name: venda_pagamentos_id_serial_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.venda_pagamentos_id_serial_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.venda_pagamentos_id_serial_seq OWNER TO postgres;

--
-- TOC entry 3789 (class 0 OID 0)
-- Dependencies: 273
-- Name: venda_pagamentos_id_serial_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.venda_pagamentos_id_serial_seq OWNED BY public.venda_pagamentos.id_serial;


--
-- TOC entry 280 (class 1259 OID 41765)
-- Name: vendas; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.vendas (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    id_serial integer NOT NULL,
    status character(1),
    tipo character(1),
    setor_id integer,
    pdv uuid,
    turno_id uuid,
    turno_posto_id uuid,
    data_hora timestamp without time zone,
    movimento date,
    automatico character(1),
    subtotal numeric(15,2) DEFAULT 0,
    desconto numeric(15,2) DEFAULT 0,
    acrescimo numeric(15,2) DEFAULT 0,
    valor_total numeric(15,2) DEFAULT 0,
    troco numeric(15,2) DEFAULT 0,
    desconto_itens numeric(15,2) DEFAULT 0,
    acrescimo_itens numeric(15,2) DEFAULT 0,
    taxa_servico numeric(15,2) DEFAULT 0,
    parceiro_id integer,
    dependente_id integer,
    frota_id integer,
    fidelidade_id integer,
    tabela_preco_id integer,
    cpf_cnpj character varying(14),
    inscricao_estadual character varying(14),
    ie_situacao character(1),
    nome_fantasia character varying(80),
    razao_social character varying(80),
    logradouro character varying(60),
    complemento character varying(60),
    numero character varying(10),
    bairro character varying(60),
    municipio character varying(60),
    cod_municipio integer,
    uf character(2),
    cep character varying(9),
    telefone character varying(11),
    email character varying(200),
    km integer,
    placa character varying(7),
    condutor character varying(40),
    unidade_consumidora character varying(10),
    horimetro character varying(10),
    pre_venda_id uuid,
    pre_venda_numero character varying(20),
    nfe_contingencia character(1),
    nfe_offline character(1),
    nfe_aguardando_envio character(1),
    nfe_tentativa_envio integer,
    nfe_tipo integer DEFAULT 0,
    nfe_data timestamp without time zone,
    nfe_numero integer DEFAULT 0,
    nfe_serie integer DEFAULT 0,
    nfe_chave character varying(60),
    nfe_protocolo character varying(60),
    nfe_recibo character varying(60),
    nfe_retorno text,
    nfe_dados_adicionais text,
    nfe_xml text,
    nfe_arquivo text,
    nfe_inconsistente character(1),
    nfe_regerar character(1),
    nfe_cancelamento_data timestamp without time zone,
    nfe_cancelamento_motivo character varying(250),
    nfe_cancelamento_protocolo character varying(60),
    nfe_cancelamento_xml text,
    nfe_cancelada character(1) DEFAULT 'F'::bpchar,
    nfe_inutilizacao_data timestamp without time zone,
    nfe_inutilizacao_protocolo character varying(60),
    nfe_inutilizada character(1) DEFAULT 'F'::bpchar,
    nfe_rejeicao text,
    voucher_id integer,
    usuario_id integer,
    vendedor_id integer,
    usuario_pre_venda_id integer,
    finalizada character(1),
    estorno character(1) DEFAULT 'F'::bpchar,
    atualiza_retaguarda character(1) DEFAULT 'F'::bpchar,
    sincronizado character(1) DEFAULT 'F'::bpchar
);


ALTER TABLE public.vendas OWNER TO postgres;

--
-- TOC entry 279 (class 1259 OID 41764)
-- Name: vendas_id_serial_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.vendas_id_serial_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.vendas_id_serial_seq OWNER TO postgres;

--
-- TOC entry 3790 (class 0 OID 0)
-- Dependencies: 279
-- Name: vendas_id_serial_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.vendas_id_serial_seq OWNED BY public.vendas.id_serial;


--
-- TOC entry 275 (class 1259 OID 33735)
-- Name: vendedores; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.vendedores (
    id integer NOT NULL,
    codigo integer NOT NULL,
    nome character varying(80)
);


ALTER TABLE public.vendedores OWNER TO postgres;

--
-- TOC entry 276 (class 1259 OID 33738)
-- Name: versoes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.versoes (
    pdv uuid,
    retaguarda integer,
    concentrador integer,
    banco integer,
    client integer
);


ALTER TABLE public.versoes OWNER TO postgres;

--
-- TOC entry 277 (class 1259 OID 33741)
-- Name: voucher; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.voucher (
    id integer,
    cliente_id integer,
    valor numeric(15,2),
    numero character varying(32),
    impresso character(1)
);


ALTER TABLE public.voucher OWNER TO postgres;

--
-- TOC entry 3406 (class 2604 OID 33500)
-- Name: abastecimentos id_serial; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.abastecimentos ALTER COLUMN id_serial SET DEFAULT nextval('public.abastecimentos_id_serial_seq'::regclass);


--
-- TOC entry 3410 (class 2604 OID 33511)
-- Name: afericoes id_serial; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.afericoes ALTER COLUMN id_serial SET DEFAULT nextval('public.afericoes_id_serial_seq'::regclass);


--
-- TOC entry 3414 (class 2604 OID 33524)
-- Name: bicos_encerrantes id_serial; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bicos_encerrantes ALTER COLUMN id_serial SET DEFAULT nextval('public.bicos_encerrantes_id_serial_seq'::regclass);


--
-- TOC entry 3420 (class 2604 OID 33534)
-- Name: caixa id_serial; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.caixa ALTER COLUMN id_serial SET DEFAULT nextval('public.caixa_id_serial_seq'::regclass);


--
-- TOC entry 3425 (class 2604 OID 33629)
-- Name: sangria_suprimento id_serial; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.sangria_suprimento ALTER COLUMN id_serial SET DEFAULT nextval('public.sangria_suprimento_id_serial_seq'::regclass);


--
-- TOC entry 3431 (class 2604 OID 33658)
-- Name: turno_postos id_serial; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.turno_postos ALTER COLUMN id_serial SET DEFAULT nextval('public.turno_postos_id_serial_seq'::regclass);


--
-- TOC entry 3429 (class 2604 OID 33652)
-- Name: turnos id_serial; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.turnos ALTER COLUMN id_serial SET DEFAULT nextval('public.turnos_id_serial_seq'::regclass);


--
-- TOC entry 3437 (class 2604 OID 33676)
-- Name: venda_cheque_trocos id_serial; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.venda_cheque_trocos ALTER COLUMN id_serial SET DEFAULT nextval('public.venda_cheque_trocos_id_serial_seq'::regclass);


--
-- TOC entry 3439 (class 2604 OID 33682)
-- Name: venda_cheques id_serial; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.venda_cheques ALTER COLUMN id_serial SET DEFAULT nextval('public.venda_cheques_id_serial_seq'::regclass);


--
-- TOC entry 3441 (class 2604 OID 33688)
-- Name: venda_deposito_trocos id_serial; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.venda_deposito_trocos ALTER COLUMN id_serial SET DEFAULT nextval('public.venda_deposito_trocos_id_serial_seq'::regclass);


--
-- TOC entry 3443 (class 2604 OID 33716)
-- Name: venda_itens id_serial; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.venda_itens ALTER COLUMN id_serial SET DEFAULT nextval('public.venda_itens_id_serial_seq'::regclass);


--
-- TOC entry 3455 (class 2604 OID 33732)
-- Name: venda_pagamentos id_serial; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.venda_pagamentos ALTER COLUMN id_serial SET DEFAULT nextval('public.venda_pagamentos_id_serial_seq'::regclass);


--
-- TOC entry 3457 (class 2604 OID 41769)
-- Name: vendas id_serial; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.vendas ALTER COLUMN id_serial SET DEFAULT nextval('public.vendas_id_serial_seq'::regclass);


--
-- TOC entry 3709 (class 0 OID 33496)
-- Dependencies: 218
-- Data for Name: abastecimentos; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.abastecimentos (id, id_serial, status, bloqueado, bico_id, retorno, quantidade, valor_unitario, total, tempo, encerrante_inicial, encerrante_final, data_hora, rfid_frentista, rfid_cliente, pdv, gerado, full_string, data_alteracao, desmembramento_id, sincronizado) FROM stdin;
\.


--
-- TOC entry 3710 (class 0 OID 33503)
-- Dependencies: 219
-- Data for Name: administradoras; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.administradoras (id, descricao, cnpj, bandeira) FROM stdin;
\.


--
-- TOC entry 3712 (class 0 OID 33507)
-- Dependencies: 221
-- Data for Name: afericoes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.afericoes (id, id_serial, pdv, setor_id, turno_id, turno_posto_id, data_hora, abastecimento_id, bico_id, quantidade, usuario_id, sincronizado) FROM stdin;
\.


--
-- TOC entry 3713 (class 0 OID 33512)
-- Dependencies: 222
-- Data for Name: alteracoes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.alteracoes (tabela, alteracao) FROM stdin;
\.


--
-- TOC entry 3714 (class 0 OID 33515)
-- Dependencies: 223
-- Data for Name: bicos; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.bicos (id, status, retorno, numero, bomba, tanque_id, produto_id, gtin, combustivel, tipo_combustivel, altera_preco, valor_unitario, abastecimento_manual, bloqueado, setor_id, tabela_preco_id, cesna_master, cesna_slave, cesna_bomba_logica, cesna_bico_logico, bloqueio_quantidade, valor_unitario_debito, valor_unitario_credito, sincroniza_preco_alterado, sincroniza_preco_data_hora) FROM stdin;
\.


--
-- TOC entry 3716 (class 0 OID 33520)
-- Dependencies: 225
-- Data for Name: bicos_encerrantes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.bicos_encerrantes (id, id_serial, turno_posto_id, bico_id, status, encerrante_inicial, encerrante_final, quantidade_vendida, afericao) FROM stdin;
\.


--
-- TOC entry 3718 (class 0 OID 33530)
-- Dependencies: 227
-- Data for Name: caixa; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.caixa (id, id_serial, pdv, tipo, turno_id, turno_posto_id, venda_id, sangria_suprimento_id, forma_pagamento_id, valor, data_hora, usuario_id, historico, sincronizado) FROM stdin;
\.


--
-- TOC entry 3769 (class 0 OID 41702)
-- Dependencies: 278
-- Data for Name: configuracoes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.configuracoes (id, pdv_numero, empresa, setor, razao_social, nome_fantasia, cnpj, inscricao_estadual, inscricao_municipal, cnae, codigo_regime_tributacao, logradouro, complemento, numero, bairro, municipio, cod_municipio, uf, cep, fone, mensagem_venda, exibir_valor_fechamento_caixa, exibir_valor_sangria, solicita_senha_venda, identifica_vendedor, diferenca_abastecimento, quantidade_maxima_gerada, quantidade_maxima_abastecimento, tipo_estabelecimento, tipo_busca_abastecimento, tipo_identificacao_cliente, tipo_identificacao_fidelidade, tipo_identificacao_usuario, desconto_fechamento, imprime_gerencial_fidelidade, imprime_gerencial_promocao, imprime_espelho_completo, imprime_espelho_vencimento, imprime_recibo_espelho, imprime_rel_fechamento_caixa, imprime_rel_fechamento_turno, imprime_descricao_grade, imprime_espelho_sangria, imprime_espelho_suprimento, abre_venda_consulta_produto, codigo_balanca, vias_espelho, pedido_agrupado, pre_venda_pagamento, alterar_pre_venda, atualizacao, versao_retaguarda, senha_usuario_ativo, efetuar_sangria_usuario, vlr_max_nfce, exibir_limite_cliente, emissao_direta_nf_pj, lista_todos_abastecimentos_pdv, id_token, token_csc, controle_estoque_combustivel) FROM stdin;
\.


--
-- TOC entry 3719 (class 0 OID 33540)
-- Dependencies: 228
-- Data for Name: fidelidade_tabelas; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.fidelidade_tabelas (id, tabela_id) FROM stdin;
\.


--
-- TOC entry 3720 (class 0 OID 33543)
-- Dependencies: 229
-- Data for Name: fidelidades; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.fidelidades (id, status, cpf_cnpj, inscricao_estadual, inscricao_municipal, nome_fantasia, razao_social, logradouro, complemento, numero, bairro, municipio, cod_municipio, uf, cep, identificacao, desconto_venda) FROM stdin;
\.


--
-- TOC entry 3721 (class 0 OID 33548)
-- Dependencies: 230
-- Data for Name: formas_pagamento; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.formas_pagamento (id, numero, tipo_pagamento, descricao, valor_aviso_sangria, somente_cadastrados, permite_troco, permite_desconto, permite_acrescimo, dados_cheque, dados_tef, maximo_parcelas, tef_rede, tef_operacao, voucher, ignora_limite_troco, solicita_vencimento, valida_limite_credito, espelho, dias_vencimento, tipo_venda, tabela_id, permite_cheque_troco, permite_deposito_troco, percentual_maximo_troco, percentual_desconto, percentual_maximo_desconto, venda_mobile, troco_em_deposito, vendas_com_juros_mobile) FROM stdin;
1	1	1	DINHEIRO	500.00	F	T	T	T	F	\N	1	nenhum	0	F	F	\N	\N	F	\N	AV	1	F	\N	0.000	0.00	0.00	T	F	\N
2	2	2	CHEQUE A VISTA	0.00	F	T	F	F	F	\N	1	nenhum	0	F	F	\N	\N	F	\N	CV	0	F	\N	0.000	0.00	0.00	T	F	\N
3	3	2	CHEQUE PRÉ-DATADO	0.00	F	F	F	F	T	\N	1	nenhum	0	F	F	\N	\N	F	\N	CP	0	F	\N	0.000	0.00	0.00	T	F	\N
4	4	10	CARTA FRETE	0.00	\N	\N	F	F	\N	\N	1	nenhum	0	F	F	\N	\N	F	\N	OF	0	F	\N	0.000	0.00	0.00	T	F	\N
5	5	5	NOTA À PRAZO	0.00	T	F	F	F	F	\N	1	nenhum	0	F	F	\N	\N	F	\N	NB	0	F	\N	0.000	0.00	0.00	F	F	\N
6	6	15	BOLETO BANCARIO	0.00	F	F	F	F	F	\N	1	nenhum	0	F	F	\N	\N	F	\N	NB	0	F	\N	0.000	0.00	0.00	T	F	\N
7	7	4	BONIFICAÇÃO	0.00	F	F	F	F	F	\N	17	nenhum	0	F	F	\N	\N	F	\N	AV	0	F	\N	0.000	0.00	0.00	T	F	\N
8	8	3	CARTÃO DE CRÉDITO	0.00	F	F	\N	\N	F	\N	1	dial	1	F	\N	\N	\N	F	\N	CC	0	F	\N	0.000	0.00	0.00	F	\N	F
9	9	4	CARTÃO DE DÉBITO	0.00	F	F	\N	\N	F	\N	1	dial	2	F	\N	\N	\N	F	\N	CC	1	F	\N	0.000	0.00	0.00	F	\N	F
\.


--
-- TOC entry 3722 (class 0 OID 33551)
-- Dependencies: 231
-- Data for Name: grades_itens; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.grades_itens (id, grade_id, codigo, descricao) FROM stdin;
\.


--
-- TOC entry 3723 (class 0 OID 33554)
-- Dependencies: 232
-- Data for Name: inutilizacao; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.inutilizacao (id, setor_id, data_hora, modelo, numero, serie, protocolo, motivo, sincronizado) FROM stdin;
\.


--
-- TOC entry 3724 (class 0 OID 33558)
-- Dependencies: 233
-- Data for Name: logs; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.logs (id, pdv, usuario_id, data_hora, tipo, historico, sincronizado) FROM stdin;
fc7d3382-51b8-437b-846a-682c5c6151d8	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 16:30:24.367	1	Venda No.: 	T
b50bd1b0-bc7e-4d86-8a10-e6699f20bc5a	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:10:52.762	1	Venda No.: 	T
4a3b75ad-489f-4909-aaeb-ea57e539c9aa	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 16:30:44.907	10	Baixa de diferença de abastecimento - Bico (Retorno): 04, Diferença: 3,900	T
be693ce6-a5f9-49b2-865b-a653cb282207	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:11:25.069	10	Baixa de diferença de abastecimento - Bico (Retorno): 05, Diferença: 2,990	T
cffaca54-bfba-40ca-b3d4-f9c6f3f67aa0	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 16:34:51.207	1	Venda No.: 	T
a795d706-4610-4f54-ac24-a4311c06289a	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:14:53.699	1	Venda No.: 	T
fa7e95d9-588d-4dc1-9c80-706caf04ec6f	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 16:35:19.203	10	Baixa de diferença de abastecimento - Bico (Retorno): 04, Diferença: 3,900	T
b16edd58-e8f0-4dc9-a4db-202e4a989c42	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:15:11.893	10	Baixa de diferença de abastecimento - Bico (Retorno): 05, Diferença: 2,990	T
d543c54b-f98c-4a7d-ba96-4f1c745659fa	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 16:38:19.715	1	Venda No.: 	T
efa1de60-0200-49bc-825d-922c33738160	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:18:44.275	1	Venda No.: 	T
15af1e3e-eec4-4e10-afca-d5c7680ed8c8	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 16:38:35.659	10	Baixa de diferença de abastecimento - Bico (Retorno): 04, Diferença: 3,900	T
2d02d29e-46e4-413c-8a18-ebc7c2fa7432	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:19:05.52	10	Baixa de diferença de abastecimento - Bico (Retorno): 05, Diferença: 2,990	T
a0d33e90-cb0e-4c4c-b9f8-715d2d692c61	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 16:39:29.76	10	Baixa de diferença de abastecimento - Bico (Retorno): 05, Diferença: 1,820	T
899af630-ad1e-48dd-ab2b-230b17684fd9	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:29:50.852	1	Venda No.: 	T
071bad0d-b4bb-4464-95c3-d3227b04aaca	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-05-28 14:07:16.658	14	Suspensão de Caixa	T
a2d4453f-39ab-4b54-9166-ab952ebedac1	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:30:12.519	10	Baixa de diferença de abastecimento - Bico (Retorno): 05, Diferença: 2,990	T
91d452c4-90ff-4581-9f1f-4094075d4c34	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-05-28 14:09:58.72	14	Suspensão de Caixa	T
b54b196a-b008-49d3-989f-3249fb3c41ea	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:35:53.015	1	Venda No.: 	T
19fd3e41-3939-46c4-8223-505ca05b2054	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-05-28 14:11:19.986	14	Suspensão de Caixa	T
9a1a546a-2733-475b-845b-591c6bebc474	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:36:11.93	10	Baixa de diferença de abastecimento - Bico (Retorno): 05, Diferença: 2,990	T
d1205bc8-5195-438b-acc9-8b1f4b68294a	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-05-28 14:12:24.855	14	Suspensão de Caixa	T
62e10c37-e723-42c1-bd5f-c11ed8f17543	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:39:44.723	1	Venda No.: 	T
6b7abe72-01f2-412c-92cc-a1447b640ae9	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-05-31 22:42:38.201	1	Venda No.: 	T
806eb06d-9f4d-43ad-8418-2eca6e0a8c34	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:40:18.609	10	Baixa de diferença de abastecimento - Bico (Retorno): 05, Diferença: 2,990	T
2d1f53e8-a9a0-4c9e-8b71-ec7029d3ce78	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-05-31 22:46:47.015	1	Venda No.: 	T
8090e652-c7a4-449e-8024-4a9d5824012d	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:40:42.785	1	Venda No.: 	T
6ae7e658-14bd-4439-aa31-93aba0fb7dd0	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-05-31 22:48:24.541	1	Venda No.: 	T
67794492-fb2a-463b-afac-88cd104e19b6	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:49:07.707	10	Baixa de diferença de abastecimento - Bico (Retorno): 05, Diferença: 2,990	T
8dc17bff-8447-43c5-b468-20f2b91dda9a	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 14:43:08.21	16	Abertura de Turno	T
d8593e00-7db2-4478-a848-24ca9c81227f	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 14:43:46.193	9	Realizacao de Suprimento - Forma de Pagamento: DINHEIRO - Valor: R$ 15,00	T
614fb773-64f6-45b3-8b01-c1450bf81b9c	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 14:51:09.623	12	 Bico No.: 2 Litros: 3,120	T
2f60f928-cf5d-4460-a16a-ddcdd3f24a92	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:04:08.109	10	Baixa de diferença de abastecimento - Bico (Retorno): 05, Diferença: 2,990	T
3dc5e739-284c-46bc-92fc-9505cc1af3fe	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:56:03.244	1	Venda No.: 	T
0d297846-30d0-46ae-a6fe-fd5bdc3cd50b	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 15:56:18.866	10	Baixa de diferença de abastecimento - Bico (Retorno): 05, Diferença: 2,990	T
7a730866-18d9-43fa-abe9-d2f29f877c18	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 16:00:03.131	1	Venda No.: 	T
a00c5b7a-42fe-494b-a838-ce99f5b4ce03	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 16:00:24.127	10	Baixa de diferença de abastecimento - Bico (Retorno): 05, Diferença: 2,990	T
62d643a3-da49-4f1e-be0b-9aafd4bd6559	c4fc16da-7250-41a6-8edb-03c8642f4bc3	0	2024-04-03 16:06:27.993	17	Fechamento de Turno	T
f9ad1052-1622-4fb3-aebd-2af268d88680	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 16:06:28.023	11	Turno Fechado	T
3a095980-724b-4800-be63-1ef77941330d	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 16:13:00.822	16	Abertura de Turno	T
a76476c0-feba-4612-8ed6-e7f393c16b09	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 16:13:17.791	9	Realizacao de Suprimento - Forma de Pagamento: DINHEIRO - Valor: R$ 10,00	T
4e46c4d0-4cc8-4ca7-8028-815dcf44b079	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 16:23:39.317	10	Baixa de diferença de abastecimento - Bico (Retorno): 04, Diferença: 3,900	T
b66f5998-a7fe-4520-a51f-48c3e6ba70cc	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 16:27:33.91	1	Venda No.: 	T
09518cef-94c8-44da-8c4b-6a899d49799b	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-04-03 16:27:54.525	10	Baixa de diferença de abastecimento - Bico (Retorno): 04, Diferença: 3,900	T
ad779465-01f3-4a4f-abef-73757c4fe362	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-05-31 22:59:53.69	16	Abertura de Turno	T
a4379783-9376-4493-b7e3-73875181f6fb	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-05-31 23:01:04.249	1	Venda No.: 	T
d6fd5623-1d52-4402-beb3-312fd0d510af	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-05-31 23:00:56.282	16	Abertura de Turno	T
5bc529f5-fcb2-497b-a881-25cbfe877e5b	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-05-31 23:26:14.511	2	Número da NFC-e: 205 Motivo TESTE 1234567890 12345	T
22a74f55-f860-4c6d-84a8-c74e86bdee62	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-07-08 20:09:10.352	11	Turno Fechado	T
bbd1d0be-0ec6-4e7e-a088-b48c6f9bdc40	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-07-08 20:09:12.103	16	Abertura de Turno	T
bd6cc9f9-76ae-4989-aa71-6513236ea5fc	c4fc16da-7250-41a6-8edb-03c8642f4bc3	0	2024-07-08 20:09:10.126	17	Fechamento de Turno	T
d73b2783-db97-4bf6-b62d-8fb6b1445715	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-07-08 20:09:49.218	9	Realizacao de Suprimento - Forma de Pagamento: DINHEIRO - Valor: R$ 10,00	T
5b3cf861-e384-4a27-94ee-cca061897bf2	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-07-08 20:16:46.233	11	Turno Fechado	T
bfd42b19-9943-49d5-8bae-8e4970789743	c4fc16da-7250-41a6-8edb-03c8642f4bc3	0	2024-07-08 20:16:46.214	17	Fechamento de Turno	T
d4a62070-aef3-4a58-9b08-d0a070673cc6	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-07-08 20:16:47.869	16	Abertura de Turno	T
788c6da7-03ab-4ba0-b9f6-2567cbfa6185	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-07-08 20:18:13.316	9	Realizacao de Suprimento - Forma de Pagamento: DINHEIRO - Valor: R$ 5,00	T
3b7da28d-bf08-4ade-a527-4684f3716a38	c4fc16da-7250-41a6-8edb-03c8642f4bc3	0	2024-07-08 20:27:52.332	17	Fechamento de Turno	T
820308d1-5ce6-44b6-9966-af03fc02c89c	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-07-08 20:27:52.35	11	Turno Fechado	T
0c3ad25c-8631-48c8-b451-439b5d1637d6	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-07-15 21:24:12.454	16	Abertura de Turno	T
81da38d9-5e79-4555-8912-025e3e31cb2b	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-07-15 21:25:21.154	9	Realizacao de Suprimento - Forma de Pagamento: DINHEIRO - Valor: R$ 10,00	T
4092ab91-b33f-4c04-b72a-42b9a4dc7d5e	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-07-15 21:42:36.463	1	Venda No.: 	T
b26c1e96-4ae4-47d3-8663-e736ddac892d	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	2024-07-15 21:54:26.412	1	Venda No.: 	T
\.


--
-- TOC entry 3725 (class 0 OID 33562)
-- Dependencies: 234
-- Data for Name: lotes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.lotes (id, numero) FROM stdin;
\.


--
-- TOC entry 3726 (class 0 OID 33565)
-- Dependencies: 235
-- Data for Name: municipios; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.municipios (codigo, uf, descricao) FROM stdin;
\.


--
-- TOC entry 3706 (class 0 OID 16830)
-- Dependencies: 215
-- Data for Name: numeracao_nfce; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.numeracao_nfce (pdv, numero) FROM stdin;
c4fc16da-7250-41a6-8edb-03c8642f4bc3	96
c4fc16da-7250-41a6-8edb-03c8642f4bc3	97
c4fc16da-7250-41a6-8edb-03c8642f4bc3	98
c4fc16da-7250-41a6-8edb-03c8642f4bc3	99
c4fc16da-7250-41a6-8edb-03c8642f4bc3	100
c4fc16da-7250-41a6-8edb-03c8642f4bc3	101
c4fc16da-7250-41a6-8edb-03c8642f4bc3	102
c4fc16da-7250-41a6-8edb-03c8642f4bc3	103
c4fc16da-7250-41a6-8edb-03c8642f4bc3	104
c4fc16da-7250-41a6-8edb-03c8642f4bc3	105
c4fc16da-7250-41a6-8edb-03c8642f4bc3	106
c4fc16da-7250-41a6-8edb-03c8642f4bc3	107
c4fc16da-7250-41a6-8edb-03c8642f4bc3	108
c4fc16da-7250-41a6-8edb-03c8642f4bc3	109
c4fc16da-7250-41a6-8edb-03c8642f4bc3	110
c4fc16da-7250-41a6-8edb-03c8642f4bc3	137
c4fc16da-7250-41a6-8edb-03c8642f4bc3	138
c4fc16da-7250-41a6-8edb-03c8642f4bc3	139
c4fc16da-7250-41a6-8edb-03c8642f4bc3	140
c4fc16da-7250-41a6-8edb-03c8642f4bc3	141
c4fc16da-7250-41a6-8edb-03c8642f4bc3	142
c4fc16da-7250-41a6-8edb-03c8642f4bc3	143
c4fc16da-7250-41a6-8edb-03c8642f4bc3	144
c4fc16da-7250-41a6-8edb-03c8642f4bc3	145
c4fc16da-7250-41a6-8edb-03c8642f4bc3	146
c4fc16da-7250-41a6-8edb-03c8642f4bc3	147
c4fc16da-7250-41a6-8edb-03c8642f4bc3	148
c4fc16da-7250-41a6-8edb-03c8642f4bc3	149
c4fc16da-7250-41a6-8edb-03c8642f4bc3	150
c4fc16da-7250-41a6-8edb-03c8642f4bc3	151
c4fc16da-7250-41a6-8edb-03c8642f4bc3	152
c4fc16da-7250-41a6-8edb-03c8642f4bc3	153
c4fc16da-7250-41a6-8edb-03c8642f4bc3	154
c4fc16da-7250-41a6-8edb-03c8642f4bc3	155
c4fc16da-7250-41a6-8edb-03c8642f4bc3	156
c4fc16da-7250-41a6-8edb-03c8642f4bc3	157
c4fc16da-7250-41a6-8edb-03c8642f4bc3	158
c4fc16da-7250-41a6-8edb-03c8642f4bc3	159
c4fc16da-7250-41a6-8edb-03c8642f4bc3	160
c4fc16da-7250-41a6-8edb-03c8642f4bc3	161
c4fc16da-7250-41a6-8edb-03c8642f4bc3	162
c4fc16da-7250-41a6-8edb-03c8642f4bc3	163
c4fc16da-7250-41a6-8edb-03c8642f4bc3	164
c4fc16da-7250-41a6-8edb-03c8642f4bc3	165
c4fc16da-7250-41a6-8edb-03c8642f4bc3	166
c4fc16da-7250-41a6-8edb-03c8642f4bc3	167
c4fc16da-7250-41a6-8edb-03c8642f4bc3	168
c4fc16da-7250-41a6-8edb-03c8642f4bc3	169
c4fc16da-7250-41a6-8edb-03c8642f4bc3	170
c4fc16da-7250-41a6-8edb-03c8642f4bc3	171
c4fc16da-7250-41a6-8edb-03c8642f4bc3	172
c4fc16da-7250-41a6-8edb-03c8642f4bc3	173
c4fc16da-7250-41a6-8edb-03c8642f4bc3	174
c4fc16da-7250-41a6-8edb-03c8642f4bc3	175
c4fc16da-7250-41a6-8edb-03c8642f4bc3	176
c4fc16da-7250-41a6-8edb-03c8642f4bc3	177
c4fc16da-7250-41a6-8edb-03c8642f4bc3	178
c4fc16da-7250-41a6-8edb-03c8642f4bc3	179
c4fc16da-7250-41a6-8edb-03c8642f4bc3	180
c4fc16da-7250-41a6-8edb-03c8642f4bc3	181
c4fc16da-7250-41a6-8edb-03c8642f4bc3	182
c4fc16da-7250-41a6-8edb-03c8642f4bc3	183
c4fc16da-7250-41a6-8edb-03c8642f4bc3	184
c4fc16da-7250-41a6-8edb-03c8642f4bc3	185
c4fc16da-7250-41a6-8edb-03c8642f4bc3	186
c4fc16da-7250-41a6-8edb-03c8642f4bc3	187
c4fc16da-7250-41a6-8edb-03c8642f4bc3	188
c4fc16da-7250-41a6-8edb-03c8642f4bc3	189
c4fc16da-7250-41a6-8edb-03c8642f4bc3	190
c4fc16da-7250-41a6-8edb-03c8642f4bc3	191
c4fc16da-7250-41a6-8edb-03c8642f4bc3	192
c4fc16da-7250-41a6-8edb-03c8642f4bc3	193
c4fc16da-7250-41a6-8edb-03c8642f4bc3	194
c4fc16da-7250-41a6-8edb-03c8642f4bc3	195
c4fc16da-7250-41a6-8edb-03c8642f4bc3	196
c4fc16da-7250-41a6-8edb-03c8642f4bc3	197
c4fc16da-7250-41a6-8edb-03c8642f4bc3	198
c4fc16da-7250-41a6-8edb-03c8642f4bc3	199
c4fc16da-7250-41a6-8edb-03c8642f4bc3	200
c4fc16da-7250-41a6-8edb-03c8642f4bc3	201
c4fc16da-7250-41a6-8edb-03c8642f4bc3	202
c4fc16da-7250-41a6-8edb-03c8642f4bc3	203
c4fc16da-7250-41a6-8edb-03c8642f4bc3	204
c4fc16da-7250-41a6-8edb-03c8642f4bc3	205
c4fc16da-7250-41a6-8edb-03c8642f4bc3	206
c4fc16da-7250-41a6-8edb-03c8642f4bc3	207
c4fc16da-7250-41a6-8edb-03c8642f4bc3	208
c4fc16da-7250-41a6-8edb-03c8642f4bc3	209
c4fc16da-7250-41a6-8edb-03c8642f4bc3	210
c4fc16da-7250-41a6-8edb-03c8642f4bc3	211
c4fc16da-7250-41a6-8edb-03c8642f4bc3	212
c4fc16da-7250-41a6-8edb-03c8642f4bc3	213
c4fc16da-7250-41a6-8edb-03c8642f4bc3	214
c4fc16da-7250-41a6-8edb-03c8642f4bc3	215
\.


--
-- TOC entry 3707 (class 0 OID 16833)
-- Dependencies: 216
-- Data for Name: numeracao_nfe; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.numeracao_nfe (pdv, numero) FROM stdin;
\.


--
-- TOC entry 3727 (class 0 OID 33568)
-- Dependencies: 236
-- Data for Name: parametros; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.parametros (pdv, chave, valor, id) FROM stdin;
\N	CONCENTRADOR_TipoMedidor	\N	3c7f37e9-b68d-44f8-b03a-d6080f9d3320
\N	CONCENTRADOR_TempoCaptura	\N	2e44de2f-36c9-404b-b035-2a010478ad3a
\N	CONCENTRADOR_IPMedidor	\N	5349db10-b154-4239-bbad-afbb3cbdae30
\N	CONCENTRADOR_PortaMedidor	\N	81cca3ab-ccd3-4461-a09c-fd4a26cc4e74
\N	CONCENTRADOR_Bombas_Slave2	\N	a34f0d3b-a15a-4f00-b380-603f65defca5
\N	CONCENTRADOR_Bombas_Slave2	\N	88debc50-298a-4e5d-bea1-493166363a3a
\N	CONCENTRADOR_PortaTCP	48001	ba34400c-36a6-415a-a3dc-9f53df4b683d
\N	CONCENTRADOR_CBC_PAF	T	6095bb84-e3e8-4785-a947-73144df62a9a
\N	CONCENTRADOR_RFID	F	76cbcbc6-1504-4cfd-9408-823d4f5c4279
\N	CONCENTRADOR_MultiplosPrecos	F	a99c6295-5c8b-46de-9dd7-2c217faa8a4a
\N	CONCENTRADOR_Fabricante	1	e5a7fcbb-94d6-4a90-9158-85ced1f2f9a8
\N	CONCENTRADOR_PortaSerial	1	f61c8be2-5361-47c5-83f1-533efc1c35b1
\N	CONCENTRADOR_IP	\N	33a06a64-58ae-4974-9825-b987dc447608
\N	CONCENTRADOR_DecimaisEncerrante	2	67e05467-4254-4a36-94f2-65eec37e4c71
\N	MONITOR_ConectaConcentrador	T	9bea53c0-90cb-4c78-9278-16f150ee0a35
\N	PDV_TipoEstabelecimento	P	c527928d-0003-4970-bf5a-33f9d0a77f88
\N	PDV_PreVendaURL	\N	81ddde88-88a5-4aa6-b2a1-5212e75f1099
\N	PDV_AbastecimentoURL	\N	4028c059-ad96-4375-97ed-586046449014
\N	PDV_Teclado	ABNT2	498e5e2f-6fe7-4c93-9435-e5b05b7910dd
\N	PDV_TouchScreen	F	b0a108b7-b00b-4411-ba2c-13dcd09ae3b4
\N	POSTO_Concentrador	T	97cf98ba-ea3a-43e9-93d6-e0327d57d0a7
3ffc33c0-7b54-4418-89fa-c0caf52f1594	PDV_Token		7c1a05c0-fd58-4785-811f-768e27b88af8
\N	POSTO_RFIDLeitor	F	3e2ea662-863f-4301-b8a4-8614821ba7e2
\N	POSTO_RFIDLeitorPorta	\N	9b160123-a669-4abe-94c0-f4a8b3423e2b
\N	POSTO_TempoEsperaBico	3000	648098fc-4768-48b8-97d8-0068fd322cb8
\N	TEF_IgnoraTef	\N	68df487f-a588-40b7-a828-3d3a47d05c47
\N	TEF_SitefIP	\N	85a4bdea-6416-4ad4-8216-e8a4e66d2208
\N	TEF_SitefLoja	\N	f256d45b-0418-4c31-8a64-4c67d4d799a2
\N	TEF_SitefTerminal	\N	4ebcaf68-767f-44e9-8b3c-0a10d4b9b88b
\N	TEF_SitefQrCode	0	072e57d6-b0b0-4919-a78d-37d7b9bd6418
\N	TEF_DtefVias	1	a847d47b-5284-44c4-8e7f-98b24b818fb2
\N	TEF_DtefTerminal	\N	c34ad768-a689-40b8-b9aa-0d4924abda49
\N	TEF_DtefEndereco	\N	0aa6623c-daab-4060-8b04-f4e713b931ae
\N	TEF_GoodCardVias	1	b22c0a67-8748-42fe-89e5-89ec03830618
\N	FID_FidHill	F	69d07600-b925-42bc-8fc6-6af77fdea2d0
\N	FID_FidFidelize	F	06330ebd-d06c-48fb-9792-d08096128c12
\N	FID_FidDotz	F	c1aba30b-4c82-4231-807d-eec5edbf6471
\N	POS_TerminalPOS	F	e94fc870-0fec-4d4c-9aeb-4351653756bc
\N	POS_PortaPOS	\N	94d6416c-ff87-454d-873f-e206bdae7051
\N	LOG_LogTEF	F	426d9cf8-31b3-472f-affc-144800614da6
\N	BALANCA_BalancaModelo	0	a815701f-81ad-4102-9178-68a1971dce95
\N	BALANCA_BalancaPorta	\N	a45ec2c2-66ba-432c-85cb-e8d0e7bf3107
\N	BALANCA_BalancaVelocidade	9600	af8480db-bf92-4f7f-b58b-d6a5119850e1
\N	NFCe_TipoDANFE	1	b67da276-3dd5-450f-9501-cd7efefbef9a
\N	NFCe_IdToken	\N	7919b3ee-745a-4da6-94e7-dcfd09e3b8a0
\N	NFCe_Token	\N	42829669-68ff-48fd-9d17-0f613deb494c
\N	NFCe_WebService	AM	5e6497fe-9e46-49b7-9646-70d91977c030
\N	NFCe_Serie	1	c5f1bc1e-96da-470e-a158-8ae452ced5d2
\N	NFCe_WebServiceTempo	10	d1274303-4247-46f0-975d-128f91cd54d1
\N	NFCe_Impressora	\N	166f2ac9-2a25-4f4b-8697-a2eacabe2bef
\N	NFe_TipoDANFE	0	82f127a5-c409-45ae-a474-6d6c319a594f
\N	NFe_WebService	AM	3e15dffc-2659-4939-a3aa-b996e41ba246
\N	NFe_Serie	1	74346fb7-0169-4c46-96c1-2455681b4557
\N	NFe_Impressora	\N	839a8b8e-7383-4b96-bfed-e82bf9f4aab0
\N	NFe_CFOPConsumoProprio	5949	23aa40f9-74c9-4a90-8af6-905c2e7be2ee
\N	SAT_TipoAmbiente	2	f21a6064-d6ed-44c7-bd84-3d2602e7d1a2
\N	SAT_TipoDANFE	1	46d1da68-1767-4054-badb-e6b5ca7414e6
\N	SAT_Impressora	\N	09ff81d3-fe59-4040-8ba5-acc2928968e1
\N	SAT_dll	\N	882a1818-755a-4d98-9aff-7228aad74e01
\N	SAT_versao	\N	a93a28aa-d1b7-4d7d-b25c-e24a73499883
\N	SAT_CodigoAtivacao	\N	b1b9c1b8-ba8c-48c7-b832-95b267b4cd63
\N	SAT_Assinatura	\N	38b688f1-05d6-4e17-80b5-a6c709b8b312
\N	SAT_Impressora	\N	04662d46-f014-4b80-8bd4-82f0a307eb0f
\N	MFe_ChaveRequisicao	\N	da6b76bc-1670-48f4-be4a-46e94718544b
\N	MFe_ChaveAcesso	\N	763c832a-5a60-4f4e-b268-0843aa109ab1
\N	MFe_Serie	\N	1fe0575f-3f26-4888-b765-cf699e024c43
\N	OUTROS_ProxyHost	\N	3e024b6a-555c-4fd3-b944-f07cc8e278c5
\N	OUTROS_ProxyPorta	\N	1de7b67c-f0ea-4342-b8ad-a73a65a40a01
\N	OUTROS_ProxyUsuario	\N	b7eba913-066c-4e46-9512-8a8275cdffe9
\N	OUTROS_ProxySenha	\N	2e3f4435-2865-4c1e-a4ef-ac0a72378bf1
\N	OUTROS_EmailServidorSMTP	\N	05bc0767-cff3-492c-93e9-48062e15857c
\N	OUTROS_EmailPorta	\N	5f450f8c-082d-4cc9-96d8-dd9bfeb6a9eb
\N	OUTROS_EmailSSL	F	7f7f0709-0d52-4fc0-a6b2-f53c42f786af
\N	OUTROS_EmailTLS	F	266531e0-e6e5-45b0-aaad-c246b2fb1198
\N	OUTROS_EmailUsuario	\N	ac9fe8ff-a6c5-494a-b1b6-8929a3b58bb0
\N	OUTROS_EmailSenha	\N	e063f9fd-926c-4df2-8b1f-7d045a6dd351
\N	OUTROS_INFPorta	\N	825f43ba-3a9f-4ba6-a518-b0bfc24330af
\N	OUTROS_INFVelocidade	9600	2aba10d8-06fa-4791-9839-320b57042b7b
\N	OUTROS_INFFabricante	0	2d55d99e-4d76-4e31-aa91-ddf6d43ea863
\N	OUTROS_EspacoEntreLinhas	0	bb083775-68c0-4022-a3b7-bafebda3b3e5
\N	OUTROS_MargemEsquerda	\N	9cdd1c00-79fd-4072-ac8c-62400cadc962
\N	OUTROS_MargemDireita	\N	e62d9bf9-77d4-4f52-865a-dc0c4f3a92ea
\N	NF_CertificadoNumeroSerie	3EB9A4020742E469	c86a5020-6880-4760-be05-d6e7ba7da155
\N	NF_CertificadoSenha	val040585	e75cf5c1-45bc-4bb6-a0a8-adc67224029f
\N	NF_CertificadoArquivo	C:\\HillPDV64\\certificado.pfx	caaa87f4-f044-4c59-9d6d-7b2ca1743a03
\N	LOG_LogSincronizacao	T	96af59ee-b2e4-4251-bba7-0e6e21b3db88
3ffc33c0-7b54-4418-89fa-c0caf52f1594	PDV_Token_Refresh		a23dbfc6-88d6-4ec5-a544-78fa33c0bdd4
\N	CONCENTRADOR_Porta	COM1	3c0e60b4-ef54-4108-9b8e-2e6a9a944aaf
\N	NFe_IdToken		874f008b-949f-4aa7-8347-b48d0536884d
\N	NFe_Token		bd2fe199-5fe0-4500-8988-7877d99860dd
3ffc33c0-7b54-4418-89fa-c0caf52f1594	PDV_Token_Refresh		5aabd884-1af8-4a27-bb6c-256ee6dcdeb9
\N	NFCe_TipoAmbiente	2	96e12b38-2669-4749-9bde-e54a71395709
3ffc33c0-7b54-4418-89fa-c0caf52f1594	PDV_Token		b267d0d7-e562-4edc-9dee-afab8926a348
\N	PDV_Contingencia	T	d4fe6432-b7e0-43ed-bebd-6ddc4a78ad53
c4fc16da-7250-41a6-8edb-03c8642f4bc3	PDV_Token	eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJwZHYiOiJDNEZDMTZEQS03MjUwLTQxQTYtOEVEQi0wM0M4NjQyRjRCQzMiLCJyZWZyZXNoLXRva2VuIjpmYWxzZSwiY25waiI6IjQ3NjA3MjU3MDAwMTcwIiwidmFsaWRhZGUiOiIyMDI0LTA3LTE2IDIxOjUzOjM5IiwiaWF0IjoxNzIxMDkxMjE5LCJleHAiOjE3MjExNzc2MTl9.RjbwoB64WlQlsn8WB_b2aPXkeo9wMnLDqHNpsf_mMS_b65s3sOOcS75V1qqF6I-15v7037wgHcGMaoCiy7DwaRn07QhYBJfA5eTpma__VZr086w7pxKrCKGWwxF1asatd4O1ntMIcy-r0rMjvigoVzR_N1w6knbl7lozU2i7otLteuF2Q8U8KySN1R63LoHjzrYknsxBpPU9V7Tbe_X_L0ii5874CbDnHqNww8BGMEka2JLTxpZvLVJrpNqldj9I6N7FSgar8UlLlyPHeMqzPQOGe2Ff_mP3-v0WwNrkqn0yoFe85Sxleg0vAQbd1q5fR9O29VDYr9ALV_TyC6JeEP21Cf4jFiMs5J0yS8sKeUv4ze3LA8sHNjhKnaYiofb8qgCKmtDvyel08rrOmKeVzasNUKgbte_0BTi8UsB-qhkCcPqG5xXTrDSmt_-FD-B3hk9kSVa2Mp1L1RNoCigg9ykkMJwcVt0zOkmHB-AlmWfj11LtRUcx_y8oIYVzq8_5	6af08486-0fea-43f4-8b0d-833f923bd1e9
c4fc16da-7250-41a6-8edb-03c8642f4bc3	PDV_Token_Refresh	eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJyZWZyZXNoLXRva2VuLWlkIjozMDUsInBkdiI6IkM0RkMxNkRBLTcyNTAtNDFBNi04RURCLTAzQzg2NDJGNEJDMyIsInJlZnJlc2gtdG9rZW4iOnRydWUsImNucGoiOiI0NzYwNzI1NzAwMDE3MCIsInZhbGlkYWRlIjoiMjAyNC0wNy0xOCAyMTo1MzozOSIsImlhdCI6MTcyMTA5MTIxOSwiZXhwIjoxNzIxMzQ5ODE5fQ.xhbawYNYafbLU7Mvqvuckg6HZO1-0EWkxDYggR17OOrazseH7qZr6PwgIXjjESx_kraeJwXfUFV_7b7AA4KBOJ7BqQAe6c-vLIpE0DySJV6ZtAaLXEdDkcqDmtIVcbtwogNsAK5FklF48QtWeZwUaxNDImta5KyWSILpJMVEQ9NtfOEP7K1tSSUgQkT5_ayy3CGImIrpwhUmtatuxsmdTxuwQYV_MfjycaUs-w4Apqkr8-fAVWGLh7Pu4-vdWaF7toCB9CrSYSkwUvTZFf4zRqMHbFZzlF4nfVRCveKJGCifRSg0Wfs7cgu__kblZhjWad-VcRQ0ZMxNhnvUcTvwndjWimI504XVQaE2nPp8PRrUlmWF-zEN1khPbYJtr53fNoED8I4e7wxKO_aLGwcoNcV7rSAcR3L3M973nWXWv52GTEHrPFeKOfU4x_SQsM4Qjg1DdDsJs0uXIusNY8HbqsHADX0FvX6IPnR6FJ_D_tflpNL65rl0dNEcrE1kCy-D	45d67a50-a3e9-4766-ba59-6d4ffc1b2c57
\N	Backend_Token	eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJwZHZfaWQiOjIsImVtcHJlc2FfaWQiOjIsImlhdCI6MTc4NTg5NDAxMCwiZXhwIjoxNzg4NDg2MDEwLCJzdWIiOiJwZHYtc3luYyJ9.ytST-dVmXbG1bpdANnLgVMPgFffVHVwixvONJWB9mD4	a9cd13df-aeda-43cb-8a8b-be1cbb47f8f9
\N	Backend_URL	https://monte-design-biz-ancient.trycloudflare.com/api	b3c76f55-df65-493f-b1ed-60ea4b0e22ae
\.


--
-- TOC entry 3728 (class 0 OID 33573)
-- Dependencies: 237
-- Data for Name: parceiro_dependentes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.parceiro_dependentes (id, status, parceiro_id, nome, rfid, limite_disponivel) FROM stdin;
1	A	2	VALDOMIRO	\N	0.00
2	A	1	JOSE	ONIX123456780DEP	0.00
3	A	14	MARCELO	\N	0.00
\.


--
-- TOC entry 3731 (class 0 OID 33582)
-- Dependencies: 240
-- Data for Name: parceiro_formas_pagameto; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.parceiro_formas_pagameto (id, parceiro_id, forma_pagamento_id, tabela_id) FROM stdin;
1	0	5	\N
2	0	5	\N
\.


--
-- TOC entry 3729 (class 0 OID 33576)
-- Dependencies: 238
-- Data for Name: parceiro_frotas; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.parceiro_frotas (id, status, parceiro_id, veiculo, placa) FROM stdin;
1	A	2	SUV	ORG2768
2	A	1	ECOSPRT	ORG2768
3	A	14	L200	ORG8900
\.


--
-- TOC entry 3730 (class 0 OID 33579)
-- Dependencies: 239
-- Data for Name: parceiro_tabelas; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.parceiro_tabelas (id, status, parceiro_id, tabela_id) FROM stdin;
1	A	2	2
\.


--
-- TOC entry 3732 (class 0 OID 33585)
-- Dependencies: 241
-- Data for Name: parceiro_tabelas_formas_pagamento; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.parceiro_tabelas_formas_pagamento (id, status, parceiro_id, forma_pagamento_id, tabela_id) FROM stdin;
14	T	2	0	2
\.


--
-- TOC entry 3733 (class 0 OID 33588)
-- Dependencies: 242
-- Data for Name: parceiros; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.parceiros (id, status, cpf_cnpj, inscricao_estadual, inscricao_municipal, nome_fantasia, razao_social, logradouro, complemento, numero, bairro, municipio, cod_municipio, uf, cep, identificacao, requer_placa, requer_km, requer_condutor, desconto_venda, limite_disponivel, email, rfid, ie_situacao) FROM stdin;
12	L	\N	\N	\N	JOSE SANTOS	\N	CONDOMÍNIO RESIDENCIAL RECANTO DOS CONTOS	\N	N/S	BENEDITO BENTES	MACEIO	2704302	AL	57084138 	\N	F	F	\N	0.00	3712.89	teste@tect.com	\N	I
1	L	47607257000170	\N	\N	HILL TECNOLOGIA	HILL TECNOLOGIA LTDA	DISTRITO POVOADO GULANDIM	CASA	74	CENTRO	ARAPIRACA	2700300	AL	57265000 	\N	F	F	\N	0.00	19.89	contato@hilltecnologia.com.br	ONIX123456780CLI	\N
2	B	15264764000161	\N	\N	TULLI BRASIL	PRISCILA DOS SANTOS MELO 07608326417	R MARIA DAS NEVES GOMES	\N	58	SANTOS DUMONT	MACEIO	2704302	AL	57075725 	\N	T	F	\N	0.00	-12144.36	jvalssilva@gmail.com	\N	\N
14	B	26786247000179	\N	\N	BOLD	ISABELLY E MARCELO MARKETING ME	RUA CÂNDIDO GONÇALVES FRANÇA	\N	818	BRASÍLIA	SETE LAGOAS	3167202	MG	35702014 	\N	F	F	\N	0.00	-259.83	elza-damata96@provale.com.br	\N	I
\.


--
-- TOC entry 3734 (class 0 OID 33593)
-- Dependencies: 243
-- Data for Name: pos; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.pos (id, serial, pdv) FROM stdin;
\.


--
-- TOC entry 3735 (class 0 OID 33596)
-- Dependencies: 244
-- Data for Name: pre_venda_pagamentos; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.pre_venda_pagamentos (id, pre_venda_id, processado, forma_pagamento_id, vencimento, desconto, acrescimo, total) FROM stdin;
\.


--
-- TOC entry 3736 (class 0 OID 33599)
-- Dependencies: 245
-- Data for Name: produtos; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.produtos (id, tipo, categoria, codigo, tipo_codigo, descricao, descricao_resumida, gtin_comercial, gtin_tributacao, codigo_auxiliar, unidade_comercial, unidade_tributacao, quantidade_comercial, quantidade_tributacao, indicador_arredondamento, indicador_producao, fracionado, pesado_caixa, etiqueta_balanca, cst, cfop, aliquota, cod_ticket, ncm, ncm_excecao, cest, imposto_chave, imposto_aliquota_importacao, imposto_aliquota_federal, imposto_aliquota_estadual, imposto_aliquota_municipal, codigo_anp, controla_numero_serie, controla_lote, solicita_vendedor, grade_id, setor_impressao_1, setor_impressao_2, setor_impressao_3, setor_impressao_4, exclusivo_kit, descricao_anp, cst_pis, cst_cofins, aliquota_pis, aliquota_cofins, tipo_combustivel, predbcefet, picmsefet, pfcpstret, pfcpst, pfcp, modbc, modbcst, pmvast, predbcst, picmsst, predbc, pglp, pgnn, pgni, vpart, observacao) FROM stdin;
1	\N	C	0000001	3	GASOLINA COMUM	GASOLINA COMUM	0000001	0000001	\N	LT	LT	\N	1.000	\N	T	T	F	F	60 	5656	0.00	\N	27101259	   	0600200	6OI7AC	0.000	13.450	27.000	0.000	320101001	\N	\N	\N	\N	\N	\N	\N	\N	\N	GASOLINA A COMUM	49 	49 	1.650	7.600	5	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	\N	\N	\N	\N	\N
2	\N	P	0000002	3	COCA COLA ZERO AÇÚCAR 2L	COCA COLA ZERO AÇÚCAR 2L	0000002	0000002	\N	UN	UN	\N	1.000	\N	T	F	F	F	60 	5656	0.00	\N	29062910	   	2803700	6OI7AC	0.000	4.200	0.000	0.000	0	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	49 	49 	1.650	7.600	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	\N	\N	\N	\N	\N
3	\N	C	0000003	3	ETANOL	ETANOL	0000003	0000003	\N	LT	LT	\N	1.000	\N	T	T	F	F	60 	5405	0.00	\N	27101259	   	0600200	6OI7AC	0.000	13.450	27.000	0.000	810101002	\N	\N	\N	\N	\N	\N	\N	\N	\N	ETANOL HIDRATADO ADITIVADO	01 	01 	1.650	7.600	1	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	\N	\N	\N	\N	\N
4	\N	C	0000004	3	DIESEL S10	DIESEL S10	0000004	0000004	\N	LT	LT	\N	1.000	\N	T	T	F	F	60 	5656	0.00	\N	27101259	   	0600200	6OI7AC	0.000	13.450	27.000	0.000	420105001	\N	\N	\N	\N	\N	\N	\N	\N	\N	ÓLEO DIESEL A S10	49 	49 	1.650	7.600	3	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	\N	\N	\N	\N	\N
5	\N	P	0000005	3	ADITIVO DS 1LT	ADITIVO DS 1LT	0000005	0000005	\N	UN	UN	\N	1.000	\N	T	F	F	F	60 	5405	0.00	\N	38112120	   	0600700	6OI7AC	0.000	18.730	0.000	0.000	820101014	\N	\N	\N	\N	\N	\N	\N	\N	\N	DIESEL B20 S1800 - ADITIVADO	49 	49 	1.650	7.600	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	\N	\N	\N	\N	\N
6	\N	P	0000006	3	AGUA MINERAL 300ML	AGUA MINERAL 300ML	0000006	0000006	\N	UN	UN	\N	1.000	\N	T	F	F	F	00 	5102	17.00	\N	22011000	02 	0300500	6OI7AC	0.000	13.450	17.000	0.000	0	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	49 	49 	1.650	7.600	0	0.000	0.000	0.000	0.000	1.000	0.000	0.000	0.000	0.000	0.000	0.000	\N	\N	\N	\N	\N
7	\N	C	0000007	3	GASOLINA ADITIVADA	GASOLINA ADITIVADA	0000007	0000007	\N	LT	LT	\N	1.000	\N	T	T	F	F	60 	5656	0.00	\N	27101259	   	0600200	6OI7AC	0.000	13.450	27.000	0.000	320101001	\N	\N	\N	\N	\N	\N	\N	\N	\N	GASOLINA A COMUM	49 	49 	1.650	7.600	6	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	\N	\N	\N	\N	\N
8	\N	G	0000008	3	GÁS NATURAL	GÁS NATURAL	0000008	0000008	\N	LT	LT	\N	1.000	\N	T	T	F	F	60 	5656	0.00	\N	27101259	   	0600200	6OI7AC	0.000	13.450	27.000	0.000	320101001	\N	\N	\N	\N	\N	\N	\N	\N	\N	GASOLINA A COMUM	49 	49 	1.650	7.600	7	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	\N	\N	\N	\N	\N
\.


--
-- TOC entry 3737 (class 0 OID 33604)
-- Dependencies: 246
-- Data for Name: produtos_codigos; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.produtos_codigos (id, status, produto_id, codigo) FROM stdin;
\.


--
-- TOC entry 3738 (class 0 OID 33607)
-- Dependencies: 247
-- Data for Name: produtos_kits; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.produtos_kits (id, kit_id, produto_id, quantidade, tabela_preco_id) FROM stdin;
\.


--
-- TOC entry 3739 (class 0 OID 33610)
-- Dependencies: 248
-- Data for Name: produtos_promocoes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.produtos_promocoes (produto_id, valor, validade) FROM stdin;
\.


--
-- TOC entry 3740 (class 0 OID 33613)
-- Dependencies: 249
-- Data for Name: produtos_series; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.produtos_series (id, produto_id, serie, venda_id) FROM stdin;
\.


--
-- TOC entry 3741 (class 0 OID 33616)
-- Dependencies: 250
-- Data for Name: produtos_setores; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.produtos_setores (id, setor_id, produto_id) FROM stdin;
10	1	8
9	1	7
8	1	6
7	1	5
6	1	4
5	1	3
4	1	2
3	1	1
\.


--
-- TOC entry 3742 (class 0 OID 33619)
-- Dependencies: 251
-- Data for Name: promocoes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.promocoes (id, status, produto_id, quantidade, a_partir, total, desconto, tipo, forma_pagamento_id, conteudo) FROM stdin;
\.


--
-- TOC entry 3744 (class 0 OID 33625)
-- Dependencies: 253
-- Data for Name: sangria_suprimento; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.sangria_suprimento (id, id_serial, pdv, setor_id, turno_id, turno_posto_id, tipo, forma_pagamento_id, valor, data_hora, usuario_id, historico, sincronizado) FROM stdin;
cbfb33ca-7be4-49ec-b349-9cac82b96d9a	18	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	ab925bd3-02b4-4e10-9cc2-5f1041c0275d	6994bff5-9e9a-41c8-b3c3-583d576ac0d4	SU	1	15.00	2024-04-03 14:43:46.004	1	TESTE	T
35c3c3fc-61f5-4e26-bbc7-a481c740817e	19	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	6c422990-d5d5-4525-814a-606fdf2617f5	ac112bba-7590-4703-a55d-91c155f9573c	SU	1	10.00	2024-04-03 16:13:17.785	1		T
f4e0258c-3ab4-4bfb-a274-f2a49ce345d2	20	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	02a776dd-58cc-4603-a5d7-ae9f3076e349	002f3b8c-90de-4b0f-9716-5b0aad058474	SU	1	10.00	2024-07-08 20:09:49.068	1		T
8beb6d44-12ec-4939-ab3b-4dd148be1633	21	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	b722c3ea-d77f-4bfd-89e3-65e9728213d9	5ee6bdff-8712-4b87-b325-420923350fa5	SU	1	5.00	2024-07-08 20:18:13.287	1	TESTE	T
ac5ea17c-3e9e-427b-a3ff-9ae332eebd74	22	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	220e4c12-4692-4bed-8b2a-bf6c187d262e	de25a878-5829-4340-a639-dc06a4edaf2d	SU	1	10.00	2024-07-15 21:25:20.75	1		T
\.


--
-- TOC entry 3745 (class 0 OID 33630)
-- Dependencies: 254
-- Data for Name: setores; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.setores (id, descricao, impressora) FROM stdin;
1	PISTA	\N
\.


--
-- TOC entry 3746 (class 0 OID 33633)
-- Dependencies: 255
-- Data for Name: tabela_preco_itens; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.tabela_preco_itens (id, tabela_preco_id, produto_id, valor_comercial, valor_tributacao) FROM stdin;
3	1	3	2.360	2.360
4	1	4	3.450	3.450
7	1	7	3.990	3.990
8	1	8	2.990	2.990
11	2	1	3.590	3.590
2	1	2	11.200	11.200
5	1	5	45.000	45.000
6	1	6	3.890	3.890
9	2	3	5.540	5.540
10	2	7	5.560	5.560
12	2	8	4.570	4.570
1	1	1	3.590	3.590
\.


--
-- TOC entry 3747 (class 0 OID 33636)
-- Dependencies: 256
-- Data for Name: tabela_precos; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.tabela_precos (id, padrao, status, descricao, exclusiva_cliente) FROM stdin;
1	T	T	PADRAO	\N
2	F	T	VENDA A PRAZO	\N
\.


--
-- TOC entry 3748 (class 0 OID 33639)
-- Dependencies: 257
-- Data for Name: tanques; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.tanques (id, numero, gtin, descricao, capacidade, estoque) FROM stdin;
2	2	0000003	ETANOL	10000.000	9843.794
3	3	0000004	DIESEL S10	20000.000	9821.102
4	4	0000007	GASOLINA ADITIVADA	15000.000	9800.385
5	5	0000008	GÁS NATURAL	50000.000	9885.267
1	1	0000001	GASOLINA COMUM	15000.000	9608.173
\.


--
-- TOC entry 3749 (class 0 OID 33642)
-- Dependencies: 258
-- Data for Name: tanques_medicoes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.tanques_medicoes (id, tanque_numero, descricao, volume_expansao, estoque_atual, data_hora, sincronizado) FROM stdin;
\.


--
-- TOC entry 3753 (class 0 OID 33654)
-- Dependencies: 262
-- Data for Name: turno_postos; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.turno_postos (id, id_serial, pdv, setor_id, status, numero, abertura_usuario_id, fechamento_usuario_id, data_hora_abertura, data_hora_fechamento, sincronizado_abertura, sincronizado_fechamento) FROM stdin;
3b03568d-4f8d-4a18-95bc-8ff9a753dab1	13	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	F	1	1	1	2024-05-31 23:00:56.288	2024-07-08 20:09:10.371	T	T
002f3b8c-90de-4b0f-9716-5b0aad058474	14	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	F	1	1	1	2024-07-08 20:09:12.119	2024-07-08 20:16:46.241	T	T
5ee6bdff-8712-4b87-b325-420923350fa5	15	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	F	2	1	1	2024-07-08 20:16:47.875	2024-07-08 20:27:52.363	T	T
de25a878-5829-4340-a639-dc06a4edaf2d	16	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	A	1	1	\N	2024-07-15 21:24:12.922	\N	T	F
\.


--
-- TOC entry 3751 (class 0 OID 33648)
-- Dependencies: 260
-- Data for Name: turnos; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.turnos (id, id_serial, pdv, setor_id, status, numero, suprimento, data_hora_abertura, data_hora_reabertura, data_hora_fechamento, tipo, venda_bruta, cancelamentos, descontos, acrescimos, abertura_usuario_id, fechamento_usuario_id, sincronizado_abertura, sincronizado_fechamento) FROM stdin;
6b3774e0-3983-4f75-b24d-224dda0da673	13	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	F	1	0.00	2024-05-31 23:00:56.278	\N	2024-07-08 20:09:08.973	P	0.00	0.00	0.00	0.00	1	0	F	F
02a776dd-58cc-4603-a5d7-ae9f3076e349	14	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	F	1	10.00	2024-07-08 20:09:12.048	\N	2024-07-08 20:16:45.02	P	0.00	0.00	0.00	0.00	1	0	F	F
b722c3ea-d77f-4bfd-89e3-65e9728213d9	15	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	F	2	5.00	2024-07-08 20:16:47.862	\N	2024-07-08 20:27:51.246	P	0.00	0.00	0.00	0.00	1	0	F	F
220e4c12-4692-4bed-8b2a-bf6c187d262e	16	c4fc16da-7250-41a6-8edb-03c8642f4bc3	1	A	1	10.00	2024-07-15 21:24:12.26	\N	\N	\N	\N	\N	\N	\N	1	\N	F	F
\.


--
-- TOC entry 3754 (class 0 OID 33659)
-- Dependencies: 263
-- Data for Name: usuario_permissoes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.usuario_permissoes (id, usuario_id, cancela_venda_aberta, cancela_venda_fechada, cancela_item, desconto_item, desconto_fechamento, desconto_fechamento_pv, acrescimo_item, acrescimo_fechamento, acrescimo_fechamento_pv, cliente_limite, cliente_bloqueado, cliente_forma_pagamento, sangria, suprimento, abertura_turno, fechamento_turno, reabertura_turno, afericao, lista_todos_abastecimentos, operacoes_tef, limite_desconto_acrescimo, sangria_lancamento_saida, desmembramento, libera_troco_maximo) FROM stdin;
1	1	T	T	T	T	T	\N	T	T	\N	T	T	\N	T	T	T	T	\N	T	T	T	\N	\N	T	T
2	2	T	T	T	T	T	\N	T	T	\N	T	T	\N	F	F	T	T	\N	T	T	T	\N	\N	T	T
3	3	T	T	T	T	T	\N	T	T	\N	T	T	\N	F	F	T	T	\N	T	T	T	\N	\N	T	T
6	6	T	T	T	T	T	\N	T	T	\N	T	T	\N	F	F	T	T	\N	T	T	T	\N	\N	T	T
\.


--
-- TOC entry 3755 (class 0 OID 33662)
-- Dependencies: 264
-- Data for Name: usuarios; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.usuarios (id, status, nome, tentativas_invalidas, login, senha, rfid, rfid_debito, rfid_credito, digital, cartao_magnetico, perc_max_desc_acres_item, valor_max_desc_acres_item, perc_max_desc_acres_subtotal, valor_max_desc_acres_subtotal) FROM stdin;
2	A	ADRIANO JOSE	\N	JOSE	$2a$10$Jdgmfr.6uHNkMgOq6Hwka.NcQO9aq0FWYZTXGYQPcwm0wFtAjfrrC	\N	\N	\N	\N	\N	0.000	0.000	0.000	0.000
6	A	ADRIANO JOSE	\N	pdv	$2a$10$CVoDguwPagkpsNiBJfDKmuikEEDGX4sNTpvLspz/klmEr98OE23TO	\N	\N	\N	\N	\N	0.000	0.000	0.000	0.000
3	A	FERNANDA DOS SANTOS	\N	NANDA	$2a$10$iQZLayH5n/1unfijdVz4sO5wHO9HxNHfNneWKkuo2iKvamfCn3L0m	\N	\N	\N	\N	\N	0.000	0.000	0.000	0.000
1	A	JOSE CARLOS DOS SANTOS	\N	pista	$2a$10$v3jOnbNSAoTWE388XBz/1elcf/Q27/UROt4msmkIUCA.f9d09Xq2a	ONIX123456780FUN	\N	\N	\N	\N	0.000	0.000	0.000	0.000
\.


--
-- TOC entry 3757 (class 0 OID 33672)
-- Dependencies: 266
-- Data for Name: venda_cheque_trocos; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.venda_cheque_trocos (id, id_serial, status, empresa_id, caixa_id, venda_id, forma_pagamento_id, sequencia, cliente_id, cheque_troco_id, agencia, conta, data_emissao, numero_cheque, valor, liberado_venda) FROM stdin;
\.


--
-- TOC entry 3759 (class 0 OID 33678)
-- Dependencies: 268
-- Data for Name: venda_cheques; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.venda_cheques (id, id_serial, venda_id, forma_pagamento_id, sequencia, compensacao, banco, agencia, agencia_digito, conta, conta_digito, numero, numero_digito, valor, vencimento, tipo_pessoa, titular, cpf_cnpj, telefone) FROM stdin;
\.


--
-- TOC entry 3761 (class 0 OID 33684)
-- Dependencies: 270
-- Data for Name: venda_deposito_trocos; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.venda_deposito_trocos (id, id_serial, forma_pagamento_id, cliente_id, venda_id, banco, agencia, agencia_digito, conta, conta_digito, tipo_conta, operacao, favorecido, cpf_cnpj, telefone, depositante, valor, observacao, data) FROM stdin;
\.


--
-- TOC entry 3763 (class 0 OID 33712)
-- Dependencies: 272
-- Data for Name: venda_itens; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.venda_itens (id, id_serial, status, venda_id, sequencia, pre_venda, produto_id, produto_gtin, quantidade, valor_comercial, valor_tributacao, subtotal, desconto, acrescimo, total, desconto_fechamento, acrescimo_fechamento, total_fechamento, cfop, cst, icms_aliquota, icms_valor, total_tributos, total_tributos_importacao, total_tributos_federal, total_tributos_estadual, total_tributos_municipal, abastecimento_id, bico_id, encerrante_inicial, encerrante_final, rfid_vendedor, rfid_cliente, setor_id, grade_item_id, grade_codigo, grade_descricao, lote_id, produto_serie_id, tabela_preco_id, tabela_padrao, vendedor_id, icmsst_valor, icmsst_aliquota, predbcefet, picmsefet, vicmsefet, pfcpstret, vfcpstret, pfcpst, vfcpst, pfcp, vfcp, modbc, modbcst, pmvast, predbcst, predbc, cst_pis, pis_valor, pis_aliquota, cst_cofins, cofins_valor, cofins_aliquota) FROM stdin;
9aa1311f-2950-42cf-a6fd-a78926137f29	214	A	494c79c5-6c3a-437a-a79b-ae6bab3ca4a2	1	F	1	0000001	1.6900	5.96	5.96	10.07	0.00	0.00	10.07	0.00	0.00	0.00	5656	60 	0.000	0.000	4.07	0.000	1.350	2.720	0.000	dd3d4394-4250-45e5-9b3d-b6b1306f3114	1	10000000.000	10000001.690			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.170	1.650	49	0.770	7.600
110e5bb6-26ce-4a8c-b4b5-9965bd86f0e5	215	A	494c79c5-6c3a-437a-a79b-ae6bab3ca4a2	2	F	4	0000004	2.2100	3.33	3.33	7.35	0.00	0.00	7.35	0.00	0.00	0.00	5656	60 	0.000	0.000	2.97	0.000	0.990	1.980	0.000	a3fd0c63-d2b6-4f52-b357-073868285354	2	10000006.110	10000008.320			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.120	1.650	49	0.560	7.600
c08d1de8-61f7-4be8-8c46-38481533a37d	216	A	f5b2483f-48cf-4c6c-af0d-11e2d8925324	1	F	4	0000004	2.9900	3.33	3.33	9.95	0.00	0.00	9.95	0.00	0.00	0.00	5656	60 	0.000	0.000	4.03	0.000	1.340	2.690	0.000	cdd21a17-8539-4d1f-84f9-e9dbacb946d9	2	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.160	1.650	49	0.760	7.600
92b6be41-c1b8-45e5-8137-15e30ec9fbdd	217	A	5332bca5-274e-4951-a3e8-7e532971b8b3	1	F	4	0000004	4.2900	3.33	3.33	14.28	0.00	0.00	14.28	0.00	0.00	0.00	5656	60 	0.000	0.000	5.78	0.000	1.920	3.860	0.000	debcf79d-df67-4b59-b1fb-1c02498f1480	2	10000010.790	10000015.080			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.240	1.650	49	1.090	7.600
44b49238-811a-4859-8954-7f38e61ca3df	218	A	5332bca5-274e-4951-a3e8-7e532971b8b3	2	F	4	0000004	2.4700	3.33	3.33	8.22	0.00	0.00	8.22	0.00	0.00	0.00	5656	60 	0.000	0.000	3.33	0.000	1.110	2.220	0.000	f15b8043-8ee3-445b-b495-ca4914693ca9	2	10000008.320	10000010.790			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.140	1.650	49	0.620	7.600
5b899e28-331e-4df3-93cc-45c9840767e9	219	A	1fdc3b9f-aead-402a-adc0-7c71b4ead870	1	F	4	0000004	3.9000	3.33	3.33	12.98	0.00	0.00	12.98	0.00	0.00	0.00	5656	60 	0.000	0.000	5.25	0.000	1.750	3.500	0.000	5a3241e6-a358-48aa-9b40-9a176d45b2ed	2	10000016.900	10000020.800			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.210	1.650	49	0.990	7.600
10d6b5c9-9242-43a8-932c-4f215e05721e	220	A	11a1b777-caa5-42f8-8bea-cdcbd43c62f5	1	F	1	0000001	3.9000	5.96	5.96	23.24	0.00	0.00	23.24	0.00	0.00	0.00	5656	60 	0.000	0.000	9.40	0.000	3.130	6.270	0.000	8b33d4d3-c377-49b4-8e57-7aec14bc08ee	1	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.380	1.650	49	1.770	7.600
2bff7f0c-ebf3-4561-a35b-ccf69c8602d9	221	A	d7876e06-c843-457d-abda-01d06823c88e	1	F	1	0000001	3.9000	5.96	5.96	23.24	0.00	0.00	23.24	0.00	0.00	0.00	5656	60 	0.000	0.000	9.40	0.000	3.130	6.270	0.000	e23066c6-ecc7-4dc2-a1e2-1c6051ce2b7b	1	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.380	1.650	49	1.770	7.600
bb895864-7e27-42d7-a69c-36e5e90583fb	222	A	d5215010-7e5a-4e26-9159-1de27f89153a	1	F	1	0000001	3.9000	5.96	5.96	23.24	0.00	0.00	23.24	0.00	0.00	0.00	5656	60 	0.000	0.000	9.40	0.000	3.130	6.270	0.000	b26e6603-e674-4447-9e7a-8eb8b4c26025	1	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.380	1.650	49	1.770	7.600
1ca06c10-f576-44a1-b5a4-f7342a29fdf8	223	A	e4ce0f55-a046-42bb-aa08-3ba2c338e476	1	F	1	0000001	3.9000	5.96	5.96	23.24	0.00	0.00	23.24	0.00	0.00	0.00	5656	60 	0.000	0.000	9.40	0.000	3.130	6.270	0.000	066a832d-7c18-4ae0-83a9-c86fff84c04d	1	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.380	1.650	49	1.770	7.600
d16cc739-d293-48c2-a2fa-2c86465a166c	224	A	e4ce0f55-a046-42bb-aa08-3ba2c338e476	2	F	4	0000004	1.8200	3.33	3.33	6.06	0.00	0.00	6.06	0.00	0.00	0.00	5656	60 	0.000	0.000	2.46	0.000	0.820	1.640	0.000	c7473f1b-a4b6-4c48-9a25-64f66a4e6291	2	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.100	1.650	49	0.460	7.600
ec298112-0902-4544-91fe-5a319e8d451f	225	A	519b8061-27ad-4fb6-9ce1-9e70a6a247d7	1	F	4	0000004	4.8100	3.33	3.33	16.01	0.00	0.00	16.01	0.00	0.00	0.00	5656	60 	0.000	0.000	6.47	0.000	2.150	4.320	0.000	0a9e83e5-5c1d-44b2-9519-c0b09591d074	2	10000020.800	10000025.610			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.260	1.650	49	1.220	7.600
c3eb9a90-8f8b-4c41-a9e1-02be36e90b18	226	A	9375e18e-5a5e-43a0-8f7b-12f46513230c	1	F	2	0000002	1.0000	11.20	11.20	11.20	0.00	0.00	11.20	0.00	0.00	0.00	5656	60 	0.000	0.000	0.47	0.000	0.470	0.000	0.000	\N	0	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.180	1.650	49	0.850	7.600
e594073b-0185-47fd-a106-23e0c4cfde53	227	A	c813052a-50e4-45fc-857d-486094edec1a	1	F	2	0000002	1.0000	11.20	11.20	11.20	0.00	0.00	11.20	0.00	0.00	0.00	5656	60 	0.000	0.000	0.47	0.000	0.470	0.000	0.000	\N	0	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.180	1.650	49	0.850	7.600
4d10d6d5-38da-43ba-ba8d-84a1480e6a96	228	A	1ba831c4-445a-4d09-8990-17a1fc9e766a	1	F	1	0000001	7.1500	1.12	5.65	8.02	0.00	0.00	8.02	0.00	0.00	0.00	5656	60 	0.000	0.000	3.25	0.000	1.080	2.170	0.000	695cfa9c-8e5c-4fd2-8d5a-088890a1757c	1	10000000.000	10000007.150			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.130	1.650	49	0.610	7.600
6e709489-f3c0-4104-8f44-52851666ed14	229	A	8514221c-897b-408c-a86b-c4633580f36f	1	F	2	0000002	1.0000	11.20	11.20	11.20	0.00	0.00	11.20	0.00	0.00	0.00	5656	60 	0.000	0.000	0.47	0.000	0.470	0.000	0.000	\N	0	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.180	1.650	49	0.850	7.600
41ba497f-ef67-459c-b0d9-9de68e5780e6	230	A	b2547dd2-e8d8-4a23-84b0-adc28e509a69	1	F	1	0000001	8.9911	1.12	5.65	10.07	0.00	0.00	10.07	0.00	0.00	0.00	5656	60 	0.000	0.000	4.07	0.000	1.350	2.720	0.000	aadae57f-c376-4e82-b16b-4854be0cf57c	1	10000007.150	10000016.120			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.170	1.650	49	0.770	7.600
bf2af61e-fbcb-4eee-8c16-fc9081585a0d	231	A	b2547dd2-e8d8-4a23-84b0-adc28e509a69	2	F	1	0000001	7.1500	1.12	5.65	8.02	0.00	0.00	8.02	0.00	0.00	0.00	5656	60 	0.000	0.000	3.25	0.000	1.080	2.170	0.000	695cfa9c-8e5c-4fd2-8d5a-088890a1757c	1	10000000.000	10000007.150			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.130	1.650	49	0.610	7.600
b10a0bf0-eb48-40e0-bef5-ddfc007fc578	232	A	b2547dd2-e8d8-4a23-84b0-adc28e509a69	3	F	4	0000004	2.2100	1.12	5.88	2.48	0.00	0.00	2.48	0.00	0.00	0.00	5656	60 	0.000	0.000	1.00	0.000	0.330	0.670	0.000	73214ac1-e531-414b-a9d6-05d8de7c153f	2	10000000.000	10000002.210			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.040	1.650	49	0.190	7.600
1aed4aeb-58ba-4988-81c3-4106a1f61096	233	A	892cf0aa-c767-4c3c-8cce-7546bd4aa772	1	F	1	0000001	8.9911	1.12	5.65	10.07	0.00	0.00	10.07	0.00	0.00	0.00	5656	60 	0.000	0.000	4.07	0.000	1.350	2.720	0.000	aadae57f-c376-4e82-b16b-4854be0cf57c	1	10000007.150	10000016.120			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.170	1.650	49	0.770	7.600
d8ef4f2d-f806-42da-bf0f-c8fe447ba166	234	A	973202c6-33e9-4988-88d2-da973aa6247f	1	F	4	0000004	2.2100	1.12	3.45	2.48	0.00	0.00	2.48	0.00	0.00	0.00	5656	60 	0.000	0.000	1.00	0.000	0.330	0.670	0.000	73214ac1-e531-414b-a9d6-05d8de7c153f	2	10000000.000	10000002.210			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.040	1.650	49	0.190	7.600
3677b5ad-7eb2-4d19-9948-cdd9104ed7c2	235	A	973202c6-33e9-4988-88d2-da973aa6247f	2	F	1	0000001	7.1500	1.12	3.59	8.02	0.00	0.00	8.02	0.00	0.00	0.00	5656	60 	0.000	0.000	3.25	0.000	1.080	2.170	0.000	695cfa9c-8e5c-4fd2-8d5a-088890a1757c	1	10000000.000	10000007.150			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.130	1.650	49	0.610	7.600
bf1e0ca0-f281-42ff-b964-8ca578b38838	236	A	b49bfb54-41ec-409c-82c0-81e085b364a7	1	F	2	0000002	1.0000	11.20	11.20	11.20	0.00	0.00	11.20	0.00	0.00	0.00	5656	60 	0.000	0.000	0.47	0.000	0.470	0.000	0.000	\N	0	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.180	1.650	49	0.850	7.600
aed545a4-5615-487f-ac64-1f5729bb9c1a	237	A	05fd7a9b-f0ce-47ac-a678-eeca06de17c1	1	F	2	0000002	1.0000	11.20	11.20	11.20	0.00	0.00	11.20	0.00	0.00	0.00	5656	60 	0.000	0.000	0.47	0.000	0.470	0.000	0.000	\N	0	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.180	1.650	49	0.850	7.600
c49f0295-6647-425c-a35d-95404ffea878	238	A	50b385ff-28f4-4e9a-ba1b-7053f726cbaa	1	F	2	0000002	1.0000	11.20	11.20	11.20	0.00	0.00	11.20	0.00	0.00	0.00	5656	60 	0.000	0.000	0.47	0.000	0.470	0.000	0.000	\N	0	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.180	1.650	49	0.850	7.600
10a4104f-45e8-4e7a-affc-a88e9d625e35	239	A	eec7eca1-00bd-479b-bc5e-2e9da4993a53	1	F	6	0000006	1.0000	3.89	3.89	3.89	0.00	0.00	3.89	0.00	0.00	0.00	5102	00 	17.000	0.660	1.18	0.000	0.520	0.660	0.000	\N	0	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	1.000	0.040	0.000	0.000	0.000	0.000	0.000	49	0.060	1.650	49	0.300	7.600
aaa691be-52ff-4ebd-b68f-363c8e3033b1	240	A	39c54d64-fa35-4801-8909-921077d22c23	1	F	2	0000002	1.0000	11.20	11.20	11.20	0.00	0.00	11.20	0.00	0.00	0.00	5656	60 	0.000	0.000	0.47	0.000	0.470	0.000	0.000	\N	0	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.180	1.650	49	0.850	7.600
96ecbdf0-96d0-4371-a385-d9c9fe215a42	241	A	35152064-cc3f-4a44-8be5-390ecbe07fb0	1	F	6	0000006	1.0000	3.89	3.89	3.89	0.00	0.00	3.89	0.00	0.00	0.00	5102	00 	17.000	0.660	1.18	0.000	0.520	0.660	0.000	\N	0	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	1.000	0.040	0.000	0.000	0.000	0.000	0.000	49	0.060	1.650	49	0.300	7.600
5dcc46b6-fa81-453a-991b-764174d188df	242	A	f05cc904-bdc8-4f60-8a2d-9d18fe1bbbcc	1	F	6	0000006	1.0000	3.89	3.89	3.89	0.00	0.00	3.89	0.00	0.00	0.00	5102	00 	17.000	0.660	1.18	0.000	0.520	0.660	0.000	\N	0	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	1.000	0.040	0.000	0.000	0.000	0.000	0.000	49	0.060	1.650	49	0.300	7.600
125367f5-6ad5-4e06-91a0-e2e34a41cca9	243	A	95f8165d-d28e-4ef6-8f24-ffad563e7787	1	F	1	0000001	1.6900	1.12	3.59	1.89	0.00	0.00	1.89	0.00	0.00	0.00	5656	60 	0.000	0.000	0.76	0.000	0.250	0.510	0.000	f2cb2093-74e3-4009-92c3-2c0101aa7b17	1	10000000.000	10000001.690			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	49	0.030	1.650	49	0.140	7.600
b7e02457-bdd7-4a4d-b8aa-261dd0c40409	244	A	a5e31ce5-eb4e-4388-ae78-dae839dff235	1	F	6	0000006	1.0000	3.89	3.89	3.89	0.00	0.00	3.89	0.00	0.00	0.00	5102	00 	17.000	0.660	1.18	0.000	0.520	0.660	0.000	\N	0	0.000	0.000			1	0			0	0	1	T	0	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	0.000	1.000	0.040	0.000	0.000	0.000	0.000	0.000	49	0.060	1.650	49	0.300	7.600
\.


--
-- TOC entry 3765 (class 0 OID 33728)
-- Dependencies: 274
-- Data for Name: venda_pagamentos; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.venda_pagamentos (id, id_serial, sequencia, venda_id, forma_pagamento_id, subtotal, desconto, acrescimo, total, recebido, troco, vinculado, data_vencimento, vendedor_id, voucher_id, tef, tef_pos, tef_terminal, tef_cnpj, tef_nsu, tef_rede, tef_bandeira, tef_operacao, tef_parcelas, tef_tipo_parcelamento, tef_tipo_transacao, tef_desconto, tef_saque, tef_sitef_instituicao, tef_sitef_bandeira, tef_via_estabelecimento, tef_via_cliente) FROM stdin;
1dc9890e-38e1-4abb-966e-f1d85dda0afe	69	1	494c79c5-6c3a-437a-a79b-ae6bab3ca4a2	1	17.42	\N	\N	17.42	17.42	0.00	F	2024-04-03	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
072b4abf-f9d8-4fc7-ae87-6f7e1585aceb	70	1	f5b2483f-48cf-4c6c-af0d-11e2d8925324	1	9.95	\N	\N	9.95	9.95	0.00	F	2024-04-03	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
14a7a566-83b0-48be-b4d0-3da6080dd514	71	1	5332bca5-274e-4951-a3e8-7e532971b8b3	1	22.50	\N	\N	22.50	22.50	0.00	F	2024-04-03	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
110b4404-c49d-4e57-a29a-7ab5e361ae80	72	1	1fdc3b9f-aead-402a-adc0-7c71b4ead870	1	12.98	\N	\N	12.98	12.98	0.00	F	2024-04-03	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
bc6df68e-0539-468e-b63b-b7088a355db7	73	1	e4ce0f55-a046-42bb-aa08-3ba2c338e476	1	29.30	\N	\N	29.30	29.30	0.00	F	2024-04-03	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
ee818eea-cc6a-41d5-b38b-6787959f9817	74	1	519b8061-27ad-4fb6-9ce1-9e70a6a247d7	1	16.01	\N	\N	16.01	16.01	0.00	F	2024-04-03	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
00f4b5bf-c1f8-4976-991a-5c18d093c148	75	1	9375e18e-5a5e-43a0-8f7b-12f46513230c	1	11.20	\N	\N	11.20	11.20	0.00	F	2024-05-28	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
dd8f3f92-435f-4fd0-a6bc-7a8f44bb0300	76	1	8514221c-897b-408c-a86b-c4633580f36f	1	11.20	\N	\N	11.20	11.20	0.00	F	2024-05-31	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
0a401420-c7ca-44d9-8c4b-b430246f4303	77	1	892cf0aa-c767-4c3c-8cce-7546bd4aa772	1	10.07	\N	\N	10.07	10.07	0.00	F	2024-05-31	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
2fc5009b-3814-411f-b35a-0f5de232ad86	78	1	973202c6-33e9-4988-88d2-da973aa6247f	1	10.50	\N	\N	10.50	10.50	0.00	F	2024-07-02	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
757c026b-3da9-4b28-9488-05c45bf0f004	79	1	b49bfb54-41ec-409c-82c0-81e085b364a7	1	11.20	\N	\N	11.20	11.20	0.00	F	2024-07-08	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
30e8d34b-03d1-4937-9fe9-fa21b86e6f9c	80	1	05fd7a9b-f0ce-47ac-a678-eeca06de17c1	1	11.20	\N	\N	11.20	11.20	0.00	F	2024-07-08	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
7bd1aca8-5e01-4bca-ae98-a0ca4e2ea464	81	1	50b385ff-28f4-4e9a-ba1b-7053f726cbaa	1	11.20	\N	\N	11.20	11.20	0.00	F	2024-07-15	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
5d625350-39f9-44f9-a4f4-1b7b18c22a1f	82	1	eec7eca1-00bd-479b-bc5e-2e9da4993a53	1	3.89	\N	\N	3.89	3.89	0.00	F	2024-07-15	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
e555e114-c4a3-49a1-9556-6f88e333a380	83	1	f05cc904-bdc8-4f60-8a2d-9d18fe1bbbcc	1	3.89	\N	\N	3.89	3.89	0.00	F	2024-07-15	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
76f6f6f4-0387-4b42-9026-6dce016ee5a0	84	1	95f8165d-d28e-4ef6-8f24-ffad563e7787	1	1.89	\N	\N	1.89	1.89	0.00	F	2024-07-15	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
14f03ae5-63df-421d-bfaa-b00fb7a0c5e8	85	1	a5e31ce5-eb4e-4388-ae78-dae839dff235	1	3.89	\N	\N	3.89	3.89	0.00	F	2024-07-15	\N	0	F	\N	\N	\N		\N	nenhum	\N	0	\N	\N	\N	\N			\N	\N
\.


--
-- TOC entry 3771 (class 0 OID 41765)
-- Dependencies: 280
-- Data for Name: vendas; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.vendas (id, id_serial, status, tipo, setor_id, pdv, turno_id, turno_posto_id, data_hora, movimento, automatico, subtotal, desconto, acrescimo, valor_total, troco, desconto_itens, acrescimo_itens, taxa_servico, parceiro_id, dependente_id, frota_id, fidelidade_id, tabela_preco_id, cpf_cnpj, inscricao_estadual, ie_situacao, nome_fantasia, razao_social, logradouro, complemento, numero, bairro, municipio, cod_municipio, uf, cep, telefone, email, km, placa, condutor, unidade_consumidora, horimetro, pre_venda_id, pre_venda_numero, nfe_contingencia, nfe_offline, nfe_aguardando_envio, nfe_tentativa_envio, nfe_tipo, nfe_data, nfe_numero, nfe_serie, nfe_chave, nfe_protocolo, nfe_recibo, nfe_retorno, nfe_dados_adicionais, nfe_xml, nfe_arquivo, nfe_inconsistente, nfe_regerar, nfe_cancelamento_data, nfe_cancelamento_motivo, nfe_cancelamento_protocolo, nfe_cancelamento_xml, nfe_cancelada, nfe_inutilizacao_data, nfe_inutilizacao_protocolo, nfe_inutilizada, nfe_rejeicao, voucher_id, usuario_id, vendedor_id, usuario_pre_venda_id, finalizada, estorno, atualiza_retaguarda, sincronizado) FROM stdin;
95f8165d-d28e-4ef6-8f24-ffad563e7787	72	A	E	1	c4fc16da-7250-41a6-8edb-03c8642f4bc3	220e4c12-4692-4bed-8b2a-bf6c187d262e	de25a878-5829-4340-a639-dc06a4edaf2d	2024-07-15 22:01:30.556	2024-07-15	F	1.89	0.00	0.00	1.89	0.00	0.00	0.00	0.00	0	0	0	0	0			 								0	  		\N	\N	0					\N		T	T	T	1	65	2024-07-15 22:01:46.776	214	1	13240747607257000170650010000002149889991895				#CF:T01 B01 N01 EI10000000,00 EF10000001,69 V1,690; AGRADECEMOS A PREFERENCIA!!; 	<?xml version="1.0" encoding="UTF-8"?><NFe xmlns="http://www.portalfiscal.inf.br/nfe"><infNFe versao="4.00" Id="NFe13240747607257000170650010000002149889991895"><ide><cUF>13</cUF><cNF>88999189</cNF><natOp>VENDA</natOp><mod>65</mod><serie>1</serie><nNF>214</nNF><dhEmi>2024-07-15T22:01:46-03:00</dhEmi><tpNF>1</tpNF><idDest>1</idDest><cMunFG>1302603</cMunFG><tpImp>4</tpImp><tpEmis>9</tpEmis><cDV>5</cDV><tpAmb>2</tpAmb><finNFe>1</finNFe><indFinal>1</indFinal><indPres>1</indPres><procEmi>0</procEmi><verProc>HillPDV 1.0.0.0</verProc><dhCont>2024-07-15T22:01:49-03:00</dhCont><xJust>EMITIDA EM CONTINGENCIA EM DECORRENCIA DE PROBLEMAS TECNICOS</xJust></ide><emit><CNPJ>47607257000170</CNPJ><xNome>Hill Tecnologia LTDA</xNome><xFant>Hill Tecnologia</xFant><enderEmit><xLgr>Distrito Povoado Gulandim</xLgr><nro>74</nro><xBairro>CENTRO</xBairro><cMun>1302603</cMun><xMun>MANAUS</xMun><UF>AM</UF><CEP>00000000</CEP><cPais>1058</cPais><xPais>BRASIL</xPais><fone>8299999999</fone></enderEmit><IE>241048010</IE><CRT>3</CRT></emit><det nItem="1"><prod><cProd>1</cProd><cEAN>SEM GTIN</cEAN><xProd>NOTA FISCAL EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL</xProd><NCM>27101259</NCM><CEST>0600200</CEST><CFOP>5656</CFOP><uCom>LT</uCom><qCom>1.6900</qCom><vUnCom>1.1200000000</vUnCom><vProd>1.89</vProd><cEANTrib>SEM GTIN</cEANTrib><uTrib>LT</uTrib><qTrib>1.6900</qTrib><vUnTrib>1.1200000000</vUnTrib><indTot>1</indTot><comb><cProdANP>320101001</cProdANP><descANP>GASOLINA A COMUM</descANP><qTemp>1.6900</qTemp><UFCons>AM</UFCons><encerrante><nBico>1</nBico><nTanque>1</nTanque><vEncIni>10000000.000</vEncIni><vEncFin>10000001.690</vEncFin></encerrante></comb></prod><imposto><vTotTrib>0.76</vTotTrib><ICMS><ICMS60><orig>0</orig><CST>60</CST></ICMS60></ICMS><PIS><PISOutr><CST>49</CST><vBC>0.00</vBC><pPIS>1.6500</pPIS><vPIS>0.03</vPIS></PISOutr></PIS><COFINS><COFINSOutr><CST>49</CST><vBC>0.00</vBC><pCOFINS>7.6000</pCOFINS><vCOFINS>0.14</vCOFINS></COFINSOutr></COFINS></imposto></det><total><ICMSTot><vBC>0.00</vBC><vICMS>0.00</vICMS><vICMSDeson>0.00</vICMSDeson><vFCP>0.00</vFCP><vBCST>0.00</vBCST><vST>0.00</vST><vFCPST>0.00</vFCPST><vFCPSTRet>0.00</vFCPSTRet><vProd>1.89</vProd><vFrete>0.00</vFrete><vSeg>0.00</vSeg><vDesc>0.00</vDesc><vII>0.00</vII><vIPI>0.00</vIPI><vIPIDevol>0.00</vIPIDevol><vPIS>0.03</vPIS><vCOFINS>0.14</vCOFINS><vOutro>0.00</vOutro><vNF>1.89</vNF><vTotTrib>0.76</vTotTrib></ICMSTot></total><transp><modFrete>9</modFrete></transp><pag><detPag><tPag>01</tPag><vPag>1.89</vPag></detPag></pag><infAdic><infCpl>#CF:T01 B01 N01 EI10000000,00 EF10000001,69 V1,690; AGRADECEMOS A PREFERENCIA!!;</infCpl></infAdic><infRespTec><CNPJ>47607257000170</CNPJ><xContato>Jose Valdomiro da Silva Santos</xContato><email>contato@hilltecnologia.com.br</email><fone>82991741328</fone></infRespTec></infNFe><infNFeSupl><qrCode>http://homnfce.sefaz.am.gov.br/nfceweb/consultarNFCe.jsp?p=13240747607257000170650010000002149889991895|2|2|15|1.89|44734139584A684461597A303046653359716548766676686646773D|0|0666E148B0ED7AC3023FC195201C8BBA84AB39AA</qrCode><urlChave>www.sefaz.am.gov.br/nfce/consulta</urlChave></infNFeSupl><Signature xmlns="http://www.w3.org/2000/09/xmldsig#"><SignedInfo><CanonicalizationMethod Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/><SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"/><Reference URI="#NFe13240747607257000170650010000002149889991895"><Transforms><Transform Algorithm="http://www.w3.org/2000/09/xmldsig#enveloped-signature"/><Transform Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/></Transforms><DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"/><DigestValue>DsA9XJhDaYz00Fe3YqeHvfvhfFw=</DigestValue></Reference></SignedInfo><SignatureValue>Xmf8/SZ9qk55ck6Py05x8H/jgF8ACZBLRibqY4LmjLiPZdE/QPQMcsaAPUdu8VG969rnzT0tE+rnlpPTRRAs4hf2Y8eMqdg0rPbx5D1JX5tP7Yc1NGXFCgck8cUy1NKI+xCR1JKXZINE1G4GmTUveP4dcYCZWF9KqHEQkaU+3GHZc231mDF0RTSphS43BOdq17+XBiG4TEwHLnqHz+YVyjGQGs8OwAlR9LTJsvK0lnHwg2MlK2/MfY9O75ELy6wbdzHSSYLz+tJqGAgQ+C3M0rNnAxHWN74muG+/4CHs6WFRVvv2M8KtkHxf99LkC8/wJiWIgY1cGqmDPKegSaLPzA==</SignatureValue><KeyInfo><X509Data><X509Certificate>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</X509Certificate></X509Data></KeyInfo></Signature></NFe>	NFCe\\202407\\13240747607257000170650010000002149889991895-nfe.xml	\N	\N	\N	\N	\N	\N	F	\N	\N	F	\N	\N	1	0	0	T	F	F	T
a5e31ce5-eb4e-4388-ae78-dae839dff235	73	A	E	1	c4fc16da-7250-41a6-8edb-03c8642f4bc3	220e4c12-4692-4bed-8b2a-bf6c187d262e	de25a878-5829-4340-a639-dc06a4edaf2d	2024-07-15 22:02:01.917	2024-07-15	F	3.89	0.00	0.00	3.89	0.00	0.00	0.00	0.00	0	0	0	0	0			 								0	  		\N	\N	0					\N		T	T	T	1	65	2024-07-15 22:02:07.669	215	1	13240747607257000170650010000002159830609967				AGRADECEMOS A PREFERENCIA!!; 	<?xml version="1.0" encoding="UTF-8"?><NFe xmlns="http://www.portalfiscal.inf.br/nfe"><infNFe versao="4.00" Id="NFe13240747607257000170650010000002159830609967"><ide><cUF>13</cUF><cNF>83060996</cNF><natOp>VENDA</natOp><mod>65</mod><serie>1</serie><nNF>215</nNF><dhEmi>2024-07-15T22:02:07-03:00</dhEmi><tpNF>1</tpNF><idDest>1</idDest><cMunFG>1302603</cMunFG><tpImp>4</tpImp><tpEmis>9</tpEmis><cDV>7</cDV><tpAmb>2</tpAmb><finNFe>1</finNFe><indFinal>1</indFinal><indPres>1</indPres><procEmi>0</procEmi><verProc>HillPDV 1.0.0.0</verProc><dhCont>2024-07-15T22:02:09-03:00</dhCont><xJust>EMITIDA EM CONTINGENCIA EM DECORRENCIA DE PROBLEMAS TECNICOS</xJust></ide><emit><CNPJ>47607257000170</CNPJ><xNome>Hill Tecnologia LTDA</xNome><xFant>Hill Tecnologia</xFant><enderEmit><xLgr>Distrito Povoado Gulandim</xLgr><nro>74</nro><xBairro>CENTRO</xBairro><cMun>1302603</cMun><xMun>MANAUS</xMun><UF>AM</UF><CEP>00000000</CEP><cPais>1058</cPais><xPais>BRASIL</xPais><fone>8299999999</fone></enderEmit><IE>241048010</IE><CRT>3</CRT></emit><det nItem="1"><prod><cProd>6</cProd><cEAN>SEM GTIN</cEAN><xProd>NOTA FISCAL EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL</xProd><NCM>22011000</NCM><CEST>0300500</CEST><EXTIPI>02</EXTIPI><CFOP>5102</CFOP><uCom>UN</uCom><qCom>1.0000</qCom><vUnCom>3.8900000000</vUnCom><vProd>3.89</vProd><cEANTrib>SEM GTIN</cEANTrib><uTrib>UN</uTrib><qTrib>1.0000</qTrib><vUnTrib>3.8900000000</vUnTrib><indTot>1</indTot></prod><imposto><vTotTrib>1.18</vTotTrib><ICMS><ICMS00><orig>0</orig><CST>00</CST><modBC>0</modBC><vBC>0.00</vBC><pICMS>17.0000</pICMS><vICMS>0.66</vICMS></ICMS00></ICMS><PIS><PISOutr><CST>49</CST><vBC>0.00</vBC><pPIS>1.6500</pPIS><vPIS>0.06</vPIS></PISOutr></PIS><COFINS><COFINSOutr><CST>49</CST><vBC>0.00</vBC><pCOFINS>7.6000</pCOFINS><vCOFINS>0.30</vCOFINS></COFINSOutr></COFINS></imposto></det><total><ICMSTot><vBC>0.00</vBC><vICMS>0.66</vICMS><vICMSDeson>0.00</vICMSDeson><vFCP>0.00</vFCP><vBCST>0.00</vBCST><vST>0.00</vST><vFCPST>0.00</vFCPST><vFCPSTRet>0.00</vFCPSTRet><vProd>3.89</vProd><vFrete>0.00</vFrete><vSeg>0.00</vSeg><vDesc>0.00</vDesc><vII>0.00</vII><vIPI>0.00</vIPI><vIPIDevol>0.00</vIPIDevol><vPIS>0.06</vPIS><vCOFINS>0.30</vCOFINS><vOutro>0.00</vOutro><vNF>3.89</vNF><vTotTrib>1.18</vTotTrib></ICMSTot></total><transp><modFrete>9</modFrete></transp><pag><detPag><tPag>01</tPag><vPag>3.89</vPag></detPag></pag><infAdic><infCpl>AGRADECEMOS A PREFERENCIA!!;</infCpl></infAdic><infRespTec><CNPJ>47607257000170</CNPJ><xContato>Jose Valdomiro da Silva Santos</xContato><email>contato@hilltecnologia.com.br</email><fone>82991741328</fone></infRespTec></infNFe><infNFeSupl><qrCode>http://homnfce.sefaz.am.gov.br/nfceweb/consultarNFCe.jsp?p=13240747607257000170650010000002159830609967|2|2|15|3.89|52585A574C6441722B4C4353733636412F73664271577447564E673D|0|4EBFCED5971C8352EFC8AA3640247B13C7F43DD8</qrCode><urlChave>www.sefaz.am.gov.br/nfce/consulta</urlChave></infNFeSupl><Signature xmlns="http://www.w3.org/2000/09/xmldsig#"><SignedInfo><CanonicalizationMethod Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/><SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"/><Reference URI="#NFe13240747607257000170650010000002159830609967"><Transforms><Transform Algorithm="http://www.w3.org/2000/09/xmldsig#enveloped-signature"/><Transform Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/></Transforms><DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"/><DigestValue>RXZWLdAr+LCSs66A/sfBqWtGVNg=</DigestValue></Reference></SignedInfo><SignatureValue>DAqCqOMwhxPcNWN/k84cbMER/jnmmyKM3kWI5fA0jQcd/9p2tKSdOwfB1Cag8SAMbfNt512QCUYz2dNaqww4IyfP2OEtDSyd88jRAWhAKi9qKckSoUqFhf2VdS0BveUAaAf95tcK1RbITBhW9LXy0qwpN6JGKJKlaxezpI/7pipQIFLigJzwrsgz3ZqgO3m84q9NFjd+stMgxk/0qhq2mSb86Nk4BRV6JTDQ0s6etRpKl10U0h9oMG2wahaw9wuQ9Ncj7eoQR773NbNSUg4o9s8T3hwMOEfBl/HOG3TMaRPq1Dj85igjrnyGQDgmclanyf0Vp0GnHEg1h6OTBT0IwQ==</SignatureValue><KeyInfo><X509Data><X509Certificate>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</X509Certificate></X509Data></KeyInfo></Signature></NFe>	NFCe\\202407\\13240747607257000170650010000002159830609967-nfe.xml	\N	\N	\N	\N	\N	\N	F	\N	\N	F	\N	\N	1	0	0	T	F	F	T
f05cc904-bdc8-4f60-8a2d-9d18fe1bbbcc	71	A	E	1	c4fc16da-7250-41a6-8edb-03c8642f4bc3	220e4c12-4692-4bed-8b2a-bf6c187d262e	de25a878-5829-4340-a639-dc06a4edaf2d	2024-07-15 21:54:56.068	2024-07-15	F	3.89	0.00	0.00	3.89	0.00	0.00	0.00	0.00	0	0	0	0	0	81732107050		 								1302603	  		\N	\N	0					\N		T	T	T	1	65	2024-07-15 21:55:21.966	213	1	13240747607257000170650010000002139008638144				AGRADECEMOS A PREFERENCIA!!; 	<?xml version="1.0" encoding="UTF-8"?><NFe xmlns="http://www.portalfiscal.inf.br/nfe"><infNFe versao="4.00" Id="NFe13240747607257000170650010000002139008638144"><ide><cUF>13</cUF><cNF>00863814</cNF><natOp>VENDA</natOp><mod>65</mod><serie>1</serie><nNF>213</nNF><dhEmi>2024-07-15T21:55:21-03:00</dhEmi><tpNF>1</tpNF><idDest>1</idDest><cMunFG>1302603</cMunFG><tpImp>4</tpImp><tpEmis>9</tpEmis><cDV>4</cDV><tpAmb>2</tpAmb><finNFe>1</finNFe><indFinal>1</indFinal><indPres>1</indPres><procEmi>0</procEmi><verProc>HillPDV 1.0.0.0</verProc><dhCont>2024-07-15T21:55:25-03:00</dhCont><xJust>EMITIDA EM CONTINGENCIA EM DECORRENCIA DE PROBLEMAS TECNICOS</xJust></ide><emit><CNPJ>47607257000170</CNPJ><xNome>Hill Tecnologia LTDA</xNome><xFant>Hill Tecnologia</xFant><enderEmit><xLgr>Distrito Povoado Gulandim</xLgr><nro>74</nro><xBairro>CENTRO</xBairro><cMun>1302603</cMun><xMun>MANAUS</xMun><UF>AM</UF><CEP>00000000</CEP><cPais>1058</cPais><xPais>BRASIL</xPais><fone>8299999999</fone></enderEmit><IE>241048010</IE><CRT>3</CRT></emit><dest><CPF>81732107050</CPF><xNome>NF-E EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL</xNome><indIEDest>9</indIEDest></dest><det nItem="1"><prod><cProd>6</cProd><cEAN>SEM GTIN</cEAN><xProd>NOTA FISCAL EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL</xProd><NCM>22011000</NCM><CEST>0300500</CEST><EXTIPI>02</EXTIPI><CFOP>5102</CFOP><uCom>UN</uCom><qCom>1.0000</qCom><vUnCom>3.8900000000</vUnCom><vProd>3.89</vProd><cEANTrib>SEM GTIN</cEANTrib><uTrib>UN</uTrib><qTrib>1.0000</qTrib><vUnTrib>3.8900000000</vUnTrib><indTot>1</indTot></prod><imposto><vTotTrib>1.18</vTotTrib><ICMS><ICMS00><orig>0</orig><CST>00</CST><modBC>0</modBC><vBC>0.00</vBC><pICMS>17.0000</pICMS><vICMS>0.66</vICMS></ICMS00></ICMS><PIS><PISOutr><CST>49</CST><vBC>0.00</vBC><pPIS>1.6500</pPIS><vPIS>0.06</vPIS></PISOutr></PIS><COFINS><COFINSOutr><CST>49</CST><vBC>0.00</vBC><pCOFINS>7.6000</pCOFINS><vCOFINS>0.30</vCOFINS></COFINSOutr></COFINS></imposto></det><total><ICMSTot><vBC>0.00</vBC><vICMS>0.66</vICMS><vICMSDeson>0.00</vICMSDeson><vFCP>0.00</vFCP><vBCST>0.00</vBCST><vST>0.00</vST><vFCPST>0.00</vFCPST><vFCPSTRet>0.00</vFCPSTRet><vProd>3.89</vProd><vFrete>0.00</vFrete><vSeg>0.00</vSeg><vDesc>0.00</vDesc><vII>0.00</vII><vIPI>0.00</vIPI><vIPIDevol>0.00</vIPIDevol><vPIS>0.06</vPIS><vCOFINS>0.30</vCOFINS><vOutro>0.00</vOutro><vNF>3.89</vNF><vTotTrib>1.18</vTotTrib></ICMSTot></total><transp><modFrete>9</modFrete></transp><pag><detPag><tPag>01</tPag><vPag>3.89</vPag></detPag></pag><infAdic><infCpl>AGRADECEMOS A PREFERENCIA!!;</infCpl></infAdic><infRespTec><CNPJ>47607257000170</CNPJ><xContato>Jose Valdomiro da Silva Santos</xContato><email>contato@hilltecnologia.com.br</email><fone>82991741328</fone></infRespTec></infNFe><infNFeSupl><qrCode>http://homnfce.sefaz.am.gov.br/nfceweb/consultarNFCe.jsp?p=13240747607257000170650010000002139008638144|2|2|15|3.89|56727A6B306867437330422F6847647832356A66702B6A704C316F3D|0|CE04737CB887B600530C7E037644EB1C04DFE88F</qrCode><urlChave>www.sefaz.am.gov.br/nfce/consulta</urlChave></infNFeSupl><Signature xmlns="http://www.w3.org/2000/09/xmldsig#"><SignedInfo><CanonicalizationMethod Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/><SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"/><Reference URI="#NFe13240747607257000170650010000002139008638144"><Transforms><Transform Algorithm="http://www.w3.org/2000/09/xmldsig#enveloped-signature"/><Transform Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/></Transforms><DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"/><DigestValue>Vrzk0hgCs0B/hGdx25jfp+jpL1o=</DigestValue></Reference></SignedInfo><SignatureValue>JXhp9m7AcDp/CUymjbaOnnCIpxkk9mOpWuT1QDoqqfjOjS6/8LfzlnrmN8KdpHFtW5+L1ZiRed9MRnU1NBSohhyMZqm2WmkwHIhF1NsGwam1jgRt0sbHNzfDe6L1bTeefRFuVaRMFy8LjrSnJSj89EOQRrgHIL6o3HqWiF6c9Ik++8g0UDxZe8yR+1rtXvp0f+b1pjAgYXcaDQWtxl0QmYVrazUWSeRKZRZ3+IzT8ZMUBwKVD7Or4z7NbGD+ehJjk3isIMMNpONKLGbfnidetBmcIOYfH2uHeq04sWIRYf8qD/7U4Tf2d4525JJhWDHSs4CIsMoPiCzBPCnUtuAJsw==</SignatureValue><KeyInfo><X509Data><X509Certificate>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</X509Certificate></X509Data></KeyInfo></Signature></NFe>	NFCe\\202407\\13240747607257000170650010000002139008638144-nfe.xml	\N	\N	\N	\N	\N	\N	F	\N	\N	F	\N	\N	1	0	0	T	F	F	T
b49bfb54-41ec-409c-82c0-81e085b364a7	63	A	E	1	c4fc16da-7250-41a6-8edb-03c8642f4bc3	02a776dd-58cc-4603-a5d7-ae9f3076e349	002f3b8c-90de-4b0f-9716-5b0aad058474	2024-07-08 20:15:58.953	2024-07-08	F	11.20	0.00	0.00	11.20	0.00	0.00	0.00	0.00	0	0	0	0	0			 								0	  		\N	\N	0					\N		F	F	F	1	65	2024-07-08 20:16:05.161	207	1	13240747607257000170650010000002071404965277	113240011911647		<?xml version='1.0' encoding='utf-8'?><soapenv:Envelope xmlns:soapenv="http://www.w3.org/2003/05/soap-envelope"><soapenv:Body><nfeResultMsg xmlns="http://www.portalfiscal.inf.br/nfe/wsdl/NFeAutorizacao4"><retEnviNFe xmlns="http://www.portalfiscal.inf.br/nfe" versao="4.00"><tpAmb>2</tpAmb><verAplic>AM4.00-NAC</verAplic><cStat>104</cStat><xMotivo>Lote processado</xMotivo><cUF>13</cUF><dhRecbto>2024-07-08T19:16:09-04:00</dhRecbto><protNFe versao="4.00"><infProt><tpAmb>2</tpAmb><verAplic>AM4.00-NAC</verAplic><chNFe>13240747607257000170650010000002071404965277</chNFe><dhRecbto>2024-07-08T19:16:09-04:00</dhRecbto><nProt>113240011911647</nProt><digVal>c7hXoEG61WJ2wcQ4YGozW/ltXno=</digVal><cStat>100</cStat><xMotivo>Autorizado o uso da NF-e</xMotivo></infProt></protNFe></retEnviNFe></nfeResultMsg></soapenv:Body></soapenv:Envelope>	AGRADECEMOS A PREFERENCIA!!; 	<?xml version="1.0" encoding="UTF-8"?><nfeProc versao="4.00" xmlns="http://www.portalfiscal.inf.br/nfe"><NFe xmlns="http://www.portalfiscal.inf.br/nfe"><infNFe versao="4.00" Id="NFe13240747607257000170650010000002071404965277"><ide><cUF>13</cUF><cNF>40496527</cNF><natOp>VENDA</natOp><mod>65</mod><serie>1</serie><nNF>207</nNF><dhEmi>2024-07-08T20:16:05-03:00</dhEmi><tpNF>1</tpNF><idDest>1</idDest><cMunFG>1302603</cMunFG><tpImp>4</tpImp><tpEmis>1</tpEmis><cDV>7</cDV><tpAmb>2</tpAmb><finNFe>1</finNFe><indFinal>1</indFinal><indPres>1</indPres><procEmi>0</procEmi><verProc>HillPDV 1.0.0.0</verProc></ide><emit><CNPJ>47607257000170</CNPJ><xNome>Hill Tecnologia LTDA</xNome><xFant>Hill Tecnologia</xFant><enderEmit><xLgr>Distrito Povoado Gulandim</xLgr><nro>74</nro><xBairro>CENTRO</xBairro><cMun>1302603</cMun><xMun>MANAUS</xMun><UF>AM</UF><CEP>00000000</CEP><cPais>1058</cPais><xPais>BRASIL</xPais><fone>8299999999</fone></enderEmit><IE>241048010</IE><CRT>3</CRT></emit><det nItem="1"><prod><cProd>2</cProd><cEAN>SEM GTIN</cEAN><xProd>NOTA FISCAL EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL</xProd><NCM>29062910</NCM><CEST>2803700</CEST><CFOP>5656</CFOP><uCom>UN</uCom><qCom>1.0000</qCom><vUnCom>11.2000000000</vUnCom><vProd>11.20</vProd><cEANTrib>SEM GTIN</cEANTrib><uTrib>UN</uTrib><qTrib>1.0000</qTrib><vUnTrib>11.2000000000</vUnTrib><indTot>1</indTot></prod><imposto><vTotTrib>0.47</vTotTrib><ICMS><ICMS60><orig>0</orig><CST>60</CST></ICMS60></ICMS><PIS><PISOutr><CST>49</CST><vBC>0.00</vBC><pPIS>1.6500</pPIS><vPIS>0.18</vPIS></PISOutr></PIS><COFINS><COFINSOutr><CST>49</CST><vBC>0.00</vBC><pCOFINS>7.6000</pCOFINS><vCOFINS>0.85</vCOFINS></COFINSOutr></COFINS></imposto></det><total><ICMSTot><vBC>0.00</vBC><vICMS>0.00</vICMS><vICMSDeson>0.00</vICMSDeson><vFCP>0.00</vFCP><vBCST>0.00</vBCST><vST>0.00</vST><vFCPST>0.00</vFCPST><vFCPSTRet>0.00</vFCPSTRet><vProd>11.20</vProd><vFrete>0.00</vFrete><vSeg>0.00</vSeg><vDesc>0.00</vDesc><vII>0.00</vII><vIPI>0.00</vIPI><vIPIDevol>0.00</vIPIDevol><vPIS>0.18</vPIS><vCOFINS>0.85</vCOFINS><vOutro>0.00</vOutro><vNF>11.20</vNF><vTotTrib>0.47</vTotTrib></ICMSTot></total><transp><modFrete>9</modFrete></transp><pag><detPag><tPag>01</tPag><vPag>11.20</vPag></detPag></pag><infAdic><infCpl>AGRADECEMOS A PREFERENCIA!!;</infCpl></infAdic><infRespTec><CNPJ>47607257000170</CNPJ><xContato>Jose Valdomiro da Silva Santos</xContato><email>contato@hilltecnologia.com.br</email><fone>82991741328</fone></infRespTec></infNFe><infNFeSupl><qrCode>http://homnfce.sefaz.am.gov.br/nfceweb/consultarNFCe.jsp?p=13240747607257000170650010000002071404965277|2|2|0|E3E1883917B3196792021FEC1676884A76F84262</qrCode><urlChave>www.sefaz.am.gov.br/nfce/consulta</urlChave></infNFeSupl><Signature xmlns="http://www.w3.org/2000/09/xmldsig#"><SignedInfo><CanonicalizationMethod Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/><SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"/><Reference URI="#NFe13240747607257000170650010000002071404965277"><Transforms><Transform Algorithm="http://www.w3.org/2000/09/xmldsig#enveloped-signature"/><Transform Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/></Transforms><DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"/><DigestValue>c7hXoEG61WJ2wcQ4YGozW/ltXno=</DigestValue></Reference></SignedInfo><SignatureValue>QjThi8kdooIP0/Zp2iEc5TRnGhCTW9S5H6M9caGo4k+QzTHbzp9t2rIktZoJvjdS7ZfGPtPTDu/7lhsUqdxwv8JNM1KSNhrAUVtQGN0/16Qzn8wHO6jdueJW+PPvOcUzjNFvkTnEOGBipaFSuo6gV9pxR9fvBEVE1trT4f0v9r0yAOWneKUJchQekq/tr2A47aJupL7IdEOrhr4lE5wt7e9REM8rvsG9punRn0W6oL/QKxIaVAqA/qndcGQVQSS28jA7E9obLGNv3cjjB6Sj31zNw1x2WmD4kB6vNO/vjnClJdOIzi048AtdjAmvs7qB4pfFy37tXj4ieIshai5+vQ==</SignatureValue><KeyInfo><X509Data><X509Certificate>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</X509Certificate></X509Data></KeyInfo></Signature></NFe><protNFe versao="4.00"><infProt><tpAmb>2</tpAmb><verAplic>AM4.00-NAC</verAplic><chNFe>13240747607257000170650010000002071404965277</chNFe><dhRecbto>2024-07-08T19:16:09-04:00</dhRecbto><nProt>113240011911647</nProt><digVal>c7hXoEG61WJ2wcQ4YGozW/ltXno=</digVal><cStat>100</cStat><xMotivo>Autorizado o uso da NF-e</xMotivo></infProt></protNFe></nfeProc>	NFCe\\Autorizados\\202407\\13240747607257000170650010000002071404965277-nfe.xml	\N	\N	\N	\N	\N	\N	F	\N	\N	F	\N	\N	1	0	0	T	F	F	T
973202c6-33e9-4988-88d2-da973aa6247f	62	A	E	1	c4fc16da-7250-41a6-8edb-03c8642f4bc3	6b3774e0-3983-4f75-b24d-224dda0da673	3b03568d-4f8d-4a18-95bc-8ff9a753dab1	2024-07-02 22:02:20.765	2024-07-02	F	10.50	0.00	0.00	10.50	0.00	0.00	0.00	0.00	0	0	0	0	0			 								0	  		\N	\N	0					\N		F	F	F	1	65	2024-07-02 22:02:42.84	206	1	13240747607257000170650010000002061816731888	113240011896927		<?xml version='1.0' encoding='utf-8'?><soapenv:Envelope xmlns:soapenv="http://www.w3.org/2003/05/soap-envelope"><soapenv:Body><nfeResultMsg xmlns="http://www.portalfiscal.inf.br/nfe/wsdl/NFeAutorizacao4"><retEnviNFe xmlns="http://www.portalfiscal.inf.br/nfe" versao="4.00"><tpAmb>2</tpAmb><verAplic>AM4.00-NAC</verAplic><cStat>104</cStat><xMotivo>Lote processado</xMotivo><cUF>13</cUF><dhRecbto>2024-07-02T21:02:55-04:00</dhRecbto><protNFe versao="4.00"><infProt><tpAmb>2</tpAmb><verAplic>AM4.00-NAC</verAplic><chNFe>13240747607257000170650010000002061816731888</chNFe><dhRecbto>2024-07-02T21:02:55-04:00</dhRecbto><nProt>113240011896927</nProt><digVal>vLa3dSKv9gcazLsvjYAaGFCnFxY=</digVal><cStat>100</cStat><xMotivo>Autorizado o uso da NF-e</xMotivo></infProt></protNFe></retEnviNFe></nfeResultMsg></soapenv:Body></soapenv:Envelope>	#CF:T01 B01 N01 EI10000000,00 EF10000007,15 V7,150; #CF:T03 B01 N02 EI10000000,00 EF10000002,21 V2,210; AGRADECEMOS A PREFERENCIA!!; 	<?xml version="1.0" encoding="UTF-8"?><nfeProc versao="4.00" xmlns="http://www.portalfiscal.inf.br/nfe"><NFe xmlns="http://www.portalfiscal.inf.br/nfe"><infNFe versao="4.00" Id="NFe13240747607257000170650010000002061816731888"><ide><cUF>13</cUF><cNF>81673188</cNF><natOp>VENDA</natOp><mod>65</mod><serie>1</serie><nNF>206</nNF><dhEmi>2024-07-02T22:02:43-03:00</dhEmi><tpNF>1</tpNF><idDest>1</idDest><cMunFG>1302603</cMunFG><tpImp>4</tpImp><tpEmis>1</tpEmis><cDV>8</cDV><tpAmb>2</tpAmb><finNFe>1</finNFe><indFinal>1</indFinal><indPres>1</indPres><procEmi>0</procEmi><verProc>HillPDV 1.0.0.0</verProc></ide><emit><CNPJ>47607257000170</CNPJ><xNome>Hill Tecnologia LTDA</xNome><xFant>Hill Tecnologia</xFant><enderEmit><xLgr>Distrito Povoado Gulandim</xLgr><nro>74</nro><xBairro>CENTRO</xBairro><cMun>1302603</cMun><xMun>MANAUS</xMun><UF>AM</UF><CEP>00000000</CEP><cPais>1058</cPais><xPais>BRASIL</xPais><fone>8299999999</fone></enderEmit><IE>241048010</IE><CRT>3</CRT></emit><det nItem="1"><prod><cProd>1</cProd><cEAN>SEM GTIN</cEAN><xProd>NOTA FISCAL EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL</xProd><NCM>27101259</NCM><CEST>0600200</CEST><CFOP>5656</CFOP><uCom>LT</uCom><qCom>7.1500</qCom><vUnCom>1.1200000000</vUnCom><vProd>8.02</vProd><cEANTrib>SEM GTIN</cEANTrib><uTrib>LT</uTrib><qTrib>7.1500</qTrib><vUnTrib>1.1200000000</vUnTrib><indTot>1</indTot><comb><cProdANP>320101001</cProdANP><descANP>GASOLINA A COMUM</descANP><qTemp>7.1500</qTemp><UFCons>AM</UFCons><encerrante><nBico>1</nBico><nTanque>1</nTanque><vEncIni>10000000.000</vEncIni><vEncFin>10000007.150</vEncFin></encerrante></comb></prod><imposto><vTotTrib>3.25</vTotTrib><ICMS><ICMS60><orig>0</orig><CST>60</CST></ICMS60></ICMS><PIS><PISOutr><CST>49</CST><vBC>0.00</vBC><pPIS>1.6500</pPIS><vPIS>0.13</vPIS></PISOutr></PIS><COFINS><COFINSOutr><CST>49</CST><vBC>0.00</vBC><pCOFINS>7.6000</pCOFINS><vCOFINS>0.61</vCOFINS></COFINSOutr></COFINS></imposto></det><det nItem="2"><prod><cProd>4</cProd><cEAN>SEM GTIN</cEAN><xProd>DIESEL S10</xProd><NCM>27101259</NCM><CEST>0600200</CEST><CFOP>5656</CFOP><uCom>LT</uCom><qCom>2.2100</qCom><vUnCom>1.1200000000</vUnCom><vProd>2.48</vProd><cEANTrib>SEM GTIN</cEANTrib><uTrib>LT</uTrib><qTrib>2.2100</qTrib><vUnTrib>1.1200000000</vUnTrib><indTot>1</indTot><comb><cProdANP>420105001</cProdANP><descANP>OLEO DIESEL A S10</descANP><qTemp>2.2100</qTemp><UFCons>AM</UFCons><encerrante><nBico>2</nBico><nTanque>3</nTanque><vEncIni>10000000.000</vEncIni><vEncFin>10000002.210</vEncFin></encerrante></comb></prod><imposto><vTotTrib>1.00</vTotTrib><ICMS><ICMS60><orig>0</orig><CST>60</CST></ICMS60></ICMS><PIS><PISOutr><CST>49</CST><vBC>0.00</vBC><pPIS>1.6500</pPIS><vPIS>0.04</vPIS></PISOutr></PIS><COFINS><COFINSOutr><CST>49</CST><vBC>0.00</vBC><pCOFINS>7.6000</pCOFINS><vCOFINS>0.19</vCOFINS></COFINSOutr></COFINS></imposto></det><total><ICMSTot><vBC>0.00</vBC><vICMS>0.00</vICMS><vICMSDeson>0.00</vICMSDeson><vFCP>0.00</vFCP><vBCST>0.00</vBCST><vST>0.00</vST><vFCPST>0.00</vFCPST><vFCPSTRet>0.00</vFCPSTRet><vProd>10.50</vProd><vFrete>0.00</vFrete><vSeg>0.00</vSeg><vDesc>0.00</vDesc><vII>0.00</vII><vIPI>0.00</vIPI><vIPIDevol>0.00</vIPIDevol><vPIS>0.17</vPIS><vCOFINS>0.80</vCOFINS><vOutro>0.00</vOutro><vNF>10.50</vNF><vTotTrib>4.25</vTotTrib></ICMSTot></total><transp><modFrete>9</modFrete></transp><pag><detPag><tPag>01</tPag><vPag>10.50</vPag></detPag></pag><infAdic><infCpl>#CF:T01 B01 N01 EI10000000,00 EF10000007,15 V7,150; #CF:T03 B01 N02 EI10000000,00 EF10000002,21 V2,210; AGRADECEMOS A PREFERENCIA!!;</infCpl></infAdic><infRespTec><CNPJ>47607257000170</CNPJ><xContato>Jose Valdomiro da Silva Santos</xContato><email>contato@hilltecnologia.com.br</email><fone>82991741328</fone></infRespTec></infNFe><infNFeSupl><qrCode>http://homnfce.sefaz.am.gov.br/nfceweb/consultarNFCe.jsp?p=13240747607257000170650010000002061816731888|2|2|0|EC02D68F921702A9BBAA4BD8FCDBA361CC14289A</qrCode><urlChave>www.sefaz.am.gov.br/nfce/consulta</urlChave></infNFeSupl><Signature xmlns="http://www.w3.org/2000/09/xmldsig#"><SignedInfo><CanonicalizationMethod Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/><SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"/><Reference URI="#NFe13240747607257000170650010000002061816731888"><Transforms><Transform Algorithm="http://www.w3.org/2000/09/xmldsig#enveloped-signature"/><Transform Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/></Transforms><DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"/><DigestValue>vLa3dSKv9gcazLsvjYAaGFCnFxY=</DigestValue></Reference></SignedInfo><SignatureValue>c7i+SrZlfLDc2bUzZzy4f1M7aVyzdafQHmda3uA+0kbzmlV+1GCeLjM39o3LbMMUT0XTNBLZx9nl+mXkdqMc8qCJ2+6edL/4UHTdqmfs+KC3SsGDIW3Oam+sNg6ddLd9d0VPJW8KxFP1fJRDmieke5mPQGQtwz1GWNr+8MVqhrUmkbbSYlQ+8qP8ht8nuu7KvoR6xI4ToKoA9b5V1C2n6ojJMowCV5ZA6UbC6qEs7uNK7Cq8Hc4g5s2ttpt1C5axH9jW+ASiVR5Jq2YRrUN9tkccm6WSNfxURCNfSiifKY/YVwXPyAU1zg0A0IjjJ69esKZJz0Y5X/pZ+RHrF+BBEQ==</SignatureValue><KeyInfo><X509Data><X509Certificate>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</X509Certificate></X509Data></KeyInfo></Signature></NFe><protNFe versao="4.00"><infProt><tpAmb>2</tpAmb><verAplic>AM4.00-NAC</verAplic><chNFe>13240747607257000170650010000002061816731888</chNFe><dhRecbto>2024-07-02T21:02:55-04:00</dhRecbto><nProt>113240011896927</nProt><digVal>vLa3dSKv9gcazLsvjYAaGFCnFxY=</digVal><cStat>100</cStat><xMotivo>Autorizado o uso da NF-e</xMotivo></infProt></protNFe></nfeProc>	NFCe\\Autorizados\\202407\\13240747607257000170650010000002061816731888-nfe.xml	\N	\N	\N	\N	\N	\N	F	\N	\N	F	\N	\N	1	0	0	T	F	F	T
50b385ff-28f4-4e9a-ba1b-7053f726cbaa	66	A	E	1	c4fc16da-7250-41a6-8edb-03c8642f4bc3	220e4c12-4692-4bed-8b2a-bf6c187d262e	de25a878-5829-4340-a639-dc06a4edaf2d	2024-07-15 21:26:21.416	2024-07-15	F	11.20	0.00	0.00	11.20	0.00	0.00	0.00	0.00	0	0	0	0	0	49579252084		 								1302603	  		\N	\N	0					\N		F	F	F	1	65	2024-07-15 21:26:51.561	210	1	13240747607257000170650010000002101796314643	113240011936104		<?xml version='1.0' encoding='utf-8'?><soapenv:Envelope xmlns:soapenv="http://www.w3.org/2003/05/soap-envelope"><soapenv:Body><nfeResultMsg xmlns="http://www.portalfiscal.inf.br/nfe/wsdl/NFeAutorizacao4"><retEnviNFe xmlns="http://www.portalfiscal.inf.br/nfe" versao="4.00"><tpAmb>2</tpAmb><verAplic>AM4.00-NAC</verAplic><cStat>104</cStat><xMotivo>Lote processado</xMotivo><cUF>13</cUF><dhRecbto>2024-07-15T20:27:07-04:00</dhRecbto><protNFe versao="4.00"><infProt><tpAmb>2</tpAmb><verAplic>AM4.00-NAC</verAplic><chNFe>13240747607257000170650010000002101796314643</chNFe><dhRecbto>2024-07-15T20:27:07-04:00</dhRecbto><nProt>113240011936104</nProt><digVal>VMCTBgyGCLXwcJoXclRen4dOE1E=</digVal><cStat>100</cStat><xMotivo>Autorizado o uso da NF-e</xMotivo></infProt></protNFe></retEnviNFe></nfeResultMsg></soapenv:Body></soapenv:Envelope>	AGRADECEMOS A PREFERENCIA!!; 	<?xml version="1.0" encoding="UTF-8"?><nfeProc versao="4.00" xmlns="http://www.portalfiscal.inf.br/nfe"><NFe xmlns="http://www.portalfiscal.inf.br/nfe"><infNFe versao="4.00" Id="NFe13240747607257000170650010000002101796314643"><ide><cUF>13</cUF><cNF>79631464</cNF><natOp>VENDA</natOp><mod>65</mod><serie>1</serie><nNF>210</nNF><dhEmi>2024-07-15T21:26:51-03:00</dhEmi><tpNF>1</tpNF><idDest>1</idDest><cMunFG>1302603</cMunFG><tpImp>4</tpImp><tpEmis>1</tpEmis><cDV>3</cDV><tpAmb>2</tpAmb><finNFe>1</finNFe><indFinal>1</indFinal><indPres>1</indPres><procEmi>0</procEmi><verProc>HillPDV 1.0.0.0</verProc></ide><emit><CNPJ>47607257000170</CNPJ><xNome>Hill Tecnologia LTDA</xNome><xFant>Hill Tecnologia</xFant><enderEmit><xLgr>Distrito Povoado Gulandim</xLgr><nro>74</nro><xBairro>CENTRO</xBairro><cMun>1302603</cMun><xMun>MANAUS</xMun><UF>AM</UF><CEP>00000000</CEP><cPais>1058</cPais><xPais>BRASIL</xPais><fone>8299999999</fone></enderEmit><IE>241048010</IE><CRT>3</CRT></emit><dest><CPF>49579252084</CPF><xNome>NF-E EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL</xNome><indIEDest>9</indIEDest></dest><det nItem="1"><prod><cProd>2</cProd><cEAN>SEM GTIN</cEAN><xProd>NOTA FISCAL EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL</xProd><NCM>29062910</NCM><CEST>2803700</CEST><CFOP>5656</CFOP><uCom>UN</uCom><qCom>1.0000</qCom><vUnCom>11.2000000000</vUnCom><vProd>11.20</vProd><cEANTrib>SEM GTIN</cEANTrib><uTrib>UN</uTrib><qTrib>1.0000</qTrib><vUnTrib>11.2000000000</vUnTrib><indTot>1</indTot></prod><imposto><vTotTrib>0.47</vTotTrib><ICMS><ICMS60><orig>0</orig><CST>60</CST></ICMS60></ICMS><PIS><PISOutr><CST>49</CST><vBC>0.00</vBC><pPIS>1.6500</pPIS><vPIS>0.18</vPIS></PISOutr></PIS><COFINS><COFINSOutr><CST>49</CST><vBC>0.00</vBC><pCOFINS>7.6000</pCOFINS><vCOFINS>0.85</vCOFINS></COFINSOutr></COFINS></imposto></det><total><ICMSTot><vBC>0.00</vBC><vICMS>0.00</vICMS><vICMSDeson>0.00</vICMSDeson><vFCP>0.00</vFCP><vBCST>0.00</vBCST><vST>0.00</vST><vFCPST>0.00</vFCPST><vFCPSTRet>0.00</vFCPSTRet><vProd>11.20</vProd><vFrete>0.00</vFrete><vSeg>0.00</vSeg><vDesc>0.00</vDesc><vII>0.00</vII><vIPI>0.00</vIPI><vIPIDevol>0.00</vIPIDevol><vPIS>0.18</vPIS><vCOFINS>0.85</vCOFINS><vOutro>0.00</vOutro><vNF>11.20</vNF><vTotTrib>0.47</vTotTrib></ICMSTot></total><transp><modFrete>9</modFrete></transp><pag><detPag><tPag>01</tPag><vPag>11.20</vPag></detPag></pag><infAdic><infCpl>AGRADECEMOS A PREFERENCIA!!;</infCpl></infAdic><infRespTec><CNPJ>47607257000170</CNPJ><xContato>Jose Valdomiro da Silva Santos</xContato><email>contato@hilltecnologia.com.br</email><fone>82991741328</fone></infRespTec></infNFe><infNFeSupl><qrCode>http://homnfce.sefaz.am.gov.br/nfceweb/consultarNFCe.jsp?p=13240747607257000170650010000002101796314643|2|2|0|C333C9DE4E113920865425893B0FA7D1C95211E2</qrCode><urlChave>www.sefaz.am.gov.br/nfce/consulta</urlChave></infNFeSupl><Signature xmlns="http://www.w3.org/2000/09/xmldsig#"><SignedInfo><CanonicalizationMethod Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/><SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"/><Reference URI="#NFe13240747607257000170650010000002101796314643"><Transforms><Transform Algorithm="http://www.w3.org/2000/09/xmldsig#enveloped-signature"/><Transform Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/></Transforms><DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"/><DigestValue>VMCTBgyGCLXwcJoXclRen4dOE1E=</DigestValue></Reference></SignedInfo><SignatureValue>Lza8Bqj1tim/84GvGT7tZmzIFgr+Rquglvb7SSottNiurzaXdyjaFY/kFcAtQhsp6bIHFehVmfVuuubdwRT2+8jt0cTlQjCoztl1ljcszmMMVTyo4IYBrIoSZUPxVBUGjASiXF3Nj0QOtPmS9KEt9W5/kFhGrmbiPbMcLc37KuDVUPrfnocpVtmTFSGJJsOzKQ8awDYzwxxaUoGbSOrrKyUO+3MIhgWPteo6gjU2+Ksp0NN9yRKRpcStIOBElHaFecbwCL2V1fypKOPtpQLJta2D9dfTlk0mQJeuIkMAuOmyhiUkc2iZkq8rz2PHMucrOkqlfWWJHD4xc1BHuVx4Nw==</SignatureValue><KeyInfo><X509Data><X509Certificate>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</X509Certificate></X509Data></KeyInfo></Signature></NFe><protNFe versao="4.00"><infProt><tpAmb>2</tpAmb><verAplic>AM4.00-NAC</verAplic><chNFe>13240747607257000170650010000002101796314643</chNFe><dhRecbto>2024-07-15T20:27:07-04:00</dhRecbto><nProt>113240011936104</nProt><digVal>VMCTBgyGCLXwcJoXclRen4dOE1E=</digVal><cStat>100</cStat><xMotivo>Autorizado o uso da NF-e</xMotivo></infProt></protNFe></nfeProc>	NFCe\\Autorizados\\202407\\13240747607257000170650010000002101796314643-nfe.xml	\N	\N	\N	\N	\N	\N	F	\N	\N	F	\N	\N	1	0	0	T	F	F	T
892cf0aa-c767-4c3c-8cce-7546bd4aa772	61	A	E	1	c4fc16da-7250-41a6-8edb-03c8642f4bc3	6b3774e0-3983-4f75-b24d-224dda0da673	3b03568d-4f8d-4a18-95bc-8ff9a753dab1	2024-05-31 23:01:36.611	2024-05-31	F	10.07	0.00	0.00	10.07	0.00	0.00	0.00	0.00	0	0	0	0	0			 								0	  		\N	\N	0					\N		F	F	F	1	65	2024-05-31 23:01:41.927	205	1	13240547607257000170650010000002051667343321	113240011714427		<?xml version='1.0' encoding='utf-8'?><soapenv:Envelope xmlns:soapenv="http://www.w3.org/2003/05/soap-envelope"><soapenv:Body><nfeResultMsg xmlns="http://www.portalfiscal.inf.br/nfe/wsdl/NFeAutorizacao4"><retEnviNFe xmlns="http://www.portalfiscal.inf.br/nfe" versao="4.00"><tpAmb>2</tpAmb><verAplic>AM4.00-NAC</verAplic><cStat>104</cStat><xMotivo>Lote processado</xMotivo><cUF>13</cUF><dhRecbto>2024-05-31T22:01:45-04:00</dhRecbto><protNFe versao="4.00"><infProt><tpAmb>2</tpAmb><verAplic>AM4.00-NAC</verAplic><chNFe>13240547607257000170650010000002051667343321</chNFe><dhRecbto>2024-05-31T22:01:45-04:00</dhRecbto><nProt>113240011714427</nProt><digVal>SZjTlNenfy3OpJ3E7f9i7EZ6dNw=</digVal><cStat>100</cStat><xMotivo>Autorizado o uso da NF-e</xMotivo></infProt></protNFe></retEnviNFe></nfeResultMsg></soapenv:Body></soapenv:Envelope>	#CF:T01 B01 N01 EI10000007,15 EF10000016,12 V8,991; AGRADECEMOS A PREFERENCIA!!; 	<?xml version="1.0" encoding="UTF-8"?><nfeProc versao="4.00" xmlns="http://www.portalfiscal.inf.br/nfe"><NFe xmlns="http://www.portalfiscal.inf.br/nfe"><infNFe versao="4.00" Id="NFe13240547607257000170650010000002051667343321"><ide><cUF>13</cUF><cNF>66734332</cNF><natOp>VENDA</natOp><mod>65</mod><serie>1</serie><nNF>205</nNF><dhEmi>2024-05-31T23:01:41-03:00</dhEmi><tpNF>1</tpNF><idDest>1</idDest><cMunFG>1302603</cMunFG><tpImp>4</tpImp><tpEmis>1</tpEmis><cDV>1</cDV><tpAmb>2</tpAmb><finNFe>1</finNFe><indFinal>1</indFinal><indPres>1</indPres><procEmi>0</procEmi><verProc>HillPDV 1.0.0.0</verProc></ide><emit><CNPJ>47607257000170</CNPJ><xNome>Hill Tecnologia LTDA</xNome><xFant>Hill Tecnologia</xFant><enderEmit><xLgr>Distrito Povoado Gulandim</xLgr><nro>74</nro><xBairro>CENTRO</xBairro><cMun>1302603</cMun><xMun>MANAUS</xMun><UF>AM</UF><CEP>00000000</CEP><cPais>1058</cPais><xPais>BRASIL</xPais><fone>8299999999</fone></enderEmit><IE>241048010</IE><CRT>3</CRT></emit><det nItem="1"><prod><cProd>1</cProd><cEAN>SEM GTIN</cEAN><xProd>NOTA FISCAL EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL</xProd><NCM>27101259</NCM><CEST>0600200</CEST><CFOP>5656</CFOP><uCom>LT</uCom><qCom>8.9911</qCom><vUnCom>1.1200000000</vUnCom><vProd>10.07</vProd><cEANTrib>SEM GTIN</cEANTrib><uTrib>LT</uTrib><qTrib>8.9911</qTrib><vUnTrib>1.1200000000</vUnTrib><indTot>1</indTot><comb><cProdANP>320101001</cProdANP><descANP>GASOLINA A COMUM</descANP><qTemp>8.9911</qTemp><UFCons>AM</UFCons><encerrante><nBico>1</nBico><nTanque>1</nTanque><vEncIni>10000007.150</vEncIni><vEncFin>10000016.120</vEncFin></encerrante></comb></prod><imposto><vTotTrib>4.07</vTotTrib><ICMS><ICMS60><orig>0</orig><CST>60</CST></ICMS60></ICMS><PIS><PISOutr><CST>49</CST><vBC>0.00</vBC><pPIS>1.6500</pPIS><vPIS>0.17</vPIS></PISOutr></PIS><COFINS><COFINSOutr><CST>49</CST><vBC>0.00</vBC><pCOFINS>7.6000</pCOFINS><vCOFINS>0.77</vCOFINS></COFINSOutr></COFINS></imposto></det><total><ICMSTot><vBC>0.00</vBC><vICMS>0.00</vICMS><vICMSDeson>0.00</vICMSDeson><vFCP>0.00</vFCP><vBCST>0.00</vBCST><vST>0.00</vST><vFCPST>0.00</vFCPST><vFCPSTRet>0.00</vFCPSTRet><vProd>10.07</vProd><vFrete>0.00</vFrete><vSeg>0.00</vSeg><vDesc>0.00</vDesc><vII>0.00</vII><vIPI>0.00</vIPI><vIPIDevol>0.00</vIPIDevol><vPIS>0.17</vPIS><vCOFINS>0.77</vCOFINS><vOutro>0.00</vOutro><vNF>10.07</vNF><vTotTrib>4.07</vTotTrib></ICMSTot></total><transp><modFrete>9</modFrete></transp><pag><detPag><tPag>01</tPag><vPag>10.07</vPag></detPag></pag><infAdic><infCpl>#CF:T01 B01 N01 EI10000007,15 EF10000016,12 V8,991; AGRADECEMOS A PREFERENCIA!!;</infCpl></infAdic><infRespTec><CNPJ>47607257000170</CNPJ><xContato>Jose Valdomiro da Silva Santos</xContato><email>contato@hilltecnologia.com.br</email><fone>82991741328</fone></infRespTec></infNFe><infNFeSupl><qrCode>http://homnfce.sefaz.am.gov.br/nfceweb/consultarNFCe.jsp?p=13240547607257000170650010000002051667343321|2|2|0|501A0146A9B01356004958E2266971C3661DF270</qrCode><urlChave>www.sefaz.am.gov.br/nfce/consulta</urlChave></infNFeSupl><Signature xmlns="http://www.w3.org/2000/09/xmldsig#"><SignedInfo><CanonicalizationMethod Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/><SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"/><Reference URI="#NFe13240547607257000170650010000002051667343321"><Transforms><Transform Algorithm="http://www.w3.org/2000/09/xmldsig#enveloped-signature"/><Transform Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/></Transforms><DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"/><DigestValue>SZjTlNenfy3OpJ3E7f9i7EZ6dNw=</DigestValue></Reference></SignedInfo><SignatureValue>iAavkQOq5XmR2qQ7jpOh/g2lQgdaelttoik8RrdRDKH8svuDP5n8ryyFOzZl5sxOLvDlz8rrvH5tyC6uuVPDT4aBRoREqaKZm7sMgEb+PHq8ZJx4nZhG+9UM0JjV7Mx8AtJI2LXp0Ol/4hwBV8RIuC3NXwd4xqcY7Uhdb+Qijceq2fCrWIuZAEmKTF/fRVzhbCm4+37UedDr9C70desgDCO0bLF5O/j2Ozs9GUp6VGVJC9jNTLggiGNmpr79Int7kMQHeUI9FdYAfHMmeoXploVFXTmCPS/e9jaBCoIZsOWXC1EDcD9eXxBpm47GpPxgRXt4P1N8shwvN3yhLOsY4w==</SignatureValue><KeyInfo><X509Data><X509Certificate>MIIH9TCCBd2gAwIBAgIIPrmkAgdC5GkwDQYJKoZIhvcNAQELBQAwdTELMAkGA1UEBhMCQlIxEzARBgNVBAoMCklDUC1CcmFzaWwxNjA0BgNVBAsMLVNlY3JldGFyaWEgZGEgUmVjZWl0YSBGZWRlcmFsIGRvIEJyYXNpbCAtIFJGQjEZMBcGA1UEAwwQQUMgU0VSQVNBIFJGQiB2NTAeFw0yMzA5MjgyMzAxMDBaFw0yNDA5MjcyMzAwNTlaMIIBCjELMAkGA1UEBhMCQlIxCzAJBgNVBAgMAkFMMQ8wDQYDVQQHDAZNYWNlaW8xEzARBgNVBAoMCklDUC1CcmFzaWwxNjA0BgNVBAsMLVNlY3JldGFyaWEgZGEgUmVjZWl0YSBGZWRlcmFsIGRvIEJyYXNpbCAtIFJGQjEWMBQGA1UECwwNUkZCIGUtQ05QSiBBMTEWMBQGA1UECwwNQUMgU0VSQVNBIFJGQjEXMBUGA1UECwwOMjkwOTE1NzEwMDAxNjAxGTAXBgNVBAsMEFZJREVPQ09ORkVSRU5DSUExLDAqBgNVBAMMI0hJTEwgVEVDTk9MT0dJQSBMVERBOjQ3NjA3MjU3MDAwMTcwMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEArKy+UV4uAArHXS0EcGW5d6WUZ4ZJgUcJ9VFGCxLfOSFlgH5hUgiNSeJsNYt+OFPh35uyt/vHljkHB1+dDjk7h0/i1Qs6dbufcI07RYCy5xOytce7Xpzcx/2m3vVYnhUAUmG8Ma68vj1VivDjA8z+3olqhNKTExWhLlmI9xg06SEbq9gSe8uEB/TDqTig+8xBtOA8hJwv+AVs2Yw3kjwq9UVklNozrdvefJxZzvWXltzNaHns6LUt90lUQ84ySTDxa8VMoSfTHyLx2ecMRi6eBkhEoILUo6ar33lGdr9EJUL7ncDSaT2Ud7H3m5jg5/R+SAZX3FkfMKKrCmWPTQ0AjQIDAQABo4IC8DCCAuwwCQYDVR0TBAIwADAfBgNVHSMEGDAWgBTs8UFRV6jmOules6Ai+QiKtTqHjzCBmQYIKwYBBQUHAQEEgYwwgYkwSAYIKwYBBQUHMAKGPGh0dHA6Ly93d3cuY2VydGlmaWNhZG9kaWdpdGFsLmNvbS5ici9jYWRlaWFzL3NlcmFzYXJmYnY1LnA3YjA9BggrBgEFBQcwAYYxaHR0cDovL29jc3AuY2VydGlmaWNhZG9kaWdpdGFsLmNvbS5ici9zZXJhc2FyZmJ2NTCBwAYDVR0RBIG4MIG1gRRKVkFMU1NJTFZBQEdNQUlMLkNPTaApBgVgTAEDAqAgEx5KT1NFIFZBTERPTUlSTyBEQSBTSUxWQSBTQU5UT1OgGQYFYEwBAwOgEBMONDc2MDcyNTcwMDAxNzCgPgYFYEwBAwSgNRMzMDQwNTE5ODUwNTQwNzQ1MjQzMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwoBcGBWBMAQMHoA4TDDAwMDAwMDAwMDAwMDBxBgNVHSAEajBoMGYGBmBMAQIBDTBcMFoGCCsGAQUFBwIBFk5odHRwOi8vcHVibGljYWNhby5jZXJ0aWZpY2Fkb2RpZ2l0YWwuY29tLmJyL3JlcG9zaXRvcmlvL2RwYy9kZWNsYXJhY2FvLXJmYi5wZGYwHQYDVR0lBBYwFAYIKwYBBQUHAwIGCCsGAQUFBwMEMIGdBgNVHR8EgZUwgZIwSqBIoEaGRGh0dHA6Ly93d3cuY2VydGlmaWNhZG9kaWdpdGFsLmNvbS5ici9yZXBvc2l0b3Jpby9sY3Ivc2VyYXNhcmZidjUuY3JsMESgQqBAhj5odHRwOi8vbGNyLmNlcnRpZmljYWRvcy5jb20uYnIvcmVwb3NpdG9yaW8vbGNyL3NlcmFzYXJmYnY1LmNybDAdBgNVHQ4EFgQUdxEGJa7aGvMQja6hX29Vn3V90AowDgYDVR0PAQH/BAQDAgXgMA0GCSqGSIb3DQEBCwUAA4ICAQA0b/qWjMEMuzBD13rSkfkQpXdsiK5xGawT4bHp40faDCeICOwXJVv0ozZ7dEFyZxsrA75+410uovbQqlqviR0WrxECjWuFoUPRBM7ehUOC2w4EhcueYivSmsUIY1Va67PyhF3yD/QYoG2t14bDHh3sVI0WDGwrWi5YVh0RPKiNHBFDrMLL0SgIlMVN/idTB3yolxq+2hn/5pUoqe9J+rs7TZl0wuccoX8icyp9/psew5HpHb3ustpGAgOBbz3DRA+VSVoJdLBSPXSbF23WQmbI8KBOA2extQeWiBKwk+nx0VdOuiawKj99QS4iFsj1I+2MeNSEUR6pfG0tj4sUUUc934SRXcxBKPSGZulrK4+ojXgRERi6HywbtCTbqz6pBt8lhimLqKMHyhxMUjujNk895HCk/YGUPQas2CBddY5fGhzldfXq/gobWesAM0KrQj4YWrk8JCSK5Ilf7CgdK661+NbUKi1bDexE060nGgplQECBqvMFKYLuCPG/6kiDc29DdtgDb6M6q+yZm2VXsyQ91f+NaFKP6eTUOEo3eSfb1q8mu/DimUjHoZUy9PTnXvCWwC8LTWR789+POX6vgP/1yhsjuZa41IT4wRItb4ssuscG9zBDMPlw/TXEazwsRVPDmR8hCKiUzSkTelupct5xHrvq3pGomOwtmzaYy2wglA==</X509Certificate></X509Data></KeyInfo></Signature></NFe><protNFe versao="4.00"><infProt><tpAmb>2</tpAmb><verAplic>AM4.00-NAC</verAplic><chNFe>13240547607257000170650010000002051667343321</chNFe><dhRecbto>2024-05-31T22:01:45-04:00</dhRecbto><nProt>113240011714427</nProt><digVal>SZjTlNenfy3OpJ3E7f9i7EZ6dNw=</digVal><cStat>100</cStat><xMotivo>Autorizado o uso da NF-e</xMotivo></infProt></protNFe></nfeProc>	NFCe\\Autorizados\\202405\\13240547607257000170650010000002051667343321-nfe.xml	\N	\N	2024-05-31 00:00:00	TESTE 1234567890 12345	113240011714428	\N	F	\N	\N	F	\N	\N	1	0	0	T	F	F	T
eec7eca1-00bd-479b-bc5e-2e9da4993a53	67	A	E	1	c4fc16da-7250-41a6-8edb-03c8642f4bc3	220e4c12-4692-4bed-8b2a-bf6c187d262e	de25a878-5829-4340-a639-dc06a4edaf2d	2024-07-15 21:27:23.026	2024-07-15	F	3.89	0.00	0.00	3.89	0.00	0.00	0.00	0.00	0	0	0	0	0			 								0	  		\N	\N	0					\N		T	T	T	1	65	2024-07-15 21:40:50.461	212	1	13240747607257000170650010000002129416880650				AGRADECEMOS A PREFERENCIA!!; 	<?xml version="1.0" encoding="UTF-8"?><NFe xmlns="http://www.portalfiscal.inf.br/nfe"><infNFe versao="4.00" Id="NFe13240747607257000170650010000002129416880650"><ide><cUF>13</cUF><cNF>41688065</cNF><natOp>VENDA</natOp><mod>65</mod><serie>1</serie><nNF>212</nNF><dhEmi>2024-07-15T21:40:50-03:00</dhEmi><tpNF>1</tpNF><idDest>1</idDest><cMunFG>1302603</cMunFG><tpImp>4</tpImp><tpEmis>9</tpEmis><cDV>0</cDV><tpAmb>2</tpAmb><finNFe>1</finNFe><indFinal>1</indFinal><indPres>1</indPres><procEmi>0</procEmi><verProc>HillPDV 1.0.0.0</verProc><dhCont>2024-07-15T21:40:50-03:00</dhCont><xJust>EMITIDA EM CONTINGENCIA EM DECORRENCIA DE PROBLEMAS TECNICOS</xJust></ide><emit><CNPJ>47607257000170</CNPJ><xNome>Hill Tecnologia LTDA</xNome><xFant>Hill Tecnologia</xFant><enderEmit><xLgr>Distrito Povoado Gulandim</xLgr><nro>74</nro><xBairro>CENTRO</xBairro><cMun>1302603</cMun><xMun>MANAUS</xMun><UF>AM</UF><CEP>00000000</CEP><cPais>1058</cPais><xPais>BRASIL</xPais><fone>8299999999</fone></enderEmit><IE>241048010</IE><CRT>3</CRT></emit><det nItem="1"><prod><cProd>6</cProd><cEAN>SEM GTIN</cEAN><xProd>NOTA FISCAL EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL</xProd><NCM>22011000</NCM><CEST>0300500</CEST><EXTIPI>02</EXTIPI><CFOP>5102</CFOP><uCom>UN</uCom><qCom>1.0000</qCom><vUnCom>3.8900000000</vUnCom><vProd>3.89</vProd><cEANTrib>SEM GTIN</cEANTrib><uTrib>UN</uTrib><qTrib>1.0000</qTrib><vUnTrib>3.8900000000</vUnTrib><indTot>1</indTot></prod><imposto><vTotTrib>1.18</vTotTrib><ICMS><ICMS00><orig>0</orig><CST>00</CST><modBC>0</modBC><vBC>0.00</vBC><pICMS>17.0000</pICMS><vICMS>0.66</vICMS></ICMS00></ICMS><PIS><PISOutr><CST>49</CST><vBC>0.00</vBC><pPIS>1.6500</pPIS><vPIS>0.06</vPIS></PISOutr></PIS><COFINS><COFINSOutr><CST>49</CST><vBC>0.00</vBC><pCOFINS>7.6000</pCOFINS><vCOFINS>0.30</vCOFINS></COFINSOutr></COFINS></imposto></det><total><ICMSTot><vBC>0.00</vBC><vICMS>0.66</vICMS><vICMSDeson>0.00</vICMSDeson><vFCP>0.00</vFCP><vBCST>0.00</vBCST><vST>0.00</vST><vFCPST>0.00</vFCPST><vFCPSTRet>0.00</vFCPSTRet><vProd>3.89</vProd><vFrete>0.00</vFrete><vSeg>0.00</vSeg><vDesc>0.00</vDesc><vII>0.00</vII><vIPI>0.00</vIPI><vIPIDevol>0.00</vIPIDevol><vPIS>0.06</vPIS><vCOFINS>0.30</vCOFINS><vOutro>0.00</vOutro><vNF>3.89</vNF><vTotTrib>1.18</vTotTrib></ICMSTot></total><transp><modFrete>9</modFrete></transp><pag><detPag><tPag>01</tPag><vPag>3.89</vPag></detPag></pag><infAdic><infCpl>AGRADECEMOS A PREFERENCIA!!;</infCpl></infAdic><infRespTec><CNPJ>47607257000170</CNPJ><xContato>Jose Valdomiro da Silva Santos</xContato><email>contato@hilltecnologia.com.br</email><fone>82991741328</fone></infRespTec></infNFe><infNFeSupl><qrCode>http://homnfce.sefaz.am.gov.br/nfceweb/consultarNFCe.jsp?p=13240747607257000170650010000002129416880650|2|2|15|3.89|39302B773433644D54722B65734C4762444750384D6B6F2B47554D3D|0|EC30E9F0AAB44EE32EA54AFE508C4E1358CB6F9C</qrCode><urlChave>www.sefaz.am.gov.br/nfce/consulta</urlChave></infNFeSupl><Signature xmlns="http://www.w3.org/2000/09/xmldsig#"><SignedInfo><CanonicalizationMethod Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/><SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"/><Reference URI="#NFe13240747607257000170650010000002129416880650"><Transforms><Transform Algorithm="http://www.w3.org/2000/09/xmldsig#enveloped-signature"/><Transform Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/></Transforms><DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"/><DigestValue>90+w43dMTr+esLGbDGP8Mko+GUM=</DigestValue></Reference></SignedInfo><SignatureValue>eB1f/O+2Gcx2hH5CbpV6XoStEr13veZes7LUN7Dc210Y+1VbHCZwCDMXYHLbBQGSyIJxXDFK8i/j8LxSyxEGop7MRt8lbf2ZHXwybkuOf0gGOSCgqsBRoJzrjFUSur26SO6HpJ+49sluXCUCu1h3R8NF0b57z2rw3+8s/j6WoAOFgw7CoKdDWiYcIAkDaRUZ4ieC+yi0Q1qCeOWpwKReq2zPLZhGdIhuFOXFtfiIUx1oqUB0PBzBpnUAf0g9XIEiRcpg8uxeNKvnQYpxYR1icUI9rQ6IYIlKcDVr9+7wDFKuvLFX6Ia1w5LKTxgzzHl+v0FSwiYZ3hicQQaXX51flw==</SignatureValue><KeyInfo><X509Data><X509Certificate>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</X509Certificate></X509Data></KeyInfo></Signature></NFe>	NFCe\\202407\\13240747607257000170650010000002129416880650-nfe.xml	\N	\N	\N	\N	\N	\N	F	\N	\N	F	\N	\N	1	0	0	T	F	F	T
05fd7a9b-f0ce-47ac-a678-eeca06de17c1	64	A	E	1	c4fc16da-7250-41a6-8edb-03c8642f4bc3	b722c3ea-d77f-4bfd-89e3-65e9728213d9	5ee6bdff-8712-4b87-b325-420923350fa5	2024-07-08 20:27:05.696	2024-07-08	F	11.20	0.00	0.00	11.20	0.00	0.00	0.00	0.00	0	0	0	0	0			 								0	  		\N	\N	0					\N		T	T	F	1	65	-infinity	209	1	13240747607257000170650010000002099721040583	113240011911691			AGRADECEMOS A PREFERENCIA!!; 	<?xml version="1.0" encoding="UTF-8"?><nfeProc versao="4.00" xmlns="http://www.portalfiscal.inf.br/nfe"><NFe xmlns="http://www.portalfiscal.inf.br/nfe"><infNFe versao="4.00" Id="NFe13240747607257000170650010000002099721040583"><ide><cUF>13</cUF><cNF>72104058</cNF><natOp>VENDA</natOp><mod>65</mod><serie>1</serie><nNF>209</nNF><dhEmi>2024-07-08T20:27:29-03:00</dhEmi><tpNF>1</tpNF><idDest>1</idDest><cMunFG>1302603</cMunFG><tpImp>4</tpImp><tpEmis>9</tpEmis><cDV>3</cDV><tpAmb>2</tpAmb><finNFe>1</finNFe><indFinal>1</indFinal><indPres>1</indPres><procEmi>0</procEmi><verProc>HillPDV 1.0.0.0</verProc><dhCont>2024-07-08T20:27:29-03:00</dhCont><xJust>EMITIDA EM CONTINGENCIA EM DECORRENCIA DE PROBLEMAS TECNICOS</xJust></ide><emit><CNPJ>47607257000170</CNPJ><xNome>Hill Tecnologia LTDA</xNome><xFant>Hill Tecnologia</xFant><enderEmit><xLgr>Distrito Povoado Gulandim</xLgr><nro>74</nro><xBairro>CENTRO</xBairro><cMun>1302603</cMun><xMun>MANAUS</xMun><UF>AM</UF><CEP>00000000</CEP><cPais>1058</cPais><xPais>BRASIL</xPais><fone>8299999999</fone></enderEmit><IE>241048010</IE><CRT>3</CRT></emit><det nItem="1"><prod><cProd>2</cProd><cEAN>SEM GTIN</cEAN><xProd>NOTA FISCAL EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL</xProd><NCM>29062910</NCM><CEST>2803700</CEST><CFOP>5656</CFOP><uCom>UN</uCom><qCom>1.0000</qCom><vUnCom>11.2000000000</vUnCom><vProd>11.20</vProd><cEANTrib>SEM GTIN</cEANTrib><uTrib>UN</uTrib><qTrib>1.0000</qTrib><vUnTrib>11.2000000000</vUnTrib><indTot>1</indTot></prod><imposto><vTotTrib>0.47</vTotTrib><ICMS><ICMS60><orig>0</orig><CST>60</CST></ICMS60></ICMS><PIS><PISOutr><CST>49</CST><vBC>0.00</vBC><pPIS>1.6500</pPIS><vPIS>0.18</vPIS></PISOutr></PIS><COFINS><COFINSOutr><CST>49</CST><vBC>0.00</vBC><pCOFINS>7.6000</pCOFINS><vCOFINS>0.85</vCOFINS></COFINSOutr></COFINS></imposto></det><total><ICMSTot><vBC>0.00</vBC><vICMS>0.00</vICMS><vICMSDeson>0.00</vICMSDeson><vFCP>0.00</vFCP><vBCST>0.00</vBCST><vST>0.00</vST><vFCPST>0.00</vFCPST><vFCPSTRet>0.00</vFCPSTRet><vProd>11.20</vProd><vFrete>0.00</vFrete><vSeg>0.00</vSeg><vDesc>0.00</vDesc><vII>0.00</vII><vIPI>0.00</vIPI><vIPIDevol>0.00</vIPIDevol><vPIS>0.18</vPIS><vCOFINS>0.85</vCOFINS><vOutro>0.00</vOutro><vNF>11.20</vNF><vTotTrib>0.47</vTotTrib></ICMSTot></total><transp><modFrete>9</modFrete></transp><pag><detPag><tPag>01</tPag><vPag>11.20</vPag></detPag></pag><infAdic><infCpl>AGRADECEMOS A PREFERENCIA!!;</infCpl></infAdic><infRespTec><CNPJ>47607257000170</CNPJ><xContato>Jose Valdomiro da Silva Santos</xContato><email>contato@hilltecnologia.com.br</email><fone>82991741328</fone></infRespTec></infNFe><infNFeSupl><qrCode>http://homnfce.sefaz.am.gov.br/nfceweb/consultarNFCe.jsp?p=13240747607257000170650010000002099721040583|2|2|08|11.20|5774437332554D746C52383177786B3846667668706B64726D796F3D|0|C8C26C314BB9D8CDD85DE947DF1234F6563A9F20</qrCode><urlChave>www.sefaz.am.gov.br/nfce/consulta</urlChave></infNFeSupl><Signature xmlns="http://www.w3.org/2000/09/xmldsig#"><SignedInfo><CanonicalizationMethod Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/><SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"/><Reference URI="#NFe13240747607257000170650010000002099721040583"><Transforms><Transform Algorithm="http://www.w3.org/2000/09/xmldsig#enveloped-signature"/><Transform Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/></Transforms><DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"/><DigestValue>WtCs2UMtlR81wxk8Ffvhpkdrmyo=</DigestValue></Reference></SignedInfo><SignatureValue>M/wcQkwIRuDGDdkXF5uZ+8I+mL6q64IqQfDK/rBVOxUwtcZFdFDp1KSPSOAd3fWoQTrD+I/ZB+WSrVj9/jdPW2Qhj4orMI2byZuoNtAbUVq0jn18pt23V0KAk5UXHOmVih4Rwwt1sII0e04KtTH46SDiAgdt5I3xZ64h+LJ1edn68+Vype0oSqHSZV0TM+ZQyClLpUnPDteaw5MuupAo2eLn2Mi/yrhWlyFRnyIzKxVfAcCP5erwaOFOMyKY07rNGrb3xu1qEQHV0vFC1lfLoz4VNqq5lBmwFdr9ubihxvSTjzQWOcLJkytPYDRUyj5Lv1YSQIMnGYRQVAuyRB0KJw==</SignatureValue><KeyInfo><X509Data><X509Certificate>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</X509Certificate></X509Data></KeyInfo></Signature></NFe><protNFe versao="4.00"><infProt><tpAmb>2</tpAmb><verAplic>AM4.00-NAC</verAplic><chNFe>13240747607257000170650010000002099721040583</chNFe><dhRecbto>2024-07-08T19:45:30-04:00</dhRecbto><nProt>113240011911691</nProt><digVal>WtCs2UMtlR81wxk8Ffvhpkdrmyo=</digVal><cStat>100</cStat><xMotivo>Autorizado o uso da NF-e</xMotivo></infProt></protNFe></nfeProc>	NFCe\\202407\\13240747607257000170650010000002099721040583-nfe.xml	\N	\N	\N	\N	\N	\N	F	\N	\N	F	\N	\N	1	0	0	T	F	F	T
a6184c19-916d-4904-a98a-b94ff33f9d24	65	C	E	1	c4fc16da-7250-41a6-8edb-03c8642f4bc3	b722c3ea-d77f-4bfd-89e3-65e9728213d9	5ee6bdff-8712-4b87-b325-420923350fa5	2024-07-08 20:27:29.336	2024-07-08	\N	0.00	0.00	0.00	0.00	0.00	0.00	0.00	0.00	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	T	T	2	65	2024-07-08 20:27:10.22	208	1	13240747607257000170650010000002081368331347	\N	\N	\N	\N	\N	\N	T	\N	\N	\N	\N	\N	F	\N	\N	F	RejeiÃ§Ã£o: Emissor nÃ£o habilitado para emissÃ£o da NF-e	\N	\N	\N	\N	F	F	F	F
39c54d64-fa35-4801-8909-921077d22c23	69	C	E	1	c4fc16da-7250-41a6-8edb-03c8642f4bc3	220e4c12-4692-4bed-8b2a-bf6c187d262e	de25a878-5829-4340-a639-dc06a4edaf2d	2024-07-15 21:41:18.198	2024-07-15	F	0.00	0.00	0.00	0.00	0.00	0.00	0.00	0.00	0	0	0	0	0			 								0	  		\N	\N	0					\N		\N	\N	F	\N	0	\N	0	0	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	F	\N	\N	F	\N	\N	1	0	0	T	F	F	T
35152064-cc3f-4a44-8be5-390ecbe07fb0	70	C	E	1	c4fc16da-7250-41a6-8edb-03c8642f4bc3	220e4c12-4692-4bed-8b2a-bf6c187d262e	de25a878-5829-4340-a639-dc06a4edaf2d	2024-07-15 21:42:46.253	2024-07-15	F	0.00	0.00	0.00	0.00	0.00	0.00	0.00	0.00	0	0	0	0	0	81732107050		 								1302603	  		\N	\N	0					\N		\N	\N	F	\N	0	\N	0	0	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	F	\N	\N	F	\N	\N	1	0	0	T	F	F	T
bb29102f-0710-4751-8eaf-9c86459b52bd	68	C	E	1	c4fc16da-7250-41a6-8edb-03c8642f4bc3	220e4c12-4692-4bed-8b2a-bf6c187d262e	de25a878-5829-4340-a639-dc06a4edaf2d	2024-07-15 21:40:50.294	2024-07-15	\N	0.00	0.00	0.00	0.00	0.00	0.00	0.00	0.00	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	T	T	1	65	2024-07-15 21:40:29.795	211	1	13240747607257000170650010000002111370907310	\N	\N	\N	\N	\N	\N	T	\N	\N	\N	\N	\N	F	\N	\N	F	RejeiÃ§Ã£o: Emissor nÃ£o habilitado para emissÃ£o da NF-e	\N	\N	\N	\N	F	F	F	F
\.


--
-- TOC entry 3766 (class 0 OID 33735)
-- Dependencies: 275
-- Data for Name: vendedores; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.vendedores (id, codigo, nome) FROM stdin;
1	1	JOSE CARLOS DOS SANTOS
2	2	FERNANDA DOS SANTOS
3	3	ADRIANO JOSE
\.


--
-- TOC entry 3767 (class 0 OID 33738)
-- Dependencies: 276
-- Data for Name: versoes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.versoes (pdv, retaguarda, concentrador, banco, client) FROM stdin;
\.


--
-- TOC entry 3768 (class 0 OID 33741)
-- Dependencies: 277
-- Data for Name: voucher; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.voucher (id, cliente_id, valor, numero, impresso) FROM stdin;
\.


--
-- TOC entry 3791 (class 0 OID 0)
-- Dependencies: 217
-- Name: abastecimentos_id_serial_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.abastecimentos_id_serial_seq', 130, true);


--
-- TOC entry 3792 (class 0 OID 0)
-- Dependencies: 220
-- Name: afericoes_id_serial_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.afericoes_id_serial_seq', 3, true);


--
-- TOC entry 3793 (class 0 OID 0)
-- Dependencies: 224
-- Name: bicos_encerrantes_id_serial_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.bicos_encerrantes_id_serial_seq', 282, true);


--
-- TOC entry 3794 (class 0 OID 0)
-- Dependencies: 226
-- Name: caixa_id_serial_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.caixa_id_serial_seq', 84, true);


--
-- TOC entry 3795 (class 0 OID 0)
-- Dependencies: 252
-- Name: sangria_suprimento_id_serial_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.sangria_suprimento_id_serial_seq', 22, true);


--
-- TOC entry 3796 (class 0 OID 0)
-- Dependencies: 261
-- Name: turno_postos_id_serial_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.turno_postos_id_serial_seq', 16, true);


--
-- TOC entry 3797 (class 0 OID 0)
-- Dependencies: 259
-- Name: turnos_id_serial_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.turnos_id_serial_seq', 16, true);


--
-- TOC entry 3798 (class 0 OID 0)
-- Dependencies: 265
-- Name: venda_cheque_trocos_id_serial_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.venda_cheque_trocos_id_serial_seq', 1, false);


--
-- TOC entry 3799 (class 0 OID 0)
-- Dependencies: 267
-- Name: venda_cheques_id_serial_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.venda_cheques_id_serial_seq', 1, false);


--
-- TOC entry 3800 (class 0 OID 0)
-- Dependencies: 269
-- Name: venda_deposito_trocos_id_serial_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.venda_deposito_trocos_id_serial_seq', 1, false);


--
-- TOC entry 3801 (class 0 OID 0)
-- Dependencies: 271
-- Name: venda_itens_id_serial_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.venda_itens_id_serial_seq', 244, true);


--
-- TOC entry 3802 (class 0 OID 0)
-- Dependencies: 273
-- Name: venda_pagamentos_id_serial_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.venda_pagamentos_id_serial_seq', 85, true);


--
-- TOC entry 3803 (class 0 OID 0)
-- Dependencies: 279
-- Name: vendas_id_serial_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.vendas_id_serial_seq', 73, true);


--
-- TOC entry 3475 (class 2606 OID 33747)
-- Name: abastecimentos abastecimentos_full_str_un; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.abastecimentos
    ADD CONSTRAINT abastecimentos_full_str_un UNIQUE (full_string);


--
-- TOC entry 3477 (class 2606 OID 33745)
-- Name: abastecimentos abastecimentos_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.abastecimentos
    ADD CONSTRAINT abastecimentos_pkey PRIMARY KEY (id);


--
-- TOC entry 3486 (class 2606 OID 33749)
-- Name: afericoes afericoes_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.afericoes
    ADD CONSTRAINT afericoes_pkey PRIMARY KEY (id);


--
-- TOC entry 3490 (class 2606 OID 33753)
-- Name: bicos_encerrantes bicos_encerrantes_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bicos_encerrantes
    ADD CONSTRAINT bicos_encerrantes_pkey PRIMARY KEY (id);


--
-- TOC entry 3488 (class 2606 OID 33751)
-- Name: bicos bicos_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bicos
    ADD CONSTRAINT bicos_pkey PRIMARY KEY (id);


--
-- TOC entry 3492 (class 2606 OID 33755)
-- Name: caixa caixa_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.caixa
    ADD CONSTRAINT caixa_pkey PRIMARY KEY (id);


--
-- TOC entry 3495 (class 2606 OID 33759)
-- Name: fidelidade_tabelas fidelidade_tabelas_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.fidelidade_tabelas
    ADD CONSTRAINT fidelidade_tabelas_pkey PRIMARY KEY (id);


--
-- TOC entry 3497 (class 2606 OID 33761)
-- Name: formas_pagamento formas_pagamento_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.formas_pagamento
    ADD CONSTRAINT formas_pagamento_pkey PRIMARY KEY (id);


--
-- TOC entry 3501 (class 2606 OID 33763)
-- Name: inutilizacao inutilizacao_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.inutilizacao
    ADD CONSTRAINT inutilizacao_pkey PRIMARY KEY (id);


--
-- TOC entry 3503 (class 2606 OID 33765)
-- Name: logs logs_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.logs
    ADD CONSTRAINT logs_pkey PRIMARY KEY (id);


--
-- TOC entry 3505 (class 2606 OID 41720)
-- Name: parametros parametros_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.parametros
    ADD CONSTRAINT parametros_pk PRIMARY KEY (id);


--
-- TOC entry 3507 (class 2606 OID 33767)
-- Name: parceiro_dependentes parceiro_dependentes_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.parceiro_dependentes
    ADD CONSTRAINT parceiro_dependentes_pkey PRIMARY KEY (id);


--
-- TOC entry 3509 (class 2606 OID 33769)
-- Name: parceiro_tabelas parceiro_tabelas_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.parceiro_tabelas
    ADD CONSTRAINT parceiro_tabelas_pkey PRIMARY KEY (id);


--
-- TOC entry 3511 (class 2606 OID 33771)
-- Name: parceiros parceiros_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.parceiros
    ADD CONSTRAINT parceiros_pkey PRIMARY KEY (id);


--
-- TOC entry 3548 (class 2606 OID 33773)
-- Name: venda_deposito_trocos pk_venda_deposito_trocos; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.venda_deposito_trocos
    ADD CONSTRAINT pk_venda_deposito_trocos PRIMARY KEY (id);


--
-- TOC entry 3513 (class 2606 OID 33775)
-- Name: pre_venda_pagamentos pre_venda_pagamentos_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.pre_venda_pagamentos
    ADD CONSTRAINT pre_venda_pagamentos_pkey PRIMARY KEY (id);


--
-- TOC entry 3517 (class 2606 OID 33777)
-- Name: produtos produtos_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.produtos
    ADD CONSTRAINT produtos_pkey PRIMARY KEY (id);


--
-- TOC entry 3521 (class 2606 OID 33779)
-- Name: produtos_series produtos_series_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.produtos_series
    ADD CONSTRAINT produtos_series_pkey PRIMARY KEY (id);


--
-- TOC entry 3525 (class 2606 OID 33781)
-- Name: produtos_setores produtos_setores_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.produtos_setores
    ADD CONSTRAINT produtos_setores_pkey PRIMARY KEY (id);


--
-- TOC entry 3527 (class 2606 OID 33783)
-- Name: sangria_suprimento sangria_suprimento_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.sangria_suprimento
    ADD CONSTRAINT sangria_suprimento_pkey PRIMARY KEY (id);


--
-- TOC entry 3529 (class 2606 OID 33785)
-- Name: setores setores_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.setores
    ADD CONSTRAINT setores_pkey PRIMARY KEY (id);


--
-- TOC entry 3532 (class 2606 OID 33787)
-- Name: tabela_preco_itens tabela_preco_itens_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tabela_preco_itens
    ADD CONSTRAINT tabela_preco_itens_pkey PRIMARY KEY (id);


--
-- TOC entry 3534 (class 2606 OID 33789)
-- Name: tabela_precos tabelas_precos_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tabela_precos
    ADD CONSTRAINT tabelas_precos_pkey PRIMARY KEY (id);


--
-- TOC entry 3538 (class 2606 OID 33793)
-- Name: turno_postos turno_postos_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.turno_postos
    ADD CONSTRAINT turno_postos_pkey PRIMARY KEY (id);


--
-- TOC entry 3536 (class 2606 OID 33791)
-- Name: turnos turnos_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.turnos
    ADD CONSTRAINT turnos_pkey PRIMARY KEY (id);


--
-- TOC entry 3540 (class 2606 OID 33795)
-- Name: usuario_permissoes usuario_permissoes_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.usuario_permissoes
    ADD CONSTRAINT usuario_permissoes_pkey PRIMARY KEY (id);


--
-- TOC entry 3542 (class 2606 OID 33798)
-- Name: usuarios usuarios_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.usuarios
    ADD CONSTRAINT usuarios_pkey PRIMARY KEY (id);


--
-- TOC entry 3544 (class 2606 OID 33800)
-- Name: venda_cheque_trocos venda_cheque_trocos_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.venda_cheque_trocos
    ADD CONSTRAINT venda_cheque_trocos_pkey PRIMARY KEY (id);


--
-- TOC entry 3546 (class 2606 OID 33802)
-- Name: venda_cheques venda_cheques_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.venda_cheques
    ADD CONSTRAINT venda_cheques_pkey PRIMARY KEY (id);


--
-- TOC entry 3557 (class 2606 OID 33804)
-- Name: venda_itens venda_itens_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.venda_itens
    ADD CONSTRAINT venda_itens_pkey PRIMARY KEY (id);


--
-- TOC entry 3561 (class 2606 OID 33806)
-- Name: venda_pagamentos venda_pagamentos_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.venda_pagamentos
    ADD CONSTRAINT venda_pagamentos_pkey PRIMARY KEY (id);


--
-- TOC entry 3563 (class 2606 OID 33810)
-- Name: vendedores vendedores_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.vendedores
    ADD CONSTRAINT vendedores_pkey PRIMARY KEY (id);


--
-- TOC entry 3478 (class 1259 OID 33811)
-- Name: idx_abas_rfid_cliente; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_abas_rfid_cliente ON public.abastecimentos USING btree (rfid_cliente);


--
-- TOC entry 3479 (class 1259 OID 33812)
-- Name: idx_abas_rfid_frentista; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_abas_rfid_frentista ON public.abastecimentos USING btree (rfid_frentista);


--
-- TOC entry 3480 (class 1259 OID 41688)
-- Name: idx_abast_data; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_abast_data ON public.abastecimentos USING btree (data_hora);


--
-- TOC entry 3481 (class 1259 OID 33814)
-- Name: idx_abast_pdv; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_abast_pdv ON public.abastecimentos USING btree (pdv);


--
-- TOC entry 3482 (class 1259 OID 33815)
-- Name: idx_abast_retorno; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_abast_retorno ON public.abastecimentos USING btree (retorno);


--
-- TOC entry 3483 (class 1259 OID 33816)
-- Name: idx_abast_status; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_abast_status ON public.abastecimentos USING btree (status);


--
-- TOC entry 3493 (class 1259 OID 33817)
-- Name: idx_caixa_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_caixa_id ON public.caixa USING btree (id);


--
-- TOC entry 3484 (class 1259 OID 41689)
-- Name: idx_duplicidades; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_duplicidades ON public.abastecimentos USING btree (retorno, quantidade, data_hora, encerrante_inicial, encerrante_final);


--
-- TOC entry 3498 (class 1259 OID 33819)
-- Name: idx_grades_itens_codigo; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_grades_itens_codigo ON public.grades_itens USING btree (codigo);


--
-- TOC entry 3499 (class 1259 OID 33820)
-- Name: idx_grades_itens_grade_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_grades_itens_grade_id ON public.grades_itens USING btree (grade_id);


--
-- TOC entry 3518 (class 1259 OID 33821)
-- Name: idx_produtos_codigos_codigo; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_produtos_codigos_codigo ON public.produtos_codigos USING btree (codigo);


--
-- TOC entry 3519 (class 1259 OID 33822)
-- Name: idx_produtos_codigos_produto; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_produtos_codigos_produto ON public.produtos_codigos USING btree (produto_id);


--
-- TOC entry 3514 (class 1259 OID 33823)
-- Name: idx_produtos_descricao; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_produtos_descricao ON public.produtos USING btree (descricao);


--
-- TOC entry 3515 (class 1259 OID 33824)
-- Name: idx_produtos_gtin; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_produtos_gtin ON public.produtos USING btree (gtin_comercial);


--
-- TOC entry 3522 (class 1259 OID 33825)
-- Name: idx_produtos_setores_produto; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_produtos_setores_produto ON public.produtos_setores USING btree (produto_id);


--
-- TOC entry 3523 (class 1259 OID 33826)
-- Name: idx_produtos_setores_setor; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_produtos_setores_setor ON public.produtos_setores USING btree (setor_id);


--
-- TOC entry 3530 (class 1259 OID 33827)
-- Name: idx_tabela_preco_itens_produto; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_tabela_preco_itens_produto ON public.tabela_preco_itens USING btree (produto_id);


--
-- TOC entry 3549 (class 1259 OID 33829)
-- Name: idx_venda_itens_abast; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_venda_itens_abast ON public.venda_itens USING btree (abastecimento_id);


--
-- TOC entry 3550 (class 1259 OID 33830)
-- Name: idx_venda_itens_bico; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_venda_itens_bico ON public.venda_itens USING btree (bico_id);


--
-- TOC entry 3551 (class 1259 OID 33831)
-- Name: idx_venda_itens_produto; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_venda_itens_produto ON public.venda_itens USING btree (produto_id);


--
-- TOC entry 3552 (class 1259 OID 33832)
-- Name: idx_venda_itens_setor; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_venda_itens_setor ON public.venda_itens USING btree (setor_id);


--
-- TOC entry 3553 (class 1259 OID 33833)
-- Name: idx_venda_itens_status; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_venda_itens_status ON public.venda_itens USING btree (status);


--
-- TOC entry 3554 (class 1259 OID 33834)
-- Name: idx_venda_itens_tabela_preco; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_venda_itens_tabela_preco ON public.venda_itens USING btree (tabela_preco_id);


--
-- TOC entry 3555 (class 1259 OID 33835)
-- Name: idx_venda_itens_venda; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_venda_itens_venda ON public.venda_itens USING btree (venda_id);


--
-- TOC entry 3558 (class 1259 OID 33836)
-- Name: idx_venda_pagamento_forma_pagamento; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_venda_pagamento_forma_pagamento ON public.venda_pagamentos USING btree (forma_pagamento_id);


--
-- TOC entry 3559 (class 1259 OID 33837)
-- Name: idx_venda_pagamentos_venda; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_venda_pagamentos_venda ON public.venda_pagamentos USING btree (venda_id);


-- Completed on 2026-08-04 23:12:35

--
-- PostgreSQL database dump complete
--

