pub mod instances;
pub mod plugins;
pub mod profiles;
pub mod property_inspector;
pub mod settings;

use crate::devices::DEVICES;
use crate::shared::{Action, CATEGORIES};

use std::collections::HashMap;

use tauri::{command, Emitter, Manager};

#[derive(Debug, serde_with::SerializeDisplay, serde::Deserialize)]
pub struct Error {
	pub description: String,
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.description)
	}
}
impl std::error::Error for Error {}

impl Error {
	fn new(description: String) -> Self {
		log::error!("{}", description);
		Self { description }
	}
}

impl From<serde_json::Error> for Error {
	fn from(error: serde_json::Error) -> Self {
		Self::new(error.to_string())
	}
}

impl From<anyhow::Error> for Error {
	fn from(error: anyhow::Error) -> Self {
		Self::new(error.to_string())
	}
}

#[command]
pub async fn get_devices() -> HashMap<std::string::String, crate::devices::DeviceInfo> {
	DEVICES.read().await.clone()
}

pub async fn update_devices() -> Option<()> {
	let app = crate::APP_HANDLE.get().unwrap();
	let _ = app.get_webview_window("main")?.emit("devices", DEVICES.read().await.clone());
	Some(())
}

#[command]
pub async fn rescan_devices() {
	let devices = DEVICES.read().await;
	if devices.len() > 0 {
		return;
	}
	crate::devices::initialise_devices();
}

#[command]
pub async fn get_categories() -> HashMap<std::string::String, Vec<Action>> {
	CATEGORIES.read().await.clone()
}

#[command]
pub async fn get_localisations(locale: &str) -> Result<HashMap<String, serde_json::Value>, Error> {
	let mut localisations: HashMap<String, serde_json::Value> = HashMap::new();

	let mut entries = match tokio::fs::read_dir(&crate::shared::config_dir().join("plugins")).await {
		Ok(entries) => entries,
		Err(error) => return Err(anyhow::Error::from(error).into()),
	};

	while let Ok(Some(entry)) = entries.next_entry().await {
		let path = match entry.metadata().await.unwrap().is_symlink() {
			true => tokio::fs::read_link(entry.path()).await.unwrap(),
			false => entry.path(),
		};
		let metadata = tokio::fs::metadata(&path).await.unwrap();
		if metadata.is_dir() {
			let Ok(locale) = tokio::fs::read(path.join(format!("{locale}.json"))).await else { continue };
			let Ok(locale): Result<serde_json::Value, _> = serde_json::from_slice(&locale) else {
				continue;
			};
			localisations.insert(path.file_name().unwrap().to_str().unwrap().to_owned(), locale);
		}
	}

	Ok(localisations)
}
