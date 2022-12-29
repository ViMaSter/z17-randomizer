use crate::{FillerItem, LocationInfo};
use crate::logic::Logic;
use crate::progress::Progress;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize)]
pub struct Check {
    name: &'static str,
    logic: Logic,
    quest: Option<FillerItem>,
    location_info: Option<LocationInfo>,
}

impl Check {
    pub fn new(name: &'static str, logic: Logic, quest: Option<FillerItem>, location_info: Option<LocationInfo>) -> Self {
        Self { name, logic, quest, location_info }
    }

    pub fn get_name(self) -> &'static str {
        self.name
    }

    pub fn get_quest(self) -> Option<FillerItem> {
        self.quest
    }

    pub fn get_location_info(self) -> Option<LocationInfo> {
        self.location_info
    }

    pub fn can_access(self, progress: &Progress) -> bool {
        self.logic.can_access(progress)
    }
}