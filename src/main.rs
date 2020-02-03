extern crate uuid;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
struct Arena<'a> {
    nodes: HashMap<&'a String, &'a Node>,
    edges: HashMap<&'a String, &'a Edge<'a>>,
    costs: HashMap<&'a String, &'a u64>,
    processed: Vec<&'a Node>,
}

impl<'a> Arena<'a> {
    fn new() -> Arena<'a> {
        Arena {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            costs: HashMap::new(),
            processed: Vec::new(),
        }
    }

    fn add_node(mut self, node: &'a Node) -> Arena {
        self.nodes.insert(&node.id, &node);
        self
    }

    fn add_nodes(self, nodes: &'a [&Node]) -> Arena<'a> {
        nodes.iter().fold(self, |acc, node| acc.add_node(node))
    }

    fn add_edge(mut self, edge: &'a Edge) -> Arena<'a> {
        self.edges.insert(&edge.id, &edge);
        self
    }

    fn add_edges(self, edges: &'a [&Edge]) -> Arena<'a> {
        edges.iter().fold(self, |acc, node| acc.add_edge(node))
    }

    fn find_neighbors(&self, node: &Node) -> Vec<&Node> {
        self.edges
            .iter()
            .filter(|(_, edge)| edge.start_node == node)
            .map(|(_, edge)| edge.end_node)
            .collect()
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    id: String,
}

impl Node {
    fn new() -> Node {
        Node {
            id: Uuid::new_v4().to_string(),
        }
    }
}

// impl FromIterator<Node> for Node {
//     from_iter<I: IntoIterator<Item=Node>>(iter: I) -> Self {
//         let mut c = vec![];
//         iter.
//     }
// }

#[derive(Debug)]
struct Edge<'a> {
    id: String,
    weight: u64,
    end_node: &'a Node,
    start_node: &'a Node,
}

impl<'a> Edge<'a> {
    fn new(weight: u64, start_node: &'a Node, end_node: &'a Node) -> Edge<'a> {
        Edge {
            id: Uuid::new_v4().to_string(),
            weight: weight,
            start_node: start_node,
            end_node: end_node,
        }
    }
}

fn main() {
    let book = Node::new();
    let poster = Node::new();
    let lp = Node::new();
    let drums = Node::new();
    let bassguitar = Node::new();
    let piano = Node::new();
    let start_poster = Edge::new(0, &book, &poster);
    let book_lp = Edge::new(5, &book, &lp);
    let poster_bassguitar = Edge::new(30, &poster, &bassguitar);
    let lp_drums = Edge::new(20, &lp, &drums);
    let bassguitar_piano = Edge::new(20, &bassguitar, &piano);
    let drums_piano = Edge::new(10, &drums, &piano);

    let nodes = [&book, &poster, &lp, &drums, &bassguitar, &piano];
    let edges = [
        &start_poster,
        &book_lp,
        &poster_bassguitar,
        &lp_drums,
        &bassguitar_piano,
        &drums_piano,
    ];

    let arena = Arena::new().add_nodes(&nodes).add_edges(&edges);

    println!("{:?}", arena);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_neighbors_test() {
        let start = Node::new();
        let one = Node::new();
        let two = Node::new();
        let three = Node::new();
        let four = Node::new();
        let not_a_neighbor = Node::new();
        let start_one = Edge::new(0, &start, &one);
        let start_two = Edge::new(0, &start, &two);
        let start_three = Edge::new(0, &start, &three);
        let start_four = Edge::new(0, &start, &four);
        let four_not = Edge::new(0, &four, &not_a_neighbor);
        let nodes = [&one, &two, &three, &four, &not_a_neighbor];
        let edges = [&start_one, &start_two, &start_three, &start_four, &four_not];
        let arena = Arena::new().add_nodes(&nodes).add_edges(&edges);
        let neighbors = arena.find_neighbors(&start);
        assert_eq!(neighbors.len(), 4);
        assert_eq!(arena.find_neighbors(&four).len(), 1);
        assert_eq!(arena.find_neighbors(&two).len(), 0);
    }
}