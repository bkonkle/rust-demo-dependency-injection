use crate::utils;

#[cfg(test)]
use fake::{Dummy, Faker, Rng};
use serde::Deserialize;

#[cfg(test)]
use crate::utils::update::dummy_update;

/// The `CreateInput` input type
#[derive(Clone, Default, Eq, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Dummy))]
pub struct Create {
    /// The Task's title
    pub title: String,

    /// The Task's description
    pub description: Option<String>,
}

/// The `UpdateInput` input type
#[derive(Clone, Default, Eq, PartialEq, Deserialize)]
pub struct Update {
    /// The Task's title
    pub title: utils::Update<String>,

    /// The Task's description
    pub description: utils::Update<String>,
}

#[cfg(test)]
impl Dummy<Faker> for Update {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        Update {
            title: dummy_update(config, rng),
            description: dummy_update(config, rng),
        }
    }
}
