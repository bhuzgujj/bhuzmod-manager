use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Save {
	version: Version,
	region: Region
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Version {
	#[serde(rename = "@major")]
	major: u16,
	#[serde(rename = "@minor")]
	minor: u16,
	#[serde(rename = "@revision")]
	revision: u16,
	#[serde(rename = "@build")]
	build: u16,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Region {
	#[serde(rename = "@id")]
	id: String,
	node: Root
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Root {
	#[serde(rename = "@id")]
	id: String,
	children: Option<Children>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Children {
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Attribute {
	#[serde(rename = "@id")]
	id: String,
	#[serde(rename = "@type")]
	types: String,
	#[serde(rename = "@value")]
	value: String,
}