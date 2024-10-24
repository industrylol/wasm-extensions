use std::sync::OnceLock;

pub use wit::DataVariant;
use wit::*;

pub trait Extension: Send + Sync {
    /// Returns a new instance of the extension.
    fn new() -> Self
    where
        Self: Sized;

    /// Invoke an extension function
    fn invoke_func(&self, fn_name: &str, args: &[DataVariant]) -> Result<DataVariant, String>;
}

pub mod wit {
    wit_bindgen::generate!({
        path: "./wit",
        skip: ["init-extension"],
        world: "extension",
    });
}

static EXTENSION: OnceLock<Box<dyn Extension>> = OnceLock::new();

pub fn register_extension(build: fn() -> Box<dyn Extension>) {
    let _ = EXTENSION.set(build());
}

#[macro_export]
macro_rules! register_extension {
    ($extension_type:ty) => {
        #[export_name = "init-extension"]
        pub extern "C" fn __init_extension() {
            api::register_extension(|| Box::new(<$extension_type as api::Extension>::new()));
        }
    };
}

wit::export!(Component);
struct Component;
impl wit::Guest for Component {
    fn invoke_func(fnname: String, args: Vec<DataVariant>) -> Result<DataVariant, String> {
        match EXTENSION.get() {
            Some(ext) => ext.invoke_func(&fnname, &args),
            None => Err("Extension not loaded".to_string()),
        }
    }
}
