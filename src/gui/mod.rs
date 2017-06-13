#[macro_use] mod macros;

use gtk::{Builder, CellRendererText, CellRendererProgress, TreeView, TreeViewColumn};
use gtk::prelude::*;

pub struct Interface {
    builder: Builder
}

impl Interface {
    pub fn new(builder: &Builder) -> Self {
        Interface { builder: builder.clone() }
    }

    pub fn create_files_tree(&self) -> TreeView {
        let files_tree: TreeView = self.builder.get_object("files_tree").unwrap();

        add_text_column!(files_tree, "Title", 0);
        add_text_column!(files_tree, "Status", 1);
        add_text_column!(files_tree, "Size", 2);
        add_progress_column!(files_tree, "Progress", 3);

        files_tree
    }
}
