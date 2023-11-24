use serde::{Serialize, Deserialize};
use serde_inline_default::serde_inline_default;

/// A state of an action.
#[serde_inline_default]
#[derive(Serialize, Deserialize)]
pub struct ActionState {
	#[serde(alias = "Image")]
	pub image: String,

	#[serde_inline_default(String::from(""))]
	#[serde(alias = "Name")]
	pub name: String,

	#[serde_inline_default(String::from(""))]
	#[serde(alias = "Title")]
	pub text: String,

	#[serde_inline_default(true)]
	#[serde(alias = "ShowTitle")]
	pub show: bool,

	#[serde_inline_default(String::from("#f2f2f2"))]
	#[serde(alias = "TitleColor")]
	pub colour: String,

	#[serde_inline_default(String::from("middle"))]
	#[serde(alias = "TitleAlignment")]
	pub alignment: String,

	#[serde_inline_default(String::from("Regular"))]
	#[serde(alias = "FontStyle")]
	pub style: String,

	#[serde_inline_default(String::from("16"))]
	#[serde(alias = "FontSize")]
	pub size: String,

	#[serde_inline_default(false)]
	#[serde(alias = "FontUnderline")]
	pub underline: bool
}

/// An action, deserialised from the plugin manifest.
#[serde_inline_default]
#[derive(Serialize, Deserialize)]
pub struct Action {
	#[serde(alias = "Name")]
	pub name: String,

	#[serde(alias = "UUID")]
	pub uuid: String,

	#[serde_inline_default(String::from(""))]
	pub plugin: String,

	#[serde(alias = "Tooltip")]
	pub tooltip: String,

	#[serde(alias = "Icon")]
	pub icon: String,

	#[serde_inline_default(true)]
	#[serde(alias = "VisibleInActionsList")]
	pub visible_in_action_list: bool,

	#[serde_inline_default(String::from(""))]
	#[serde(alias = "PropertyInspectorPath")]
	pub property_inspector: String,

	#[serde_inline_default(vec![String::from("Keypad")])]
	#[serde(alias = "Controllers")]
	pub controllers: Vec<String>,
	
	#[serde(alias = "States")]
	pub states: Vec<ActionState>
}

/// Information about the slot an instance is located in.
#[derive(Serialize, Deserialize)]
pub struct ActionContext {
	pub device: String,
	pub profile: String,
	pub controller: String,
	pub position: u8,
	pub index: u16 
}

impl ToString for ActionContext {
	fn to_string(&self) -> String {
		format!("{}.{}.{}.{}.{}", self.device, self.profile, self.controller, self.position, self.index)
	}
}

/// An instance of an action.
#[derive(Serialize, Deserialize)]
pub struct ActionInstance {
	pub action: Action,
	pub context: ActionContext,
	pub states: Vec<ActionState>,
	pub current_state: u16,
	pub settings: String
}
