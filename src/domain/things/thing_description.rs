// use derive_more::From;
use unicode_segmentation::UnicodeSegmentation;
extern crate derive_more;

use crate::prelude::*;

#[derive(
	Clone,
	serde::Deserialize,
	Debug,
	serde::Serialize,
	PartialEq,
	derive_more::From,
	derive_more::Into,
	derive_more::AsRef,
	sqlx::Type,
)]
pub struct ThingDescription(String);

/// Implementation of the default Thing for creating a new thing.
///
/// You can also use the #[derive(default)]
impl Default for ThingDescription {
	fn default() -> Self {
		Self("No description provided...".to_string())
	}
}

impl ThingDescription {
    /// Returns an instance of `ThingDescription` if the input satisfies all
    /// our validation constraints on subscriber names.
    /// It panics otherwise.
    pub fn parse(description: impl Into<String>) -> Result<ThingDescription> {
        let description: String = description.into();

        // `.trim()` returns a view over the input `s` without trailing
        // whitespace-like characters.
        // `.is_empty` checks if the view contains any character.
        let is_empty_or_whitespace = description.trim().is_empty();

        // A grapheme is defined by the Unicode standard as a "user-perceived"
        // character: `å` is a single grapheme, but it is composed of two characters
        // (`a` and `̊`).
        //
        // `graphemes` returns an iterator over the graphemes in the input `s`.
        // `true` specifies that we want to use the extended grapheme definition set,
        // the recommended one.
        let is_too_long = description.graphemes(true).count() > 256;

        // Iterate over all characters in the input `s` to check if any of them matches
        // one of the characters in the forbidden array.
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = description.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(Error::ThingDescriptionValidationError { description })
        } else {
            Ok(Self(description))
        }
    }
}

// impl AsRef<str> for ThingDescription {
//     fn as_ref(&self) -> &str {
//         &self.0
//     }
// }

#[cfg(test)]
mod tests {
    use crate::domain::ThingDescription;
    use claim::{assert_err, assert_ok};

    // Override with more flexible error
    pub type Result<T> = core::result::Result<T, Error>;
	pub type Error = Box<dyn std::error::Error>;

    use fake::faker::lorem::en::*;
    use fake::Fake;

        #[test]
	fn thing_description_default() -> Result<()> {
        let default_thing_description = ThingDescription::default();
        let check = ThingDescription::parse("No description provided...")?;
        assert_eq!(default_thing_description, check);

		Ok(())
	}

    #[test]
    fn a_256_grapheme_long_name_is_valid() -> Result<()> {
        let description = "a̐".repeat(256);
        assert_ok!(ThingDescription::parse(description));

        Ok(())
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() -> Result<()> {
        let description = "a".repeat(257);
        assert_err!(ThingDescription::parse(description.clone()));
        assert!(
            matches!(
                ThingDescription::parse(description),
                Err(crate::error::Error::ThingDescriptionValidationError {..} )
            )
        );

        Ok(())
    }

    #[test]
    fn whitespace_only_names_are_rejected() -> Result<()> {
        let description = " ".to_string();
        assert_err!(ThingDescription::parse(description.clone()));
        assert!(
            matches!(
                ThingDescription::parse(description),
                Err(crate::error::Error::ThingDescriptionValidationError {..} )
            )
        );
        Ok(())
    }

    #[test]
    fn empty_string_is_rejected() -> Result<()> {
        let description = "".to_string();
        assert_err!(ThingDescription::parse(description.clone()));
        assert!(
            matches!(
                ThingDescription::parse(description),
                Err(crate::error::Error::ThingDescriptionValidationError {..} )
            )
        );
        Ok(())
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() -> Result<()> {
        for description in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let description = description.to_string();
            assert_err!(ThingDescription::parse(description.clone()));
            assert!(
                matches!(
                ThingDescription::parse(description),
                Err(crate::error::Error::ThingDescriptionValidationError {..} )
            )
        );        }

        Ok(())
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() -> Result<()> {
        let description: String = Sentence(3..7).fake();
        assert_ok!(ThingDescription::parse(description));

        Ok(())
    }
}