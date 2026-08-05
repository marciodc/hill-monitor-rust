#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
#![recursion_limit = "256"]

mod backend_url;
mod config;
mod scheduler;

use single_instance::SingleInstance;
use std::env;
use tracing::{error, info};
use tracing_subscriber::prelude::*;

fn is_enabled_flag(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_uppercase().as_str(),
        "T" | "TRUE" | "1" | "YES" | "Y" | "SIM" | "S"
    )
}

#[cfg(target_os = "windows")]
fn maybe_enable_debug_console(enabled: bool) {
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::Storage::FileSystem::{
        CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_READ,
        FILE_SHARE_WRITE, OPEN_EXISTING,
    };
    use windows_sys::Win32::System::Console::AllocConsole;
    use windows_sys::Win32::System::Console::{STD_ERROR_HANDLE, STD_OUTPUT_HANDLE, SetStdHandle};

    fn to_wstring(value: &str) -> Vec<u16> {
        value.encode_utf16().chain(Some(0)).collect()
    }

    if enabled {
        unsafe {
            let _ = AllocConsole();

            let conout = to_wstring("CONOUT$");
            let handle = CreateFileW(
                conout.as_ptr(),
                FILE_GENERIC_READ | FILE_GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                std::ptr::null(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                std::ptr::null_mut(),
            );

            if handle != INVALID_HANDLE_VALUE {
                let _ = SetStdHandle(STD_OUTPUT_HANDLE, handle as HANDLE);
                let _ = SetStdHandle(STD_ERROR_HANDLE, handle as HANDLE);
                let _ = CloseHandle(handle);
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn spawn_windows_tray() {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr;
    use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM};
    use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows_sys::Win32::UI::Shell::{
        Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NOTIFYICONDATAW,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu,
        DispatchMessageW, GetCursorPos, GetMessageW, LoadImageW, PostQuitMessage, RegisterClassW,
        SetForegroundWindow, TrackPopupMenu, TranslateMessage, CW_USEDEFAULT, HICON, HMENU,
        IMAGE_ICON, LR_DEFAULTSIZE, LR_LOADFROMFILE, MF_STRING, MSG, TPM_BOTTOMALIGN,
        TPM_LEFTALIGN, TPM_LEFTBUTTON, WINDOW_EX_STYLE, WM_COMMAND, WM_DESTROY, WM_RBUTTONUP,
        WM_USER, WNDCLASSW, WS_OVERLAPPEDWINDOW,
    };

    const WM_TRAYICON: u32 = WM_USER + 1;
    const ID_TRAY_EXIT: usize = 1001;

    unsafe extern "system" fn window_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_COMMAND => {
                if wparam == ID_TRAY_EXIT {
                    info!("Sair selecionado via menu da bandeja. Finalizando.");
                    unsafe { PostQuitMessage(0) };
                    std::process::exit(0);
                }
                0
            }
            WM_TRAYICON => {
                if lparam as u32 == WM_RBUTTONUP {
                    let menu: HMENU = unsafe { CreatePopupMenu() };
                    if !menu.is_null() {
                        let exit_label = to_wstring(OsStr::new("Sair"));
                        unsafe {
                            AppendMenuW(menu, MF_STRING, ID_TRAY_EXIT, exit_label.as_ptr());
                            SetForegroundWindow(hwnd);
                        }

                        let mut point = POINT { x: 0, y: 0 };
                        if unsafe { GetCursorPos(&mut point) } != 0 {
                            unsafe {
                                TrackPopupMenu(
                                    menu,
                                    TPM_LEFTALIGN | TPM_BOTTOMALIGN | TPM_LEFTBUTTON,
                                    point.x,
                                    point.y,
                                    0,
                                    hwnd,
                                    ptr::null(),
                                );
                            }
                        }

                        unsafe { DestroyMenu(menu) };
                    }
                    0
                } else {
                    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
                }
            }
            WM_DESTROY => {
                unsafe { PostQuitMessage(0) };
                0
            }
            _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
        }
    }

    fn to_wstring(value: &OsStr) -> Vec<u16> {
        value.encode_wide().chain(Some(0)).collect()
    }

    std::thread::spawn(move || unsafe {
        let icon_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("Resource")
            .join("app.ico");
        if !icon_path.exists() {
            error!("Ícone da bandeja não encontrado em {:?}", icon_path);
            return;
        }

        let class_name = to_wstring(OsStr::new("hill_monitor_tray_window"));
        let hinstance = GetModuleHandleW(ptr::null());
        if hinstance.is_null() {
            error!("Falha ao obter handle do módulo para o tray do Windows.");
            return;
        }

        let wnd = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: hinstance,
            lpszClassName: class_name.as_ptr(),
            ..std::mem::zeroed()
        };

        if RegisterClassW(&wnd) == 0 {
            error!("Falha ao registrar classe da janela do tray do Windows.");
            return;
        }

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class_name.as_ptr(),
            class_name.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            0,
            CW_USEDEFAULT,
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            hinstance,
            ptr::null(),
        );

        if hwnd.is_null() {
            error!("Falha ao criar janela oculta do tray do Windows.");
            return;
        }

        let icon_w = to_wstring(icon_path.as_os_str());
        let icon = LoadImageW(
            ptr::null_mut(),
            icon_w.as_ptr(),
            IMAGE_ICON,
            0,
            0,
            LR_LOADFROMFILE | LR_DEFAULTSIZE,
        ) as HICON;

        if icon.is_null() {
            error!("Falha ao carregar ícone do disco para o tray do Windows.");
            return;
        }

        let mut nid = std::mem::zeroed::<NOTIFYICONDATAW>();
        nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = hwnd;
        nid.uID = 1;
        nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
        nid.uCallbackMessage = WM_TRAYICON;
        nid.hIcon = icon;

        let tip = to_wstring(OsStr::new("Hill Monitor"));
        let copy_len = tip.len().min(nid.szTip.len());
        nid.szTip[..copy_len].copy_from_slice(&tip[..copy_len]);

        if Shell_NotifyIconW(NIM_ADD, &nid) == 0 {
            error!("Falha ao adicionar ícone na bandeja do Windows.");
            return;
        }

        let mut msg = std::mem::zeroed::<MSG>();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    });
}

