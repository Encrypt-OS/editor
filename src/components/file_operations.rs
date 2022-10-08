use super::misc::*;
use super::{OpenDialog, SaveDialog, ErrorDialog};
use crate::state::ActiveMetadata;
use gtk::*;
use sourceview::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::sync::RwLock;
use faccess::PathExt;

pub enum SaveAction {
    New(ActiveMetadata),
    Saved,
    Canceled,
}

pub fn save(
    editor: &Buffer,
    headerbar: &HeaderBar,
    window: &Window,
    path_label: &Label,
    save: &Button,
    current_file: &RwLock<Option<ActiveMetadata>>,
    save_as: bool,
) {
    if let Some(text) = get_buffer(editor) {
        let result = if save_as {
            write_data(None, text.as_bytes())
        } else {
            write_data(current_file.read().unwrap().as_ref(), text.as_bytes())
        };

        match result {
            Ok(SaveAction::New(file)) => {
                path_label.set_text(&file.get_path().to_string_lossy());
                if let Some(filename) = file.get_path().file_name() {
                    headerbar.set_subtitle(Some(&filename.to_string_lossy()));
                }

                let mut current_file = current_file.write().unwrap();
                *current_file = Some(file);
                save.set_sensitive(false);
            }

            Ok(SaveAction::Saved) => {
                if let Some(ref mut current_file) = *current_file.write().unwrap() {
                    current_file.set_sum(&text.as_bytes());
                    save.set_sensitive(false);
                }
            }

            Ok(SaveAction::Canceled) => (),

            _ => {
                let error_dialog = ErrorDialog::new(&window);
                let _ = error_dialog.run();
            }
        }
    }
}

fn write_data(path: Option<&ActiveMetadata>, data: &[u8]) -> io::Result<SaveAction> {
    if let Some(path) = path {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path.get_path())?;
        file.write_all(&data)?;
        return Ok(SaveAction::Saved);
    }

    let save_dialog = SaveDialog::new(None);
    if let Some(new_path) = save_dialog.run() {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&new_path)?;
        file.write_all(&data)?;
        Ok(SaveAction::New(ActiveMetadata::new(new_path, data)))
    } else {
        Ok(SaveAction::Canceled)
    }
}

pub fn save_before_close(
    editor: &Buffer,
    window: &Window,
    save: &Button,
    current_file: &RwLock<Option<ActiveMetadata>>,
) -> bool
{
    let mut is_saved = false;
    if let Some(text) = get_buffer(editor) {
        let result = write_data(current_file.read().unwrap().as_ref(), text.as_bytes());

        match result {
            Ok(SaveAction::New(file)) => {
                let mut current_file = current_file.write().unwrap();
                *current_file = Some(file);
                save.set_sensitive(false);
                is_saved = true;
            }

            Ok(SaveAction::Saved) => {
                if let Some(ref mut current_file) = *current_file.write().unwrap() {
                    current_file.set_sum(&text.as_bytes());
                    save.set_sensitive(false);
                }
                is_saved = true;
            }

            Ok(SaveAction::Canceled) => (),

            _ => {
                let error_dialog = ErrorDialog::new(&window);
                let _ = error_dialog.run();
            }
        }
    }
    is_saved
}

pub fn open(editor: &Buffer, headerbar: &HeaderBar, path_label: &Label, current_file: &RwLock<Option<ActiveMetadata>>) {
    let open_dialog = OpenDialog::new({
        let lock = current_file.read().unwrap();
        if let Some(ref path) = *lock {
            path.get_dir()
        } else {
            None
        }
    });

    if let Some(new_file) = open_dialog.run() {
        if let Ok(mut file) = File::open(&new_file) {
            let mut contents = String::new();
            let _ = file.read_to_string(&mut contents);

            if new_file.writable() {
                path_label.set_text(&new_file.to_string_lossy());
            } else {
                path_label.set_text(&format!("{} - [READ ONLY]", &new_file.to_string_lossy()));
            }
            if let Some(filename) = new_file.file_name() {
                headerbar.set_subtitle(Some(&filename.to_string_lossy()));
            }

            *current_file.write().unwrap() = Some(ActiveMetadata::new(new_file, &contents.as_bytes()));
            editor.set_text(&contents);
            editor.place_cursor(&editor.get_start_iter());
        }
    }
}

pub fn open_from_files(
    editor: &Buffer,
    headerbar: &HeaderBar,
    path_label: &Label,
    current_file: &RwLock<Option<ActiveMetadata>>,
    path: String,
) {
    let new_file = PathBuf::from(path);
    if let Ok(mut file) = File::open(&new_file) {
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);

        if new_file.writable() {
            path_label.set_text(&new_file.to_string_lossy());
        } else {
            path_label.set_text(&format!("{} - [READ ONLY]", &new_file.to_string_lossy()));
        }
        if let Some(filename) = new_file.file_name() {
            headerbar.set_subtitle(Some(&filename.to_string_lossy()));
        }

        *current_file.write().unwrap() = Some(ActiveMetadata::new(new_file, &contents.as_bytes()));
        editor.set_text(&contents);
        editor.place_cursor(&editor.get_start_iter());
    }
}