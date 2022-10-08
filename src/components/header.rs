use gio::SettingsExt;
use gtk::*;

pub struct Header {
    pub container: HeaderBar,
    pub open: Button,
    pub save: Button,
    pub save_as: Button,
    pub theme_switch: Switch,
    pub font_button: FontButton,
    pub find_button: ToggleButton,
    pub enable_wrapping_word: CheckButton,
    pub enable_wrapping_char: CheckButton,
}

impl Header {
    pub fn new() -> Header {
        let container = HeaderBar::new();
        container.set_title(Some("eddit"));
        container.set_subtitle(Some("New file"));
        container.set_show_close_button(true);

        let settings = gio::Settings::new("com.github.maze-n.eddit");

        let open = Button::new_from_icon_name(Some("document-open"), IconSize::LargeToolbar);
        open.set_tooltip_text(Some("Open a file\n   Ctrl + O"));
        open.set_valign(Align::Center);
        let save = Button::new_from_icon_name(Some("document-save"), IconSize::LargeToolbar);
        save.set_tooltip_text(Some("Save file\n Ctrl + S"));
        save.set_valign(Align::Center);
        let save_as = Button::new_from_icon_name(Some("document-save-as"), IconSize::LargeToolbar);
        save_as.set_tooltip_text(Some("Save as"));
        save_as.set_valign(Align::Center);
        let menu_button = MenuButton::new();
        menu_button.set_tooltip_text(Some("Preferences"));
        menu_button.set_valign(Align::Center);
        menu_button.set_image(Some(&Image::new_from_icon_name(Some("open-menu"), IconSize::LargeToolbar)));
        let find_button = ToggleButton::new();
        find_button.set_image(Some(&Image::new_from_icon_name(Some("edit-find"), IconSize::LargeToolbar)));
        find_button.set_valign(Align::Center);
        find_button.set_tooltip_text(Some("Find and replace\n         Ctrl + F"));

        let popover = Popover::new(Some(&menu_button));
        let pop_container = Box::new(Orientation::Vertical, 6);
        pop_container.set_border_width(12);

        let theme_selector = Grid::new();
        theme_selector.set_column_homogeneous(true);
        theme_selector.set_row_homogeneous(true);
        theme_selector.set_column_spacing(12);
        theme_selector.set_hexpand(true);

        let light_icon = Image::new_from_file("/opt/com.github.maze-n.eddit/icons/day.svg");
        light_icon.set_halign(Align::End);
        let dark_icon = Image::new_from_file("/opt/com.github.maze-n.eddit/icons/night.svg");
        dark_icon.set_halign(Align::Start);
        let theme_switch = Switch::new();
        theme_switch.set_tooltip_text(Some("Toggle editor theme"));
        theme_switch.set_halign(Align::Center);

        theme_selector.add(&light_icon);
        theme_selector.add(&theme_switch);
        theme_selector.add(&dark_icon);

        let font_button = FontButton::new();
        font_button.set_halign(Align::Center);
        font_button.set_tooltip_text(Some("Select font and size"));
        font_button.set_use_font(true);
        if let Some(font) = settings.get_string("font") {
            font_button.set_font(font.as_str());
        }

        let text_wrap_header_label = Label::new(Some("Text Wrapping"));
        text_wrap_header_label.set_halign(Align::Start);

        let enable_wrapping_word = CheckButton::new_with_label("Enable text wrapping");
        let word_wrap = settings.get_boolean("text-wrap-word");
        enable_wrapping_word.set_active(word_wrap);

        let enable_wrapping_char = CheckButton::new_with_label("Split words over two lines");
        let char_wrap = settings.get_boolean("text-wrap-char");
        enable_wrapping_char.set_active(char_wrap);

        let revealer = Revealer::new();
        revealer.set_transition_type(RevealerTransitionType::SlideDown);
        revealer.add(&enable_wrapping_char);
        revealer.set_reveal_child(word_wrap);

        let revealer_clone = revealer.clone();
        enable_wrapping_word.connect_toggled(move |enable_wrapping_word| {
            revealer_clone.set_reveal_child(enable_wrapping_word.get_active());
        });

        pop_container.pack_start(&theme_selector, true, true, 0);
        pop_container.pack_start(&Separator::new(Orientation::Horizontal), true, true, 6);
        pop_container.pack_start(&font_button, true, true, 0);
        pop_container.pack_start(&Separator::new(Orientation::Horizontal), true, true, 0);
        pop_container.pack_start(&text_wrap_header_label, true, true, 0);
        pop_container.pack_start(&enable_wrapping_word, true, true, 0);
        pop_container.pack_start(&revealer, true, true, 0);
        pop_container.show_all();

        popover.add(&pop_container);

        menu_button.set_popover(Some(&popover));

        container.pack_start(&open);
        container.pack_start(&save);
        container.pack_start(&save_as);
        container.pack_end(&menu_button);
        container.pack_end(&find_button);

        Header {
            container,
            open,
            save,
            save_as,
            theme_switch,
            font_button,
            find_button,
            enable_wrapping_word,
            enable_wrapping_char,
        }
    }
}