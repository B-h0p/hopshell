extern crate winres;

fn main() {
    #[cfg(windows)] {
        let mut res = winres::WindowsResource::new();
            // This path can be absolute, or relative to your crate root.
            res.set_icon("./assets/icon.ico").compile().unwrap();
    }
}