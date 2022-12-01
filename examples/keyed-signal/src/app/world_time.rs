use gloo_net::http;
use gloo_timers::future::TimeoutFuture;
use leptos::Memo;
use leptos_router::ParamsMap;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

///See: http://worldtimeapi.org
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldTime {
    pub timezone: String,
    pub utc_offset: String,
}

impl Display for WorldTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: UTC{}", self.timezone, self.utc_offset,)
    }
}

impl WorldTime {
    pub async fn fetch(key: &WorldTimeParams) -> Result<WorldTime, String> {
        TimeoutFuture::new(300).await;

        let url = format!(
            "http://worldtimeapi.org/api/timezone/{}/{}",
            key.area, key.location
        );
        http::Request::get(&url)
            .send()
            .await
            .map_err(|e| format!("Could not load: {e}"))?
            .json::<WorldTime>()
            .await
            .map_err(|e| format!("Invalid json: {e}"))
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldTimeParams {
    pub area: String,
    pub location: String,
}

impl WorldTimeParams {
    #[allow(dead_code)]
    pub fn new(params: Memo<ParamsMap>) -> Self {
        let params = params.get();
        match (params.get("area"), params.get("location")) {
            (Some(area), Some(location)) => WorldTimeParams {
                area: area.to_owned(),
                location: location.to_owned(),
            },
            _ => WorldTimeParams::default(),
        }
    }
}

impl Display for WorldTimeParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.area, self.location)
    }
}
