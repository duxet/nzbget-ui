macro_rules! add_text_column {
    ($tree:ident, $title:expr, $id:expr) => {{
        let column   = TreeViewColumn::new();
        let renderer = CellRendererText::new();

        column.set_title($title);
        column.set_resizable(true);
        column.pack_start(&renderer, true);
        column.add_attribute(&renderer, "text", $id);
        $tree.append_column(&column);
    }}
}

macro_rules! add_progress_column {
    ($tree:ident, $title:expr, $id:expr) => {{
        let column   = TreeViewColumn::new();
        let renderer = CellRendererProgress::new();

        column.set_title($title);
        column.set_resizable(true);
        column.pack_start(&renderer, true);
        column.add_attribute(&renderer, "value", $id);
        $tree.append_column(&column);
    }}
}
