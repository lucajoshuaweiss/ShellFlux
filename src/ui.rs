use gtk4::prelude::*;
use gtk4::{
    ApplicationWindow, Box, Button, Entry, Label, ListBox, ListBoxRow, Orientation,
    ResponseType, ScrolledWindow, TextView, WrapMode,
};
use glib::clone;
use std::cell::RefCell;
use std::rc::Rc;
use crate::file_operations;
use crate::shell_operations;

pub fn build_ui(app: &gtk4::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("ShellFlux")
        .default_width(800)
        .default_height(600)
        .build();

    let scripts_dir = "/home/user/.shellflux";
    file_operations::ensure_scripts_directory(scripts_dir);

    let scripts_map = Rc::new(RefCell::new(file_operations::load_scripts(scripts_dir)));

    // This holds all ui components
    let main_box = Box::new(Orientation::Horizontal, 6);

    // This holds the sidebar, the button to create a new shell script and the delete button
    let left_box = Box::new(Orientation::Vertical, 6);

    // This holds the editing textview, the response textview, the save button and the run shellscript button
    let right_box = Box::new(Orientation::Vertical, 6);

    // This shows all shell scripts that exist inside of the scripts_dir
    let sidebar = ListBox::new();
    sidebar.set_vexpand(true);
    sidebar.set_hexpand(false);

    let selected_script = Rc::new(RefCell::new(None::<String>));
    let text_view = TextView::new();
    let buffer = text_view.buffer();

    let mut script_names: Vec<_> = scripts_map.borrow().keys().cloned().collect();
    script_names.sort();

    for name in script_names {
        let row = ListBoxRow::new();
        let label = Label::new(Some(&name));
        row.set_child(Some(&label));
        sidebar.append(&row);
    }

    // Change current script when it is selected in the sidebar
    sidebar.connect_row_selected(clone!(@strong buffer, @strong scripts_map, @strong selected_script => move |_, row_opt| {
        if let Some(row) = row_opt {
            if let Some(label) = row.child().and_then(|w| w.downcast::<Label>().ok()) {
                let title = label.text().to_string();
                if let Some(content) = scripts_map.borrow().get(&title) {
                    buffer.set_text(content);
                    *selected_script.borrow_mut() = Some(title);
                }
            }
        }
    }));

    left_box.append(&sidebar);

    let new_file_button = Button::with_label("New Script");
    left_box.append(&new_file_button);

    new_file_button.connect_clicked(clone!(@strong sidebar, @strong scripts_map, @strong selected_script, @strong buffer, @strong window => move |_| {
        let dialog = gtk4::Dialog::with_buttons(
            Some("New Script"),
            Some(&window),
            gtk4::DialogFlags::MODAL,
            &[("Create", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
        );

        let entry = Entry::new();
        entry.set_placeholder_text(Some("Script name (no extension)"));
        dialog.content_area().append(&entry);
        dialog.set_default_response(ResponseType::Ok);
        dialog.show();

        dialog.connect_response(clone!(@strong sidebar, @strong scripts_map, @strong selected_script, @strong buffer => move |dialog, resp| {
            if resp == ResponseType::Ok {
                let name = entry.text().to_string().trim().to_string();
                if !name.is_empty() {
                    file_operations::save_script_to_file(scripts_dir, &name, "");

                    let row = ListBoxRow::new();
                    let label = Label::new(Some(&name));
                    row.set_child(Some(&label));
                    sidebar.append(&row);
                    sidebar.select_row(Some(&row));

                    scripts_map.borrow_mut().insert(name.clone(), String::new());
                    *selected_script.borrow_mut() = Some(name);
                    buffer.set_text("");
                }
            }
            dialog.close();
        }));
    }));

    let delete_file_button = Button::with_label("Delete Script");
    left_box.append(&delete_file_button);

    delete_file_button.connect_clicked(clone!(@strong sidebar, @strong scripts_map, @strong selected_script, @strong buffer => move |_| {
        let title = {
            let script_ref = selected_script.borrow();
            if let Some(title) = &*script_ref {
                Some(title.clone())
            } else {
                None
            }
        };

        if let Some(title) = title {
            file_operations::delete_script_from_file(scripts_dir, &title);

            if let Some(selected_row) = sidebar.selected_row() {
                sidebar.remove(&selected_row);
            }

            scripts_map.borrow_mut().remove(&title);

            *selected_script.borrow_mut() = Some(String::new());
            buffer.set_text("");
        } else {
            eprintln!("No script selected.");
        }
    }));

    let scroll = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .child(&text_view)
        .build();
    right_box.append(&scroll);

    let shell_output_view = TextView::new();
    shell_output_view.set_editable(false);
    shell_output_view.set_cursor_visible(false);
    shell_output_view.set_wrap_mode(WrapMode::Word);

    let shell_scroll = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .child(&shell_output_view)
        .min_content_height(100)
        .build();
    right_box.append(&shell_scroll);

    let save_button = Button::with_label("Save Script");
    right_box.append(&save_button);

    save_button.connect_clicked(clone!(@strong scripts_map, @strong selected_script, @strong buffer => move |_| {
        if let Some(title) = &*selected_script.borrow() {
            if title.is_empty() { return; }
            let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false).to_string();
            scripts_map.borrow_mut().insert(title.clone(), text.clone());
            file_operations::save_script_to_file(scripts_dir, title, &text);
        } else {
            eprintln!("No script selected.");
        }
    }));

    let run_script_button = Button::with_label("Run Script");
    right_box.append(&run_script_button);

    run_script_button.connect_clicked(clone!(@strong text_view, @strong shell_output_view => move |_| {
        let cmd_text = text_view
            .buffer()
            .text(&text_view.buffer().start_iter(), &text_view.buffer().end_iter(), false)
            .to_string();

        shell_operations::operation_with_status(&shell_output_view, "bash", "-c", &cmd_text);
    }));

    main_box.append(&left_box);
    main_box.append(&right_box);

    window.set_child(Some(&main_box));
    window.show();
}
