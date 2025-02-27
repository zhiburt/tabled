use crate::error::Error;

/// Defines the casing for the attributes long representation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CasingStyle {
    /// Indicate word boundaries with uppercase letter, excluding the first word.
    Camel,
    /// Keep all letters lowercase and indicate word boundaries with hyphens.
    Kebab,
    /// Indicate word boundaries with uppercase letter, including the first word.
    Pascal,
    /// Keep all letters uppercase and indicate word boundaries with underscores.
    ScreamingSnake,
    /// Keep all letters lowercase and indicate word boundaries with underscores.
    Snake,
    /// Keep all letters lowercase and remove word boundaries.
    Lower,
    /// Keep all letters uppercase and remove word boundaries.
    Upper,
    /// Split string by words and start each with upper latter.
    UpperTitle,
    /// Split string by words.
    LowerTitle,
    /// Use the original attribute name defined in the code.
    Verbatim,
}

impl CasingStyle {
    pub fn from_lit(name: &syn::LitStr) -> Result<Self, Error> {
        use self::CasingStyle::*;
        use heck::ToUpperCamelCase;

        let normalized = name.value().to_upper_camel_case().to_lowercase();

        match normalized.as_ref() {
            "camel" | "camelcase" => Ok(Camel),
            "kebab" | "kebabcase" => Ok(Kebab),
            "pascal" | "pascalcase" => Ok(Pascal),
            "screamingsnake" | "screamingsnakecase" => Ok(ScreamingSnake),
            "snake" | "snakecase" => Ok(Snake),
            "lower" | "lowercase" => Ok(Lower),
            "upper" | "uppercase" => Ok(Upper),
            "uppertitle" | "uppertitlecase" => Ok(UpperTitle),
            "lowertitle" | "lowertitlecase" => Ok(LowerTitle),
            "verbatim" | "verbatimcase" => Ok(Verbatim),
            _ => {
                const SUPPORTED: &str = "supported values are ['camelCase', 'kebab-case', 'PascalCase', 'SCREAMING_SNAKE_CASE', 'snake_case', 'lowercase', 'UPPERCASE', 'lower title case', 'Upper Title Case', 'verbatim']";

                Err(Error::new(
                    format!("unsupported casing: `{:?}`", name.value()),
                    name.span(),
                    Some(SUPPORTED.to_owned()),
                ))
            }
        }
    }

    pub fn cast(self, text: String) -> String {
        use CasingStyle::*;

        match self {
            Pascal => heck::ToUpperCamelCase::to_upper_camel_case(text.as_str()),
            Camel => heck::ToLowerCamelCase::to_lower_camel_case(text.as_str()),
            Kebab => heck::ToKebabCase::to_kebab_case(text.as_str()),
            Snake => heck::ToSnakeCase::to_snake_case(text.as_str()),
            ScreamingSnake => heck::ToShoutySnakeCase::to_shouty_snake_case(text.as_str()),
            Lower => heck::ToSnakeCase::to_snake_case(text.as_str()).replace('_', ""),
            Upper => heck::ToShoutySnakeCase::to_shouty_snake_case(text.as_str()).replace('_', ""),
            LowerTitle => heck::ToSnakeCase::to_snake_case(text.as_str()).replace('_', " "),
            UpperTitle => heck::ToTitleCase::to_title_case(text.as_str()),
            Verbatim => text,
        }
    }
}
