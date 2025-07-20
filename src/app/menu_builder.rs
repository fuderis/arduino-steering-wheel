use crate::prelude::*;
use tauri::{ menu::{ Menu, MenuItem, IsMenuItem }, AppHandle, Runtime };
use std::collections::VecDeque;

/// The system tray menu builder
pub struct MenuBuilder {
    items: VecDeque<(String, String, bool, Option<String>)>,
}

impl ::std::default::Default for MenuBuilder {
    fn default() -> Self {
        Self {
            items: vec_deque![],
        }
    }
}

impl MenuBuilder {
    /// Creates a new menu item
    pub fn item<S: Into<String>>(mut self, id: S, text: S, enabled: bool, /* accelerator: Option<S> */) -> Self {
        self.items.push_back((id.into(), text.into(), enabled, None, /* accelerator.map(|s| s.into()) */));
        self
    }

    /// Builds the system tray menu
    pub(super) fn build<R: Runtime>(&self, app_handle: &AppHandle<R>) -> Menu<R> {
        // creating MenuItem's:
        let menu_items: Vec<MenuItem<R>> = self.items.iter()
            .map(|(id, text, enabled, accelerator)| {
                MenuItem::with_id(
                    app_handle,
                    id,
                    text,
                    *enabled,
                    accelerator.as_deref(),
                ).unwrap()
            })
            .collect();

        // converting to Vec<&dyn IsMenuItem<R>>:
        let menu_refs: Vec<&dyn IsMenuItem<R>> = menu_items.iter().map(|item| item as &dyn IsMenuItem<R>).collect();

        Menu::with_items(app_handle, &menu_refs).unwrap()
    }
}
