use crate::app::app::{App, ViewPageName};

impl App {
    pub fn handle_page_jump(&mut self, next_page: ViewPageName) {
        self.view_page = next_page;
    }
}
