use glib::clone;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};
fn main() {
  let app = Application::new(Some("org.gtk.example"), Default::default());

  app.connect_activate(build_ui);

  app.run();
}

fn build_ui(app: &Application) {
  let window = ApplicationWindow::new(app);

  window.set_title(Some("My GTK APP"));

  let button = build_button();

  button.connect_clicked(clone!(@weak button => move |_|{
    button.set_label("Hello world");
  }));

  window.set_child(Some(&button));
  window.present();
}

fn build_button() -> Button {
  let button = Button::builder()
    .label("Press me!")
    .margin_top(12)
    .margin_bottom(12)
    .margin_start(12)
    .margin_end(12)
    .build();
  button
}
