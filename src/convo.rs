

use serde::{Deserialize, Serialize};
use spin_sdk::key_value::Store;

#[derive(Debug, Deserialize, Serialize)]
pub struct Conversation {
    pub id: String,
    pub interactions: Vec<Interaction>,
}

const SYSTEM_INSTRUCTION: &str = r#"
<<SYS>>
You are an AI sidekick to help users with extending their knowledge about geography.
Keep your answers as short as possible.
"#;

impl Conversation {
    pub fn new(id: String) -> Self {
        Conversation {
            id,
            interactions: vec![],
        }
    }

    pub fn get_prompt(&self, input: &str) -> String {
        let mut prompt = String::new();
        prompt.push_str("<s>[INST]");
        prompt.push_str(SYSTEM_INSTRUCTION);
        prompt.push_str("[/INST]</s>\n");
        for interaction in &self.interactions {
            prompt.push_str(&interaction.get_prompt());
        }
        prompt.push_str(format!("<s>[INST]{}[/INST]</s>\n", input).as_str());
        prompt
    }

    pub fn store(&self) -> anyhow::Result<()> {
        let store = Store::open_default()?;
        let _ = store.set_json(&self.id, self);
        Ok(())
    }

    pub fn load(id: &str) -> anyhow::Result<Self> {
        let store = Store::open_default()?;
        match store.get_json::<Conversation>(id)? {
            None => Ok(Conversation::new(id.to_string())),
            Some(c) => Ok(c)
        }
    }

    pub fn add_interaction(&mut self, input: &str, output: &str) {
        self.interactions.push(Interaction {
            input: input.to_string(),
            output: output.to_string(),
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Interaction {
    pub input: String,
    pub output: String,
}

impl Interaction {
    pub fn get_prompt(&self) -> String {
        format!("<s>\n[INST] {} [/INST] {}\n</s>", self.input, self.output)
    }
}