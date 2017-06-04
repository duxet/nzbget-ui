extern crate gtk;
extern crate hyper;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use gtk::prelude::*;
use gtk::{Builder, CellRendererText, ListStore, Statusbar, Type, TreeView, TreeViewColumn, Window};

mod client;
#[macro_use] mod macros;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Group {
    NZBNicename: String,
    Status: String
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let builder = Builder::new_from_string(include_str!("interface.glade"));
    let window: Window = builder.get_object("main_window").unwrap();
    let status_bar :Statusbar = builder.get_object("status_bar").unwrap();
    let files_tree: TreeView = builder.get_object("files_tree").unwrap();

    let context_id = status_bar.get_context_id("");
    status_bar.push(context_id, "nzbget-ui");

    let files_store = ListStore::new(&[Type::String, Type::String]);
    add_column!(files_tree, "Title", 0);
    add_column!(files_tree, "Status", 1);

    files_tree.set_model(Some(&files_store));

    window.show_all();

    let groups = load_groups();

    for group in groups {
        files_store.insert_with_values(None, &[0, 1], &[&group.NZBNicename, &group.Status]);
    }

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

fn load_groups() -> Vec<Group> {
    let response = client::call_method("listgroups");

    serde_json::from_value(response.result).unwrap()
}
