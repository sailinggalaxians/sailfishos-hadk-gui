use eframe::egui;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Sailfish OS HADK",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    android_root: String,
    vendor: String,
    device: String,
    port_arch: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            android_root: "$HOME/hadk".to_owned(),
            vendor: "".to_owned(),
            device: "".to_owned(),
            port_arch: "".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Try to fetch the username for greeting message from the environment variable "USER"
            if let Ok(username) = std::env::var("USER") {
                ui.heading(format!("Welcome, {}!", username));
            } else {
                // Standard greeting if the username cannot be fetched
                ui.heading("Welcome to the GUI for the Sailfish OS HADK!");
            }

            ui.horizontal(|ui| {
                ui.label("Manufacturer:");
                ui.text_edit_singleline(&mut self.vendor);
            });

            ui.horizontal(|ui| {
                ui.label("Device codename:");
                ui.text_edit_singleline(&mut self.device);
            });

            ui.horizontal(|ui| {
                ui.label("CPU architecture:");
                ui.text_edit_singleline(&mut self.port_arch);
            });

            if ui.button("Save configuration").clicked() {
                save_hadk_env(
                    &self.android_root,
                    &self.vendor,
                    &self.device,
                    &self.port_arch,
                );
            }
        });
    }
}

fn save_hadk_env(android_root: &str, vendor: &str, device: &str, port_arch: &str) {
    let home_dir = std::env::var("HOME").expect("Could not find the $HOME directory");
    let path = PathBuf::from(format!("{}/.hadk.env", home_dir));
    let mut file = File::create(path).expect("Could not create .hadk.env");

    let content = format!(
        r#"export ANDROID_ROOT="{android_root}"
export VENDOR="{vendor}"
export DEVICE="{device}"
export PORT_ARCH="{port_arch}"
alias mb2='mb2 --output-dir "{android_root}/droid-local-repo/{device}"'
"#,
        android_root = android_root,
        vendor = vendor,
        device = device,
        port_arch = port_arch
    );

    file.write_all(content.as_bytes())
        .expect("Could not write to .hadk.env");
}
