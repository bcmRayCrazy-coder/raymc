fn main() {
    #[cfg(target_os = "windows")]
    {
        use winres::WindowsResource;
        let mut res = WindowsResource::new();
        res.set_icon("assets/winres/icon.ico");
        res.compile().unwrap();
    }
}
