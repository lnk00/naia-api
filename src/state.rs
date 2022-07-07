use mongodb::Client;

#[derive(Clone)]
pub struct State {
    pub db: Client,
}
