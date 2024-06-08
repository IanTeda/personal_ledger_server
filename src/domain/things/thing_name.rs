use unicode_segmentation::UnicodeSegmentation;

use crate::prelude::*;
extern crate derive_more;

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
pub struct ThingName(String);

/// Implementation of the default Thing for creating a new thing.
///
/// You can also use the #[derive(default)]
impl Default for ThingName {
	fn default() -> Self {
		Self("No Name".to_string())
	}
}

impl ThingName {
	// type Error = Error;

	/// Returns an instance of `ThingName` if the input satisfies all
	/// our validation constraints on subscriber names.
	/// It panics otherwise.
	pub fn parse(name: impl Into<String>) -> Result<ThingName> {
        let name: String = name.into();

		// `.trim()` returns a view over the input `s` without trailing
		// whitespace-like characters.
		// `.is_empty` checks if the view contains any character.
		let is_empty_or_whitespace = name.trim().is_empty();

		// A grapheme is defined by the Unicode standard as a "user-perceived"
		// character: `å` is a single grapheme, but it is composed of two characters
		// (`a` and `̊`).
		//
		// `graphemes` returns an iterator over the graphemes in the input `s`.
		// `true` specifies that we want to use the extended grapheme definition set,
		// the recommended one.
		let is_too_long = name.graphemes(true).count() > 256;

		// Iterate over all characters in the input `s` to check if any of them matches
		// one of the characters in the forbidden array.
		let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
		let contains_forbidden_characters =
			name.chars().any(|g| forbidden_characters.contains(&g));

		if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
			// Err(Error::Generic(format!("{} is not a valid thing name.", name)))
			Err(Error::ThingNameValidationError { name })
		} else {
			Ok(Self(name))
		}
	}
}

#[cfg(test)]
mod tests {
	// Override with more flexible error
	pub type Result<T> = core::result::Result<T, Error>;
	pub type Error = Box<dyn std::error::Error>;

	use crate::domain::ThingName;
	use claim::{assert_err, assert_ok};
	use fake::faker::name::en::Name;
	use fake::Fake;

    #[test]
	fn thing_name_default() -> Result<()> {
        let default_thing_name = ThingName::default();
        let check = ThingName::parse("No Name")?;
        assert_eq!(default_thing_name, check);

		Ok(())
	}

	#[test]
	fn a_256_grapheme_long_name_is_valid() -> Result<()> {
		let name = "a̐".repeat(256);
		assert_ok!(ThingName::parse(name));

		Ok(())
	}

	#[test]
	fn a_name_longer_than_256_graphemes_is_rejected() -> Result<()> {
		let name = "a".repeat(257);
		assert_err!(ThingName::parse(name.clone()));
		assert!(
			matches!(
                ThingName::parse(name),
                Err(crate::error::Error::ThingNameValidationError {..} )
            )
		);

		Ok(())
	}

	#[test]
	fn whitespace_only_names_are_rejected() -> Result<()> {
		let name = " ".to_string();
		assert_err!(ThingName::parse(name.clone()));
		assert!(
			matches!(
                ThingName::parse(name),
                Err(crate::error::Error::ThingNameValidationError {..} )
            )
		);

		Ok(())
	}

	#[test]
	fn empty_string_is_rejected() -> Result<()> {
		let name = "".to_string();
		assert_err!(ThingName::parse(name.clone()));
		assert!(
			matches!(
                ThingName::parse(name),
                Err(crate::error::Error::ThingNameValidationError {..} )
            )
		);

		Ok(())
	}

	#[test]
	fn names_containing_an_invalid_character_are_rejected() -> Result<()> {
		for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
			let name = name.to_string();
			assert_err!(ThingName::parse(name.clone()));
			assert!(
				matches!(
                ThingName::parse(name),
                Err(crate::error::Error::ThingNameValidationError {..} )
            )
		);		}
		Ok(())
	}

	#[test]
	fn a_valid_name_is_parsed_successfully() -> Result<()> {
		let name: String = Name().fake();
		assert_ok!(ThingName::parse(name));

		Ok(())
	}
}
