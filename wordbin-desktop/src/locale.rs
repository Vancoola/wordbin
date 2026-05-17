use crate::i18n::Locale;
use shared_types::settings::Language;

impl From<Language> for Locale {
    fn from(value: Language) -> Self {
        match value {
            Language::En => Locale::en,
            Language::Ru => Locale::ru,
            Language::De => Locale::de,
            Language::Fr => Locale::fr,
            Language::Ja => Locale::ja,
            Language::Ko => Locale::ko,
            Language::Zh => Locale::zh,
            Language::Es => Locale::es,
        }
    }
}

impl From<Locale> for Language {
    fn from(value: Locale) -> Self {
        match value {
            Locale::en => Language::En,
            Locale::ru => Language::Ru,
            Locale::de => Language::De,
            Locale::fr => Language::Fr,
            Locale::ja => Language::Ja,
            Locale::ko => Language::Ko,
            Locale::zh => Language::Zh,
            Locale::es => Language::Es,
        }
    }
}
