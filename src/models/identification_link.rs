/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentificationLink {
	#[serde(rename = "type")]
	pub r#type: Type,
	#[serde(rename = "id")]
	pub id: String,
}

impl IdentificationLink {
	pub fn new(r#type: Type, id: String) -> IdentificationLink {
		IdentificationLink { r#type, id }
	}
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
	#[serde(rename = "oauth_google")]
	OauthGoogle,
	#[serde(rename = "oauth_twitter")]
	OauthTwitter,
	#[serde(rename = "oauth_mock")]
	OauthMock,
	#[serde(rename = "saml")]
	Saml,
	#[serde(other)]
	Other,
}

impl Default for Type {
	fn default() -> Type {
		Self::OauthGoogle
	}
}
