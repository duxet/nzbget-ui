macro_rules! add_column {
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
