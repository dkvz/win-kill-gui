#![windows_subsystem = "windows"]

use nwg::{self, NativeUi};
mod main_window;
mod commands;
use main_window::MainWindow;

fn main() {
  nwg::init().expect("Failed to init Native Windows GUI");
  nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

  let _app = MainWindow::build_ui(Default::default())
    .expect("Failed to build UI");

  nwg::dispatch_thread_events();
}
