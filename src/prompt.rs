use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Prompt {
    pub text: String
}