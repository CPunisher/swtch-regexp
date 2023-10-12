use petgraph::{graph::NodeIndex, Direction::Outgoing, Graph};

type State = NodeIndex<u32>;
type NodeLabel = String;
type EdgeLabel = Option<char>;

fn merge_graph<N: Clone, E: Clone>(g1: &Graph<N, E>, g2: &Graph<N, E>) -> Graph<N, E> {
    let mut graph = Graph::new();
    let node_mapping_1: Vec<_> = g1
        .node_indices()
        .map(|node_index| {
            let node_weight = g1.node_weight(node_index).unwrap().clone();
            graph.add_node(node_weight.clone())
        })
        .collect();
    let node_mapping_2: Vec<_> = g2
        .node_indices()
        .map(|node_index| {
            let node_weight = g2.node_weight(node_index).unwrap().clone();
            graph.add_node(node_weight.clone())
        })
        .collect();

    for edge in g1.raw_edges() {
        let source = node_mapping_1[edge.source().index()];
        let target = node_mapping_1[edge.target().index()];
        graph.add_edge(source, target, edge.weight.clone());
    }

    for edge in g2.raw_edges() {
        let source = node_mapping_2[edge.source().index()];
        let target = node_mapping_2[edge.target().index()];
        graph.add_edge(source, target, edge.weight.clone());
    }
    graph
}

pub struct Nfa {
    graph: Graph<NodeLabel, EdgeLabel>,
    initial_state: State,
}

impl Nfa {
    fn accepted_states(&self) -> Vec<State> {
        self.graph
            .node_indices()
            .filter(|&i| self.graph.edges_directed(i, Outgoing).count() == 0)
            .collect()
    }
}

impl Nfa {
    pub fn literal_character(c: char) -> Self {
        let mut nfa = Self::default();
        let state = nfa.graph.add_node("".to_string());
        nfa.graph.add_edge(nfa.initial_state, state, Some(c));
        nfa
    }

    // pub fn catenation(self, rhs: Nfa) -> Self {
    //     let mut nfa = Self::merge(&self, &rhs);
    //     nfa.graph
    //         .add_edge(nfa.initial_state, self.initial_state, None);
    //     for sink in self.accepted_states() {
    //         nfa.graph.add_edge(sink, rhs.initial_state, None);
    //     }
    //     nfa
    // }

    // pub fn alternation(self, rhs: Nfa) -> Self {}

    // pub fn zero_or_one(self) -> Self {}

    // pub fn zero_or_more(self) -> Self {}

    // pub fn one_or_more(self) -> Self {}
}

impl Default for Nfa {
    fn default() -> Self {
        let mut graph = Graph::new();
        let initial_state = graph.add_node("".to_string());
        Self {
            graph,
            initial_state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_nfa() {
        let nfa = Nfa::default();
        assert_eq!(nfa.graph.node_count(), 1);
        assert_eq!(nfa.graph.edge_count(), 0);
    }

    #[test]
    fn test_literal_charater() {
        let nfa = Nfa::literal_character('a');
        assert_eq!(nfa.graph.node_count(), 2);
        assert_eq!(nfa.graph.edge_count(), 1);
        assert_eq!(
            nfa.graph.edges(nfa.initial_state).next().unwrap().weight(),
            &Some('a')
        );
    }

    #[test]
    fn test_merge() {
        let g1 = Nfa::literal_character('a').graph;
        let g2 = Nfa::literal_character('b').graph;
        let graph = merge_graph(&g1, &g2);
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn test_from_str() {}
}