fn normalize_log_level(level: &str) -> &'static str {
    match level.trim().to_ascii_uppercase().as_str() {
        "TRACE" => "trace",
        "DEBUG" => "debug",
        "WARN" | "WARNING" => "warn",
        "ERROR" => "error",
        "OFF" => "off",
        _ => "info",
    }
}

fn setup_logging(
    log_dir: &std::path::Path,
    console_level: &str,
) -> Option<tracing_appender::non_blocking::WorkerGuard> {

    // Daily rotating file appender (e.g. monitor.log.2026-08-04)
    // Note: tracing_appender rolling appends the date suffix automatically
    let file_appender = tracing_appender::rolling::daily(log_dir, "monitor.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_filter(tracing_subscriber::EnvFilter::new("warn"));

    let console_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(tracing_subscriber::EnvFilter::new(normalize_log_level(console_level)));

    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .init();

    Some(guard)
}

#[tokio::main]
async fn main() {
    // 1. Single Instance Check
    let instance = SingleInstance::new("br.com.hilltecnologia.monitor")
        .expect("Falha ao inicializar verificação de instância única.");
    if !instance.is_single() {
        eprintln!("Já existe uma instância do aplicativo em execução.");
        std::process::exit(1);
    }

    // 2. Resolve executable paths and load configuration (monitor.ini)
    let exe_dir = match env::current_exe().ok().and_then(|p| p.parent().map(|p| p.to_path_buf())) {
        Some(dir) => dir,
        None => {
            eprintln!("Não foi possível determinar o diretório do executável.");
            std::process::exit(1);
        }
    };

    let ini_path = exe_dir.join("monitor.ini");

    // Create a dummy ini file if it doesn't exist for test purposes
    if !ini_path.exists() {
        let default_ini_content = "\
DB_IP=localhost
DB_PORTA=5432
LOG_SQL=F
LOG=INFO
LOG_TERMINAL=INFO
EXIBIR_TERMINAL=F
FABRICANTE=companytec
";
        if let Err(e) = std::fs::write(&ini_path, default_ini_content) {
            error!("Falha ao criar monitor.ini padrão: {:?}", e);
        }
    }

    let ini = match config::IniFile::read_from_file(&ini_path) {
        Ok(ini) => ini,
        Err(e) => {
            eprintln!("Erro ao ler o arquivo INI: {:?}", e);
            std::process::exit(1);
        }
    };

    #[cfg(target_os = "windows")]
    maybe_enable_debug_console(is_enabled_flag(&ini.exibir_terminal));

    // 3. Setup Logging
    let log_dir = exe_dir.join("Log");
    let console_level = if is_enabled_flag(&ini.exibir_terminal) {
        ini.log_terminal.as_str()
    } else {
        "off"
    };
    let _guard = setup_logging(&log_dir, console_level);

    info!("Iniciando hill-monitor...");
    info!("Lendo arquivo de configuração de: {:?}", ini_path);
    info!("Configuração carregada com sucesso.");
    info!("DB IP: {}", ini.db_ip);
    info!("DB Porta: {}", ini.db_porta);
    info!("Log arquivo: WARN");
    info!("Log terminal: {}", ini.log_terminal);
    info!("Exibir terminal: {}", ini.exibir_terminal);
    info!("SQL Log: {}", ini.log_sql);
    info!("Fabricante: {}", ini.fabricante);

    // 4. Connect to Database
    let log_sql = is_enabled_flag(&ini.log_sql);

    let db_conn = match hill_common::db::establish_connection(&ini.db_ip, &ini.db_porta, log_sql).await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Não foi possível estabelecer conexão com o banco de dados: {:?}", e);
            return;
        }
    };

    // 5. Initialize Concentrador (Serial Port & Scheduler)
    let config_helper = hill_common::config_helper::ConfigHelper::new(db_conn.clone());
    let serial_port = config_helper
        .get_parametro("CONCENTRADOR_Porta", None)
        .await
        .unwrap_or(None)
        .unwrap_or_else(|| "COM1".to_string());

    info!("Inicializando comunicação com o concentrador serial na porta: {}", serial_port);
    let com = hill_concentrador::com::ConcentradorCom::new(&serial_port);
    let op = hill_concentrador::operation::ConcentradorOperacao::new(com, &ini.fabricante);
    let concentrador_scheduler = hill_concentrador::scheduler::ConcentradorScheduler::new(op, db_conn.clone());
    concentrador_scheduler.start();

    // 6. Start Monitor Schedulers (Atualizacao, Contingencia, Envio)
    let monitor_schedulers = scheduler::MonitorSchedulers::new(db_conn.clone());
    monitor_schedulers.start();

    // 7. Start HTTP Web Server using Axum on the local machine default port
    let app = hill_pdv::web::create_router(db_conn);
    let bind_addr = "0.0.0.0:5000";

    info!("Servidor Web sendo iniciado em: {}", bind_addr);

    let listener = match tokio::net::TcpListener::bind(bind_addr).await {
        Ok(l) => l,
        Err(e) => {
            error!(
                "Erro ao vincular listener TCP para o Servidor Web em {}: {:?}",
                bind_addr, e
            );
            concentrador_scheduler.stop();
            monitor_schedulers.stop();
            return;
        }
    };

    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            error!("Erro na execução do Servidor Web Axum: {:?}", e);
        }
    });

    // Inicialização do ícone da bandeja (System Tray)
    #[cfg(target_os = "linux")]
    {
        struct MyTray {
            icon_data: Vec<u8>,
            width: i32,
            height: i32,
        }

        impl ksni::Tray for MyTray {
            fn id(&self) -> String {
                "hill-monitor".to_string()
            }

            fn title(&self) -> String {
                "Hill Monitor".to_string()
            }

            fn icon_pixmap(&self) -> Vec<ksni::Icon> {
                vec![ksni::Icon {
                    width: self.width,
                    height: self.height,
                    data: self.icon_data.clone(),
                }]
            }

            fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
                use ksni::menu::StandardItem;
                vec![
                    StandardItem {
                        label: "Sair".to_string(),
                        activate: Box::new(|_| {
                            info!("Sair selecionado via menu da bandeja. Finalizando.");
                            std::process::exit(0);
                        }),
                        ..Default::default()
                    }
                    .into()
                ]
            }
        }

        let png_bytes = include_bytes!("../Resource/app.png");
        match image::load_from_memory_with_format(png_bytes, image::ImageFormat::Png) {
            Ok(img) => {
                let rgba = img.into_rgba8();
                let (width, height) = rgba.dimensions();
                
                // Convert RGBA to ARGB (required by ksni/dbus)
                let mut argb = Vec::with_capacity(rgba.len());
                for chunk in rgba.chunks_exact(4) {
                    let r = chunk[0];
                    let g = chunk[1];
                    let b = chunk[2];
                    let a = chunk[3];
                    argb.push(a);
                    argb.push(r);
                    argb.push(g);
                    argb.push(b);
                }

                let tray = MyTray {
                    icon_data: argb,
                    width: width as i32,
                    height: height as i32,
                };
                let svc = ksni::TrayService::new(tray);
                svc.spawn();
            }
            Err(e) => {
                error!("Erro ao decodificar PNG embutido para a bandeja: {:?}", e);
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        spawn_windows_tray();
    }

    // Mantém a thread principal ativa indefinidamente
    let (_tx, rx) = tokio::sync::oneshot::channel::<()>();
    let _ = rx.await;

    concentrador_scheduler.stop();
    monitor_schedulers.stop();
    info!("Aplicação finalizada.");
}
