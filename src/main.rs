use std::clone::Clone;
use std::default::Default;
use std::option::Option;
use std::option::Option::Some;
use std::result::Result::Ok;

use gdk::{EventButton, EventType};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    ApplicationWindow, Builder, ButtonExt, GtkWindowExt, Inhibit, ListBox, ListBoxExt, PopoverExt,
    PopoverMenu, WidgetExt,
};

const GLADE_SRC: &str = include_str!("grid.glade");

#[derive(Clone)]
struct AppCtx {
    window: gtk::ApplicationWindow,
    meta_context_menu: PopoverMenu,
    search_buttons: SearchButtons,
    list_box: ListBox,
}

#[derive(Clone)]
struct SearchButtons {
    pub button_one: gtk::Button,
    pub button_two: gtk::Button,
    pub button_three: gtk::Button,
}

impl SearchButtons {
    pub fn get_button_one(&self) -> gtk::Button {
        self.button_one.clone()
    }
    pub fn get_button_two(&self) -> gtk::Button {
        self.button_two.clone()
    }

    pub fn get_button_three(&self) -> gtk::Button {
        self.button_three.clone()
    }
}

impl AppCtx {
    pub fn new(application: &gtk::Application) -> Self {
        let builder = Builder::new_from_string(GLADE_SRC);
        let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
        window.set_application(Some(application));

        let meta_context_menu: PopoverMenu = builder
            .get_object("meta_context_menu")
            .expect("Couldn't get meta_context_menu");

        let meta_button_one: gtk::Button = builder
            .get_object("meta_button_one")
            .expect("Couldn't get meta_button_one");

        let meta_button_two: gtk::Button = builder
            .get_object("meta_button_two")
            .expect("Couldn't get meta_button_two");

        let meta_button_three: gtk::Button = builder
            .get_object("meta_button_three")
            .expect("Couldn't get meta_button_three");

        let list_box: gtk::ListBox = builder
            .get_object("list_box")
            .expect("Couldn't get list_box");

        let search_buttons = SearchButtons {
            button_one: meta_button_one,
            button_two: meta_button_two,
            button_three: meta_button_three,
        };

        AppCtx {
            window,
            meta_context_menu,
            search_buttons,
            list_box,
        }
    }

    pub fn open_meta_context_menu(&self) {
        self.meta_context_menu.popup()
    }

    pub fn close_meta_context_menu(&self) {
        self.meta_context_menu.popdown()
    }

    pub fn setup_search_buttons(&self) {
        self.clone().setup_button_one_clicked();
        self.clone().setup_button_two_clicked();
        self.clone().setup_button_three_clicked();
    }

    pub fn set_meta_context_menu_relative_to<P: IsA<gtk::Widget>>(&self, widget: Option<&P>) {
        self.meta_context_menu.set_relative_to(widget)
    }

    pub fn get_window(&self) -> gtk::ApplicationWindow {
        self.window.clone()
    }

    fn connect_mouse_events(self, list_box: ListBox, event_button: EventButton) -> Inhibit {
        if let EventType::ButtonPress = event_button.get_event_type() {
            if let Some(row) = ListBoxExt::get_selected_row(&list_box) {
                self.set_meta_context_menu_relative_to(Some(&row));
                self.setup_search_buttons();

                match event_button.get_button() {
                    // Left click
                    1 => {
                        // debug!("Detected left click on {:?}: {:?}", search_type, text);
                        Inhibit(false)
                    }
                    // Right click
                    3 => {
                        // debug!("Detected right click on {:?}: {:?}", search_type, text);
                        self.open_meta_context_menu();
                        Inhibit(true)
                    }
                    // Ignore other button presses
                    _ => Inhibit(true),
                }
            } else {
                Inhibit(false)
            }
        } else {
            Inhibit(false)
        }
    }

    pub fn setup_button_one_clicked(self) {
        let ctx = self.clone();
        ctx.search_buttons
            .get_button_one()
            .connect_clicked(move |_s| {
                ctx.close_meta_context_menu();
                // do thing
                dbg!("one clicked");
            });
    }

    pub fn setup_button_two_clicked(self) {
        let ctx = self.clone();
        ctx.search_buttons
            .get_button_two()
            .connect_clicked(move |_s| {
                ctx.close_meta_context_menu();
                // do thing
                dbg!("two clicked");
            });
    }

    pub fn setup_button_three_clicked(self) {
        let ctx = self.clone();
        ctx.search_buttons
            .get_button_three()
            .connect_clicked(move |_s| {
                ctx.close_meta_context_menu();
                // do thing
                dbg!("three clicked");
            });
    }

    fn setup_meta_list(self) {
        self.clone().list_box.connect_button_press_event(
            move |list_box: &ListBox, event_button: &EventButton| {
                self.clone()
                    .connect_mouse_events(list_box.clone(), event_button.clone())
            },
        );
    }
}

fn build_ui(application: &gtk::Application) -> std::io::Result<()> {
    let app_ctx = AppCtx::new(application);

    let window = app_ctx.get_window();
    app_ctx.setup_meta_list();

    window.show_all();
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let application = gtk::Application::new(Some("org.evanjs.popover-help"), Default::default())?;

    application.connect_activate(|app| {
        build_ui(app).unwrap();
    });

    application.run(&[]);

    Ok(())
}
