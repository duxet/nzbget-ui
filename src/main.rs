extern crate gtk;
extern crate glib;
extern crate hyper;
extern crate humansize;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::cell::RefCell;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use gtk::prelude::*;
use gtk::{AboutDialog, Builder, CellRendererText, CellRendererProgress, Label, ListStore,
    Statusbar, Type, TreeView, TreeViewColumn, Menu, MenuItem, Widget, Window};
use humansize::{FileSize, file_size_opts as options};

mod client;
#[macro_use] mod macros;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Group {
    NZBNicename: String,
    Status: String,
    FileSizeLo: u32,
    FileSizeHi: u32,
    DownloadedSizeLo: u32,
    DownloadedSizeHi: u32,
    RemainingSizeLo: u32,
    RemainingSizeHi: u32
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Status {
    DownloadRate: i32,
    DownloadPaused: bool
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let builder = Builder::new_from_string(include_str!("interface.glade"));

    let window: Window = builder.get_object("main_window").unwrap();
    let about_item :MenuItem = builder.get_object("about_item").unwrap();
    let about_dialog :AboutDialog = builder.get_object("about_dialog").unwrap();
    let status_bar :Statusbar = builder.get_object("status_bar").unwrap();
    let files_tree: TreeView = builder.get_object("files_tree").unwrap();
    let current_speed :Label = builder.get_object("current_speed").unwrap();

    about_item.connect_activate(move |_| {
        about_dialog.run();
        about_dialog.hide();
    });

    let context_id = status_bar.get_context_id("");
    status_bar.push(context_id, "nzbget-ui");

    let files_store = ListStore::new(&[Type::String, Type::String, Type::String, Type::F64]);
    add_text_column!(files_tree, "Title", 0);
    add_text_column!(files_tree, "Status", 1);
    add_text_column!(files_tree, "Size", 2);
    add_progress_column!(files_tree, "Progress", 3);

    files_tree.set_model(Some(&files_store));

    let widget = files_tree.upcast::<Widget>();

    widget.connect_button_release_event(|_, event| {
        if event.get_button() != 3 {
            return Inhibit(false)
        }

        let popup_menu = Menu::new();

        let item = MenuItem::new_with_label("resume");
        popup_menu.append(&item);

        let item = MenuItem::new_with_label("pause");
        popup_menu.append(&item);

        popup_menu.append(&gtk::SeparatorMenuItem::new());

        let item = MenuItem::new_with_label("delete");
        popup_menu.append(&item);

        popup_menu.show_all();
        popup_menu.popup_easy(event.get_button(), event.get_time());

        Inhibit(false)
    });

    window.show_all();

    let (tx, rx) = channel();
    GLOBAL.with(move |global| {
        *global.borrow_mut() = Some((current_speed, rx))
    });

    thread::spawn(move || {
        loop {
            let status = load_status();

            tx.send(status).unwrap();

            thread::sleep(std::time::Duration::from_millis(1000));
            glib::idle_add(receive);
        }
    });

    let groups = load_groups();

    for group in groups {
        let file_size = format!("{}{}", group.FileSizeHi, group.FileSizeLo).parse::<i64>().unwrap();
        let downloaded_size = format!("{}{}", group.DownloadedSizeHi, group.DownloadedSizeLo).parse::<i64>().unwrap();

        let progress = downloaded_size as f64 / file_size as f64 * 100.0;
        let human_size = file_size.file_size(options::CONVENTIONAL).unwrap();

        files_store.insert_with_values(None, &[0, 1, 2, 3], &[&group.NZBNicename, &group.Status, &human_size, &progress]);
    }

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

fn load_groups() -> Vec<Group> {
    let client = client::Client::new("http://localhost:6789");
    let response = client.call_method("listgroups");

    serde_json::from_value(response.result).unwrap()
}

fn load_status() -> Status {
    let client = client::Client::new("http://localhost:6789");
    let response = client.call_method("status");

    serde_json::from_value(response.result).unwrap()
}

fn receive() -> glib::Continue {
    GLOBAL.with(|global| {
        if let Some((ref label, ref rx)) = *global.borrow() {
            if let Ok(status) = rx.try_recv() {
                let mut human_speed = status.DownloadRate.file_size(options::CONVENTIONAL).unwrap();
                human_speed.push_str("/s");
                label.set_label(&human_speed);
            }
        }
    });
    glib::Continue(false)
}

thread_local!(
    static GLOBAL: RefCell<Option<(Label, Receiver<Status>)>> = RefCell::new(None)
);
