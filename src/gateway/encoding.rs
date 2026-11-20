#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Encoding {
    #[default]
    Json,
    #[cfg(feature = "etf")]
    Etf,
}

impl Encoding {
    pub(crate) const fn query_value(self) -> &'static str {
        match self {
            Self::Json => "json",
            #[cfg(feature = "etf")]
            Self::Etf => "etf",
        }
    }
}
