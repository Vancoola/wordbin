use leptos_i18n_build::{Config, TranslationsInfos};
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let i18n_mod_directory = PathBuf::from(std::env::var_os("OUT_DIR").expect("")).join("i18n");

    let cfg = Config::new("en")?
        .add_locale("ru")?
        .add_locale("de")?
        .add_locale("fr")?
        .add_locale("ja")?
        .add_locale("ko")?
        .add_locale("zh")?
        .add_locale("es")?;

    let translations_infos = TranslationsInfos::parse(cfg)?;

    translations_infos.emit_diagnostics();

    translations_infos.rerun_if_locales_changed();

    translations_infos.generate_i18n_module(i18n_mod_directory)?;

    Ok(())
}
