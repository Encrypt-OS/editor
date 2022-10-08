use gio::SettingsExt;
use glib::GString;
use gtk::*;
use sourceview::*;

pub fn get_buffer(buffer: &Buffer) -> Option<GString> {
    let (start, end) = buffer.get_bounds();
    buffer.get_text(&start, &end, true)
}

pub fn before_quit(window: &Window, word_wrap_checkbox: &CheckButton, char_wrap_checkbox: &CheckButton) {
    let size = window.get_size();
    let position = window.get_position();
    let is_maximized = window.get_property_is_maximized();
    let settings = gio::Settings::new("com.github.maze-n.eddit");
    let word_wrap = word_wrap_checkbox.get_active();
    let char_wrap = char_wrap_checkbox.get_active();

    settings.set_int("pos-x", position.0);
    settings.set_int("pos-y", position.1);
    settings.set_int("window-width", size.0);
    settings.set_int("window-height", size.1);
    settings.set_boolean("is-maximized", is_maximized);
    settings.set_boolean("text-wrap-word", word_wrap);
    settings.set_boolean("text-wrap-char", char_wrap);
}

pub fn set_sensitivity (entry: &SearchEntry, up: &Button, down: &Button, text: &str, iter: &TextIter) {
    let search_flag = TextSearchFlags::CASE_INSENSITIVE;
    if text != "" {
        if let Some(_) = iter.forward_search(text, search_flag, None) {
            down.set_sensitive(true);
        } else {
            down.set_sensitive(false);
        }
    
        if let Some(_) = iter.backward_search(text, search_flag, None) {
            up.set_sensitive(true);
        } else {
            up.set_sensitive(false);
        }
    } else {
        up.set_sensitive(false);
        down.set_sensitive(false);
    }

    if !up.get_sensitive() && !down.get_sensitive() && text != "" {
        entry.get_style_context().add_class(&gtk::STYLE_CLASS_ERROR);
        entry.set_icon_from_icon_name(EntryIconPosition::Primary, Some("dialog-error-symbolic"));
    } else {
        entry.get_style_context().remove_class(&gtk::STYLE_CLASS_ERROR);
        entry.set_icon_from_icon_name(EntryIconPosition::Primary, Some("edit-find-symbolic"));
    }
}