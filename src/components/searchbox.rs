use gtk::*;

pub struct SearchBox {
    pub container: Box,
    pub search_entry: SearchEntry,
    pub replace_entry: Entry,
    pub replace_button: Button,
    pub replace_all_button: Button,
    pub up: Button,
    pub down: Button,
}

impl SearchBox {
    pub fn new() -> SearchBox {
        let container = Box::new(Orientation::Horizontal, 0);
        container.get_style_context().add_class("search-bar");

        let search_grid = Grid::new();
        search_grid
            .get_style_context()
            .add_class(&STYLE_CLASS_LINKED);
        search_grid.set_border_width(4);
        let search_entry = SearchEntry::new();
        search_entry.set_hexpand(true);
        search_entry.set_placeholder_text(Some("Find"));
        let up = Button::new_from_icon_name(Some("go-up-symbolic"), IconSize::SmallToolbar);
        up.set_tooltip_text(Some("Previous Item"));
        up.set_sensitive(false);
        let down = Button::new_from_icon_name(Some("go-down-symbolic"), IconSize::SmallToolbar);
        down.set_tooltip_text(Some("Next Item"));
        down.set_sensitive(false);

        search_grid.add(&search_entry);
        search_grid.add(&down);
        search_grid.add(&up);

        let replace_grid = Grid::new();
        replace_grid
            .get_style_context()
            .add_class(&STYLE_CLASS_LINKED);
        replace_grid.set_border_width(4);
        let replace_entry = Entry::new();
        replace_entry.set_hexpand(true);
        replace_entry.set_placeholder_text(Some("Replace With"));
        let replace_button = Button::new_with_label("Replace");
        let replace_all_button = Button::new_with_label("Replace All");
        replace_button.set_sensitive(false);
        replace_all_button.set_sensitive(false);

        replace_grid.add(&replace_entry);
        replace_grid.add(&replace_button);
        replace_grid.add(&replace_all_button);

        container.add(&search_grid);
        container.add(&replace_grid);

        SearchBox {
            container,
            search_entry,
            replace_entry,
            replace_button,
            replace_all_button,
            up,
            down,
        }
    }
}