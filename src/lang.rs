use std::fmt;

// Definition of Lang and Script lists are generated by build.rs
include!(concat!(env!("OUT_DIR"), "/lang.rs"));

impl Lang {
    /// Get enum by ISO 639-3 code as a string.
    ///
    /// # Example
    /// ```
    /// use whatlang::Lang;
    /// assert_eq!(Lang::from_code("ukr"), Some(Lang::Ukr));
    /// ```
    pub fn from_code<S: Into<String>>(code: S) -> Option<Lang> {
        lang_from_code(code)
    }

    /// Convert enum into ISO 639-3 code as a string.
    ///
    /// # Example
    /// ```
    /// use whatlang::Lang;
    /// assert_eq!(Lang::Ukr.code(), "ukr");
    /// ```
    pub fn code(&self) -> &str {
        lang_to_code(*self)
    }

    /// Get a language name in the language itself.
    ///
    /// # Example
    /// ```
    /// use whatlang::Lang;
    /// assert_eq!(Lang::Ukr.name(), "Українська");
    /// ```
    pub fn name(self) -> &'static str {
        lang_to_name(self)
    }

    /// Get a human readable name of the language in English.
    ///
    /// # Example
    /// ```
    /// use whatlang::Lang;
    /// assert_eq!(Lang::Deu.eng_name(), "German");
    /// ```
    pub fn eng_name(self) -> &'static str {
        lang_to_eng_name(self)
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub type LangProfile = &'static [&'static str];
pub type LangProfileList = &'static [(Lang, LangProfile)];

#[cfg(test)]
mod tests {
    use super::Lang;

    #[test]
    fn test_from_code() {
        assert_eq!(Lang::from_code("rus".to_string()), Some(Lang::Rus));
        assert_eq!(Lang::from_code("ukr"), Some(Lang::Ukr));
        assert_eq!(Lang::from_code("ENG"), Some(Lang::Eng));
        assert_eq!(Lang::from_code("oops"), None);
    }

    #[test]
    fn test_code() {
        assert_eq!(Lang::Spa.code(), "spa");
    }

    #[test]
    fn test_name() {
        assert_eq!(Lang::Rus.name(), "Русский");
        assert_eq!(Lang::Spa.name(), "Español");
        assert_eq!(Lang::Epo.name(), "Esperanto");
    }

    #[test]
    fn test_eng_name() {
        assert_eq!(Lang::Spa.eng_name(), "Spanish");
        assert_eq!(Lang::Epo.eng_name(), "Esperanto");
        assert_eq!(Lang::Rus.eng_name(), "Russian");
    }
}
