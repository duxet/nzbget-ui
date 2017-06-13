extern crate gtk;
extern crate glib;
extern crate hyper;
extern crate humansize;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

#[macro_use] mod gui;
mod json_rpc;
mod nzbget;

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver};
use std::sync::Mutex;
use std::thread;
use gui::Interface;
use gtk::prelude::*;
use gtk::{AboutDialog, Builder, Label, ListStore, Statusbar, ToolButton,
    Type, TreeIter, Menu, MenuItem, Widget, Window};
use humansize::{FileSize, file_size_opts as options};
use nzbget::{Client, Group, Status};

fn main() {
    gtk::init().unwrap();

    let builder = Builder::new_from_string(include_str!("interface.glade"));

    let window: Window = builder.get_object("main_window").unwrap();
    let about_item: MenuItem = builder.get_object("about_item").unwrap();
    let about_dialog: AboutDialog = builder.get_object("about_dialog").unwrap();
    let status_bar: Statusbar = builder.get_object("status_bar").unwrap();

    let interface = Interface::new(&builder);
    let files_tree = interface.create_files_tree();

    about_item.connect_activate(move |_| {
        about_dialog.run();
        about_dialog.hide();
    });

    let pause_button: ToolButton = builder.get_object("pause_button").unwrap();
    pause_button.connect_clicked(|button| {
        let client = Client::new("http://localhost:6789");
        let widget = button.clone().upcast::<Widget>();

        widget.set_sensitive(false);

        if button.get_stock_id().unwrap() == "gtk-media-pause" {
            client.pause_download();
        } else {
            client.resume_download();
        }

        widget.set_sensitive(true);
    });

    let context_id = status_bar.get_context_id("");
    status_bar.push(context_id, "nzbget-ui");

    let files_store = ListStore::new(&[Type::String, Type::String, Type::String, Type::F64]);
    files_tree.set_model(Some(&files_store));

    let widget = files_tree.upcast::<Widget>();

    widget.connect_button_release_event(|_, event| {
        if event.get_button() != 3 {
            return Inhibit(false)
        }

        let popup_menu = Menu::new();

        let item = MenuItem::new_with_label("resume");
        popup_menu.append(&item);

        item.connect_activate(|_| {
            println!("aaa");
        });

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

    let rendered_groups: Mutex<HashMap<u32, TreeIter>> = Mutex::new(HashMap::new());

    let (tx, rx) = channel();
    GLOBAL.with(move |global| {
        *global.borrow_mut() = Some((builder, files_store, rendered_groups, rx))
    });

    thread::spawn(move || {
        loop {
            let client = Client::new("http://localhost:6789");

            let groups = client.load_groups();
            let status = client.load_status();

            tx.send((groups, status)).unwrap();

            thread::sleep(std::time::Duration::from_millis(1000));
            glib::idle_add(receive);
        }
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

fn render_group(group: &Group, files_store: &ListStore) -> TreeIter {
    let human_size = group.file_size().file_size(options::CONVENTIONAL).unwrap();
    files_store.insert_with_values(Some(group.NZBID), &[0, 1, 2, 3], &[&group.NZBNicename, &group.Status, &human_size, &group.progress()])
}

fn update_group(group: &Group, iter: &TreeIter, files_store: &ListStore) {
    let human_size = group.file_size().file_size(options::CONVENTIONAL).unwrap();
    files_store.set(iter, &[0, 1, 2, 3], &[&group.NZBNicename, &group.Status, &human_size, &group.progress()])
}

fn receive() -> glib::Continue {
    GLOBAL.with(|global| {
        if let Some((ref builder, ref files_store, ref rendered_groups, ref rx)) = *global.borrow() {
            if let Ok(data) = rx.try_recv() {
                let (groups, status) = data;

                let pause_button: ToolButton = builder.get_object("pause_button").unwrap();

                if status.DownloadPaused {
                    pause_button.set_stock_id("gtk-media-play");
                } else {
                    pause_button.set_stock_id("gtk-media-pause");
                }

                let mut rendered_groups = rendered_groups.lock().unwrap();

                for group in groups {
                    if rendered_groups.contains_key(&group.NZBID) {
                        update_group(&group, rendered_groups.get(&group.NZBID).unwrap(), &files_store);
                        continue;
                    }

                    let rendered_group = render_group(&group, &files_store);
                    rendered_groups.insert(group.NZBID, rendered_group);
                }

                let mut human_speed = status.DownloadRate.file_size(options::CONVENTIONAL).unwrap();
                human_speed.push_str("/s");

                let current_speed: Label = builder.get_object("current_speed").unwrap();
                current_speed.set_label(&human_speed);
            }
        }
    });
    glib::Continue(false)
}

thread_local!(
    static GLOBAL: RefCell<Option<(Builder, ListStore, Mutex<HashMap<u32, TreeIter>>, Receiver<(Vec<Group>, Status)>)>> = RefCell::new(None)
);
