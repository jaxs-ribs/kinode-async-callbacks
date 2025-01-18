use crate::*;

#[derive(Default, Serialize, Deserialize)]
pub struct State {
    #[serde(skip)]
    pub pending_callbacks: HashMap<String, Callback>,

    /// Just a stack with the responses that we fill up as we get them
    /// If this works, I will buy myself a steak 
    pub my_lego_stack: Vec<String>,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("pending_callbacks", &"<callbacks>")
            .field("my_lego_stack", &self.my_lego_stack)
            .finish()
    }
}
