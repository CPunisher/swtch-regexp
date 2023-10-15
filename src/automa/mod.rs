use petgraph::{graph::NodeIndex, Graph};

mod dfa;
mod error;
mod nfa;

pub use dfa::Dfa;
pub use nfa::Nfa;

type State = NodeIndex<u32>;
type NodeLabel = String;
type EdgeLabel = Option<char>;
type NfaGraph = Graph<NodeLabel, EdgeLabel>;
