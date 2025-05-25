use once_cell::sync::Lazy;
use crate::config::config::AppConfig;

pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    AppConfig::init()
});