use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::UnGraph;
use rustworkx_core::Result;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
type RwxResult<T> = Result<Option<(usize, Vec<T>)>, Box<dyn Error>>;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let edges = parse(&input);
    let part_1 = part_1(&edges);
    println!("Part 1: {}", part_1);
}

fn part_1(edges: &Vec<Vec<String>>) -> u64 {
    let mut graph = UnGraph::new_undirected();

    let verts = edges
        .iter()
        .flatten()
        .map(|x| x.as_str())
        .collect::<HashSet<_>>();

    let nodes = verts
        .iter()
        .map(|&s| (s, graph.add_node(s)))
        .collect::<HashMap<_, _>>();

    for adjacent in edges {
        let node = adjacent[0].as_str();

        for adj in adjacent[1..].iter().map(|x| x.as_str()) {
            graph.add_edge(nodes[node], nodes[adj], 1);
        }
    }

    let min_cut: RwxResult<_> = stoer_wagner_min_cut(&graph, |_| Ok(1));

    if let Ok(Some((_, cut))) = min_cut {
        let product = (verts.len() - cut.len()) * cut.len();
        return product as u64;
    }

    return 0;
}

fn parse(input: &str) -> Vec<Vec<String>> {
    let mut edges = Vec::new();
    for line in input.lines() {
        let mut edge = Vec::new();
        for item in line.split(":").collect::<Vec<&str>>() {
            for i in item.split(" ").collect::<Vec<&str>>() {
                if i == "" {
                    continue;
                }
                edge.push(i.to_string());
            }
        }

        edges.push(edge);
    }

    return edges;
}
