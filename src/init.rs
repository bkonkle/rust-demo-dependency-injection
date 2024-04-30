use nakago::{config::Config as _, inject};
use nakago_axum::AxumApplication;

use crate::config::{Config, CONFIG};

pub async fn app() -> inject::Result<AxumApplication<Config>> {
    let mut app = AxumApplication::default().with_config_tag(&CONFIG);

    todo!()
}
