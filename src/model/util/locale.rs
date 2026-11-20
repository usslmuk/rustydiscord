use crate::model::macros::str_enum;

str_enum! {



















    pub enum Locale {

        Indonesian = "id",

        Danish = "da",

        German = "de",

        EnglishUk = "en-GB",

        EnglishUs = "en-US",

        Spanish = "es-ES",

        SpanishLatam = "es-419",

        French = "fr",

        Croatian = "hr",

        Italian = "it",

        Lithuanian = "lt",

        Hungarian = "hu",

        Dutch = "nl",

        Norwegian = "no",

        Polish = "pl",

        PortugueseBrazilian = "pt-BR",

        Romanian = "ro",

        Finnish = "fi",

        Swedish = "sv-SE",

        Vietnamese = "vi",

        Turkish = "tr",

        Czech = "cs",

        Greek = "el",

        Bulgarian = "bg",

        Russian = "ru",

        Ukrainian = "uk",

        Hindi = "hi",

        Thai = "th",

        ChineseChina = "zh-CN",

        Japanese = "ja",

        ChineseTaiwan = "zh-TW",

        Korean = "ko",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_and_unknown() {
        assert_eq!(
            serde_json::to_string(&Locale::SpanishLatam).unwrap(),
            r#""es-419""#
        );
        let unknown: Locale = serde_json::from_str(r#""tlh""#).unwrap();
        assert_eq!(unknown, Locale::Unknown("tlh".into()));
        assert_eq!(serde_json::to_string(&unknown).unwrap(), r#""tlh""#);

        assert_eq!(Locale::Unknown("pl".into()), Locale::Polish);
    }
}
