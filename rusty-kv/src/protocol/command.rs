// Represents a command sent by a client to the key-value store.
// Each client request is parsed into one of these commands.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Set {
        key: String,
        value: String,
    },
    Get {
        key: String,
    },
    Delete {
        key: String,
    },
    Exists {
        key: String,
    },
}