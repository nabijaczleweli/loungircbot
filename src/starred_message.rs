use regex::Regex;
use std::iter::FromIterator;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct StarredMessage {
	pub starrers: Vec<String>,
	pub stars   : u64,
	pub sender  : String,
	pub message : String,
}

impl StarredMessage {
	pub fn from_message_content(message: &str, starrer: Option<String>) -> Option<StarredMessage> {
		match starrer {
			Some(starrer) =>
				Self::regex().captures(message).map(move |captures| StarredMessage{
					starrers: vec![starrer],
					stars   : 1u64,
					sender  : captures[1].to_string(),
					message : captures[2].to_string(),
				}),
			None => None,
		}
	}


	fn regex() -> Regex {
		Regex::new(r#"[[:blank:]]*<([[:alnum:]\[\]`_^{|}-]{0,16})>[[:blank:]]*(.+)[[:blank:]]*"#).unwrap()
	}
}


#[cfg(test)]
mod tests {
	use starred_message::StarredMessage;


	#[test]
	fn message_extracts_correctly() {
		let msg = StarredMessage::from_message_content("<nabijaczleweli> I only clean 'round these parts", Some("thecoshman".to_string())).unwrap();
		assert_eq!(msg.sender, "nabijaczleweli");
		assert_eq!(msg.message, "I only clean 'round these parts");
	}

	#[test]
	fn message_fails_properly() {
		assert_eq!(StarredMessage::from_message_content("<nabijaczleweclean 'round these parts", Some("thecoshman".to_string())), None);
		assert_eq!(StarredMessage::from_message_content("<nabijaczleweli>", Some("thecoshman".to_string())), None);
	}

	#[test]
	fn message_propagates_starrer() {
		assert_eq!(StarredMessage::from_message_content("<nabijaczleweli> I only clean 'round these parts", None), None);
		assert_eq!(StarredMessage::from_message_content("<nabijaczleweli> I only clean 'round these parts",
		                                                Some("thecoshman".to_string())).unwrap().starrers, vec!["thecoshman".to_string()]);
	}

	#[test]
	fn message_defaults_to_1_star() {
		assert_eq!(StarredMessage::from_message_content("<nabijaczleweli> I only clean 'round these parts", Some("thecoshman".to_string())).unwrap().stars, 1);
	}
}
