use crate::app::app::{App, ViewPage};

impl App {
    pub fn handle_page_jump(&mut self, next_page: ViewPage) {
        self.view_page = next_page;
    }
}
