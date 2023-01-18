use crate::alfred::AlfredItem;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SwitcherError {
    #[error("profile `{0}` not found")]
    ProfileNotFound(String),

    #[error("key `{0}` not defined")]
    KeyNotDefined(String),

    #[error("Read `{1}` error: `{0}`")]
    ReadError(std::io::Error, String),

    #[error("parse json error")]
    ParseError(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawSwitcher {
    current_profile: String,
    profiles: HashMap<String, HashMap<String, String>>,
}

#[derive(Clone)]
pub struct Profile {
    name: String,
    is_current: bool,
    key_app_map: HashMap<String, String>,
}

impl Profile {
    fn compare(a: &Profile, b: &Profile) -> Ordering {
        let (a_is_default, b_is_default) =
            (a.name == "default", b.name == "default");

        a.is_current
            .cmp(&b.is_current)
            .reverse()
            .then(a_is_default.cmp(&b_is_default).reverse())
            .then(a.name.cmp(&b.name))
    }
}

impl From<Profile> for AlfredItem {
    fn from(profile: Profile) -> AlfredItem {
        if profile.is_current {
            AlfredItem::new_with_sub(profile.name, "current".to_string())
        } else {
            AlfredItem::new(profile.name)
        }
    }
}

pub struct Switcher {
    file_str: String,
    raw_switcher: RawSwitcher,
    current_profile: String,
    profiles: HashMap<String, Profile>,
}

impl Switcher {
    pub fn from_file(file_str: &str) -> Result<Self, SwitcherError> {
        let file = File::open(file_str).map_err(|source| {
            SwitcherError::ReadError(source, file_str.to_string())
        })?;

        let reader = BufReader::new(&file);

        let config: RawSwitcher = serde_json::from_reader(reader)?;

        let mut profiles: HashMap<String, Profile> = HashMap::new();
        let mut current_profile: String = String::new();

        for p in &config.profiles {
            let name: &str = p.0;
            let profile = Profile {
                name: name.to_string(),
                is_current: config.current_profile == name,
                key_app_map: p.1.to_owned(),
            };

            if profile.is_current {
                current_profile = name.to_string();
            }

            profiles.insert(name.to_string(), profile);
        }

        Ok(Switcher {
            file_str: file_str.to_string(),
            raw_switcher: config,
            current_profile,
            profiles,
        })
    }

    pub fn list_profiles(&self) -> Vec<&Profile> {
        let mut v: Vec<&Profile> = self.profiles.values().collect();
        v.sort_by(|a, b| Profile::compare(a, b));
        v
    }

    pub fn get_detail(
        &self,
        name: &str,
    ) -> Result<HashMap<String, String>, SwitcherError> {
        let mut merged = self
            .profiles
            .get("default")
            .ok_or_else(|| {
                SwitcherError::ProfileNotFound("default".to_string())
            })?
            .key_app_map
            .clone();

        let profile_map = self
            .profiles
            .get(name)
            .ok_or_else(|| SwitcherError::ProfileNotFound(name.to_string()))?
            .key_app_map
            .clone();

        merged.extend(profile_map.into_iter());

        Ok(merged)
    }

    pub fn get_app(&self, key: &str) -> Result<String, SwitcherError> {
        let key_to_app = self.get_detail(&self.current_profile)?;

        return key_to_app
            .get(key)
            .map(|s| s.to_owned())
            .ok_or_else(|| SwitcherError::KeyNotDefined(key.to_string()));
    }

    pub fn change_profile(
        &mut self,
        profile: &str,
    ) -> Result<(), SwitcherError> {
        self.profiles.get(profile).ok_or_else(|| {
            SwitcherError::ProfileNotFound(profile.to_string())
        })?;

        self.raw_switcher.current_profile = profile.to_string();
        self.current_profile = profile.to_string();

        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.file_str)
            .map_err(|source| {
                SwitcherError::ReadError(source, self.file_str.to_owned())
            })?;

        serde_json::to_writer_pretty(file, &self.raw_switcher)?;

        Ok(())
    }
}
