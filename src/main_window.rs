use nwd::NwgUi;
use nwg::{self, EventData};
use super::commands;

pub const APP_TITLE: &'static str = "Kill that PID";

#[derive(Default, NwgUi)]
pub struct MainWindow {
  #[nwg_control(size: (350, 400), position: (1200, 300), title: APP_TITLE, flags: "WINDOW|VISIBLE")]
  #[nwg_events(
    OnWindowClose: [MainWindow::quit],
    OnKeyPress: [MainWindow::key_press(SELF, EVT_DATA)]
  )]
  
  window: nwg::Window,

  #[nwg_layout(parent: window, spacing: 1)]
  grid: nwg::GridLayout,

  #[nwg_control(text: "Enter PID here:")]
  #[nwg_layout_item(layout: grid, row: 0, col: 0)]
  pid_edit_label: nwg::Label,

  #[nwg_control(focus: true)]
  #[nwg_layout_item(layout: grid, row: 1, col: 0)]
  pid_edit: nwg::TextInput,

  #[nwg_control(text: "Kill it!")]
  #[nwg_layout_item(layout: grid, row: 2, col: 0, row_span: 3)]
  #[nwg_events( OnButtonClick: [MainWindow::kill_button_click] )]
  kill_button: nwg::Button
}

impl MainWindow {

  fn kill_button_click(&self) {
    self.kill_pid();
  }

  fn key_press(&self, data: &EventData) {
    match data {
      EventData::OnKey(nwg::keys::_K) => {
        self.kill_pid();
      },
      _ => ()
    }
  }

  fn kill_pid(&self) {
    if let Ok(pid) = self.pid_edit.text().parse::<usize>() {
      match commands::cli_kill(pid) {
        Ok(output) => nwg::modal_info_message(&self.window, APP_TITLE, &output),
        Err(err) => nwg::modal_error_message(&self.window, APP_TITLE, &err.to_string())
      };
      return;
    }
    nwg::modal_error_message(&self.window, APP_TITLE, "Not a valid PID.");
  }
  
  fn quit(&self) {
    // From what I understand, this closes the app:
    nwg::stop_thread_dispatch();
  }

}