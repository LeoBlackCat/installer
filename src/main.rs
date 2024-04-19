extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
extern crate winapi;


use nwd::NwgUi;
use crate::nwg::NativeUi;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use directories::UserDirs;
use std::process::Command;


#[derive(Default, NwgUi)]
pub struct EmbedApp {
    #[nwg_control(size: (800, 600), center: true, flags: "POPUP|VISIBLE", title: "Installer")]
    #[nwg_events( OnWindowClose: [EmbedApp::close], OnInit: [EmbedApp::init]  )]
    window: nwg::Window,

    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_resource(source_embed: Some(&data.embed), source_embed_str: Some("UPDATE"))]
    update: nwg::Bitmap,

    #[nwg_resource(source_embed: Some(&data.embed), source_embed_str: Some("CLOSE"))]
    close: nwg::Bitmap,

    #[nwg_control(position: (272, 130), size: (256, 240), bitmap: Some(&data.update))]
    embed_bitmap: nwg::ImageFrame,

    #[nwg_control(position: (280, 450), size: (240, 40), text: "Install", flags: "VISIBLE|OWNERDRAW")]
    #[nwg_events( OnButtonClick: [EmbedApp::download_and_install] )]
    install_button: nwg::Button,

    #[nwg_control(position: (768, 0), size: (24, 24), text: "1", flags: "VISIBLE|BITMAP|OWNERDRAW", bitmap: Some(&data.close))]
    #[nwg_events( OnButtonClick: [EmbedApp::close] )]
    close_button: nwg::Button,

    #[nwg_control(position: (0, 0), size: (200, 24), text: "Hello", flags: "VISIBLE|PUSHLIKE")]
    fuck_button: nwg::Button,
}

impl EmbedApp {
    fn init(&self) {}

    fn download_and_install(&self) {
        let url = "https://github.com/msys2/msys2-installer/releases/download/2024-01-13/msys2-x86_64-20240113.exe";
        let download_path = Self::get_download_path("installer_from_rust.exe")
            .expect("Failed to get download path");
        let download_path_clone = download_path.clone(); // Clone the path for use in the async block


        tokio::spawn(async move {
            let response = reqwest::get(url).await.expect("Failed to download file");
            if response.status().is_success() {
                let body = response.bytes().await.expect("Failed to read bytes");
                let mut dest = File::create(download_path).expect("Failed to create file");
                dest.write_all(&body).expect("Failed to write bytes");
                println!("Download completed successfully.");
            } else {
                println!("Failed to download file: {}", response.status());
            }
        });

        // Execute the installer after a delay to ensure the download completes
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(5)); // Adjust timing as needed
            if let Err(e) = Command::new(download_path_clone).spawn() {
                eprintln!("Failed to execute installer: {}", e);
            }
        });
        
    }

    fn get_download_path(filename: &str) -> Option<PathBuf> {
        if let Some(user_dirs) = UserDirs::new() {
            user_dirs.download_dir().map(|dir| dir.join(filename))
        } else {
            None
        }
    }

    fn close(&self) {
        nwg::stop_thread_dispatch();
    }
}


#[tokio::main]
async fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = EmbedApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}