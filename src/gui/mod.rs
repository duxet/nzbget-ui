#[macro_use] mod macros;

use gtk::{Builder, CellRendererText, CellRendererProgress, TreeView, TreeViewColumn};
use gtk::prelude::*;

pub struct Interface {
    pub builder: Builder
}

impl Interface {
    pub fn new() -> Self {
        let builder = Builder::new_from_string(include_str!("interface.glade"));
        Interface { builder }
    }

    pub fn create_files_tree(&self) -> TreeView {
        let files_tree: TreeView = self.builder.get_object("files_tree").unwrap();

        add_text_column!(files_tree, "Title", 1);
        add_text_column!(files_tree, "Status", 2);
        add_text_column!(files_tree, "Size", 3);
        add_progress_column!(files_tree, "Progress", 4);

        files_tree
    }
}
