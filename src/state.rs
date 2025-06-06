use std::sync::Arc;
use std::sync::LazyLock;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::fs::read_to_string;
use std::fs::write;

static STATE_PATH: LazyLock<PathBuf> = LazyLock::new(|| crate::dirs::STATE.join("state.ron"));

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct State {
    pub previous_server: Option<Arc<SocketAddr>>,
}
impl State {
    #[must_use]
    pub fn read() -> Option<Self> {
        ron::from_str(&read_to_string(&*STATE_PATH).ok()?).ok()
    }

    pub fn write(&self) {
        write(&*STATE_PATH, ron::to_string(self).expect("failed to serialise state")).expect("failed to write state to file");
    }
}
