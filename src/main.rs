extern crate gtk;
extern crate hyper;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use gtk::prelude::*;
use gtk::{AboutDialog, Builder, CellRendererText, CellRendererProgress, ListStore,
    Statusbar, Type, TreeView, TreeViewColumn, Menu, MenuItem, Widget, Window};

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

    about_item.connect_activate(move |_| {
        about_dialog.run();
        about_dialog.hide();
    });

    let context_id = status_bar.get_context_id("");
    status_bar.push(context_id, "nzbget-ui");

    let files_store = ListStore::new(&[Type::String, Type::String, Type::F32]);
    add_text_column!(files_tree, "Title", 0);
    add_text_column!(files_tree, "Status", 1);
    add_progress_column!(files_tree, "Progress", 2);

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

    let groups = load_groups();

    for group in groups {
        let file_size = format!("{}{}", group.FileSizeHi, group.FileSizeLo).parse::<f32>().unwrap();
        let downloaded_size = format!("{}{}", group.DownloadedSizeHi, group.DownloadedSizeLo).parse::<f32>().unwrap();

        let progress = downloaded_size / file_size * 100.0;

        files_store.insert_with_values(None, &[0, 1, 2], &[&group.NZBNicename, &group.Status, &progress]);
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
