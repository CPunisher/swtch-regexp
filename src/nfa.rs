use petgraph::{graph::NodeIndex, Graph};

type State = NodeIndex<u32>;
type NodeLabel = String;
type EdgeLabel = Option<char>;
type NfaGraph = Graph<NodeLabel, EdgeLabel>;

pub struct Nfa {
    graph: NfaGraph,
    initial_state: State,
    accepted_states: Vec<State>,
}

impl Nfa {
    fn merge(
        g1: &NfaGraph,
        g2: &NfaGraph,
    ) -> (NfaGraph, impl Fn(State) -> State, impl Fn(State) -> State) {
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

        let node_mapper_1 = move |node_index: State| node_mapping_1[node_index.index()];
        let node_mapper_2 = move |node_index: State| node_mapping_2[node_index.index()];

        for edge in g1.raw_edges() {
            let source = node_mapper_1(edge.source());
            let target = node_mapper_1(edge.target());
            graph.add_edge(source, target, edge.weight.clone());
        }

        for edge in g2.raw_edges() {
            let source = node_mapper_2(edge.source());
            let target = node_mapper_2(edge.target());
            graph.add_edge(source, target, edge.weight.clone());
        }

        (graph, Box::new(node_mapper_1), Box::new(node_mapper_2))
    }
}

impl Nfa {
    pub fn literal_character(c: char) -> Self {
        let mut graph = NfaGraph::new();
        let initial_state = graph.add_node("".to_string());
        let node = graph.add_node("".to_string());
        graph.add_edge(initial_state, node, Some(c));
        Self {
            graph,
            initial_state,
            accepted_states: vec![node],
        }
    }

    pub fn catenation(self, rhs: Nfa) -> Self {
        let (mut graph, mapper1, mapper2) = Self::merge(&self.graph, &rhs.graph);
        for sink in self.accepted_states {
            graph.add_edge(mapper1(sink), mapper2(rhs.initial_state), None);
        }
        Self {
            graph,
            initial_state: self.initial_state,
            accepted_states: rhs.accepted_states,
        }
    }

    pub fn alternation(self, rhs: Nfa) -> Self {
        let (mut graph, mapper1, mapper2) = Self::merge(&self.graph, &rhs.graph);
        let initial_state = graph.add_node("".to_string());
        graph.add_edge(initial_state, mapper1(self.initial_state), None);
        graph.add_edge(initial_state, mapper2(rhs.initial_state), None);

        let accepted_states = self
            .accepted_states
            .iter()
            .map(|&s| mapper1(s))
            .chain(rhs.accepted_states.iter().map(|&s| mapper2(s)))
            .collect();

        Self {
            graph,
            initial_state,
            accepted_states,
        }
    }

    pub fn zero_or_one(self) -> Self {
        let mut graph = self.graph.clone();
        let initial_state = graph.add_node("".to_string());
        graph.add_edge(initial_state, self.initial_state, None);
        let accepted_states = [vec![initial_state], self.accepted_states].concat();
        Self {
            graph,
            initial_state,
            accepted_states,
        }
    }

    pub fn zero_or_more(self) -> Self {
        let mut graph = self.graph.clone();
        let initial_state = graph.add_node("".to_string());
        graph.add_edge(initial_state, self.initial_state, None);
        for sink in self.accepted_states {
            graph.add_edge(sink, self.initial_state, None);
        }
        let accepted_states = vec![initial_state];
        Self {
            graph,
            initial_state,
            accepted_states,
        }
    }

    pub fn one_or_more(self) -> Self {
        let mut graph = self.graph.clone();
        let new_state = graph.add_node("".to_string());
        for sink in self.accepted_states {
            graph.add_edge(sink, new_state, None);
        }
        graph.add_edge(new_state, self.initial_state, None);
        Self {
            graph,
            initial_state: self.initial_state,
            accepted_states: vec![new_state],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_catenation() {
        let nfa_1 = Nfa::literal_character('a');
        let nfa_2 = Nfa::literal_character('b');
        let nfa = nfa_1.catenation(nfa_2);
        assert_eq!(nfa.graph.node_count(), 4);
        assert_eq!(nfa.graph.edge_count(), 3);
        assert!(nfa
            .graph
            .edge_weights()
            .find(|e| e.filter(|&w| w == 'a').is_some())
            .is_some());
        assert!(nfa
            .graph
            .edge_weights()
            .find(|e| e.filter(|&w| w == 'b').is_some())
            .is_some());
    }

    #[test]
    fn test_alternation() {
        let nfa_1 = Nfa::literal_character('a');
        let nfa_2 = Nfa::literal_character('b');
        let nfa = nfa_1.alternation(nfa_2);
        assert_eq!(nfa.graph.node_count(), 5);
        assert_eq!(nfa.graph.edge_count(), 4);
    }

    #[test]
    fn test_zero_or_one() {
        let nfa = Nfa::literal_character('a').zero_or_one();
        assert_eq!(nfa.graph.node_count(), 3);
        assert_eq!(nfa.graph.edge_count(), 2);
    }

    #[test]
    fn test_zero_or_more() {
        let nfa = Nfa::literal_character('a').zero_or_more();
        assert_eq!(nfa.graph.node_count(), 3);
        assert_eq!(nfa.graph.edge_count(), 3);
    }

    #[test]
    fn test_one_or_more() {
        let nfa = Nfa::literal_character('a').one_or_more();
        assert_eq!(nfa.graph.node_count(), 3);
        assert_eq!(nfa.graph.edge_count(), 3);
    }

    #[test]
    fn test_merge() {
        let g1 = Nfa::literal_character('a').graph;
        let g2 = Nfa::literal_character('b').graph;
        let (graph, _, _) = Nfa::merge(&g1, &g2);
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn test_from_str() {}
}
