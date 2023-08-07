use tera::Tera;

use crate::configuration::Settings;
use crate::service::dydx::DydxService;

pub struct AppData {
    pub dydx: DydxService,
    pub tmpl: Tera,
}

impl AppData {
    pub fn new(settings: Settings) -> AppData {
        AppData {
            dydx: DydxService {
                settings
            },
            tmpl: setup_tera(),
        }
    }
}

fn setup_tera() -> Tera {
    Tera::new(
        concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")
    ).unwrap()
}