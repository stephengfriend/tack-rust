use chrono::NaiveDate;
use reqwest::{Client as ReqwestClient, ClientBuilder, RequestBuilder};
use std::time::Duration;

pub enum Availability {
    AM,
    FULL,
    PM,
}

pub enum EngineManufacturer {
    Mercury,
}

pub struct Location {
    description: String,
    details: Option<String>,
    id: String,
    name: String,
}
pub struct Reservation {
    available: Option<Availability>,
    date: NaiveDate,
    id: String,
    location: Location,
    vessel: Vessel,
}

pub struct Vessel {
    id: String,
    name: String,
    details: VesselDetails,
}

pub struct VesselDetails {
    bimini: bool,
    engine_hp: Option<u32>,
    engine_manufacturer: Option<EngineManufacturer>,
    length: Option<i32>,
    livewell: bool,
    manufacturer: Option<VesselManufacturer>,
    vessel_type: Option<VesselType>,
}

pub enum VesselManufacturer {
    Mercury,
}

pub enum VesselType {
    PONTOON,
    BAY,
}

pub struct Client {
    http_client: ReqwestClient,
    is_logged_in: bool,
}

impl Client {
    pub fn new(username: String, password: String) -> Self {
        Self {
            http_client: ClientBuilder::new()
                .connect_timeout(Duration::new(6, 0))
                .timeout(Duration::new(30, 0))
                .build()
                .unwrap(),
            is_logged_in: false,
        }
    }

    pub fn get(&self, url: String) -> RequestBuilder {
        self.http_client.get(url)
    }

    async fn login(&self) {
        if !self.is_logged_in {}
    }

    async fn club_and_member_id(&self) -> (u32, u32) {
        (53, 73052)
    }

    pub async fn is_logged_in(&self) -> bool {
        self.is_logged_in
    }
}
