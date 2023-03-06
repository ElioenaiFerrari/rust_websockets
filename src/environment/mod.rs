use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Environment {
    pub database_url: String,
}

pub fn get() -> Environment {
    return match envy::from_env::<Environment>() {
        Ok(environment) => environment,
        Err(error) => panic!("{:#?}", error),
    };
}
