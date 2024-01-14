use petgraph::{visit::EdgeRef, Direction::Outgoing, Graph, Undirected};
use std::{cmp, collections::HashMap, collections::HashSet, fs};

type graph_t = Graph<isize, isize, Undirected>;
type node_t = petgraph::prelude::NodeIndex;

// Implement stoer_wagner
// Return (cut of the phase, last vertex group size)
fn min_cut_phase(graph: &mut graph_t) -> (isize, isize) {
    let mut partition = HashSet::<node_t>::new();
    let first_node = graph.node_indices().nth(0).unwrap();
    partition.insert(first_node);

    let mut cut_of_the_phase: Option<isize> = None;
    let mut last_vertex: Option<node_t> = None;
    let mut second_last_vertex: Option<node_t> = Some(first_node); // for the case where |V| == 2

    while partition.len() != graph.node_count() {
        let mut tightest_vertex: Option<node_t> = None;
        let mut largest_weight_sum: Option<isize> = None;

        for node in graph.node_indices().filter(|n| !partition.contains(n)) {
            let mut weight_sum = 0isize;
            for edge in graph.edges_directed(node, Outgoing) {
                if partition.contains(&edge.target()) {
                    weight_sum += edge.weight();
                }
            }
            if largest_weight_sum.is_none() {
                largest_weight_sum = Some(weight_sum);
                tightest_vertex = Some(node);
            } else if weight_sum > largest_weight_sum.unwrap() {
                largest_weight_sum = Some(weight_sum);
                tightest_vertex = Some(node);
            }
        }

        let tightest_vertex = tightest_vertex.unwrap();
        partition.insert(tightest_vertex);
        if partition.len() == graph.node_count() {
            last_vertex = Some(tightest_vertex);
            // Calculate cut of the phase
            cut_of_the_phase = Some(graph.edges(tightest_vertex).map(|e| e.weight()).sum());
        } else if partition.len() == graph.node_count() - 1 {
            second_last_vertex = Some(tightest_vertex);
        }
    }

    let last_vertex = last_vertex.unwrap();
    let second_last_vertex = second_last_vertex.unwrap();
    // Merge last two nodes
    let merged_vertex = graph.add_node(
        graph.node_weight(last_vertex).unwrap() + graph.node_weight(second_last_vertex).unwrap(),
    );
    let mut merged_vertex_target_to_weight = HashMap::<node_t, isize>::new();
    for edge in graph
        .edges_directed(last_vertex, Outgoing)
        .filter(|e| e.target() != second_last_vertex)
    {
        merged_vertex_target_to_weight.insert(edge.target(), *edge.weight());
    }
    for edge in graph
        .edges_directed(second_last_vertex, Outgoing)
        .filter(|e| e.target() != last_vertex)
    {
        merged_vertex_target_to_weight
            .entry(edge.target())
            .and_modify(|v| *v += *edge.weight())
            .or_insert(*edge.weight());
    }
    for (target, weight) in merged_vertex_target_to_weight.into_iter() {
        graph.add_edge(merged_vertex, target, weight);
    }

    let last_vertex_groupsize = *graph.node_weight(last_vertex).unwrap();

    graph.remove_node(second_last_vertex);
    graph.remove_node(last_vertex);

    // println!("{}: {}", cut_of_the_phase.unwrap(), last_vertex_groupsize);
    (cut_of_the_phase.unwrap(), last_vertex_groupsize)
}

// Returns (min cut, group product)
fn min_cut(graph: &mut graph_t) -> (isize, isize) {
    let graph_size = graph.node_count() as isize;
    let mut group_product: Option<isize> = None;
    let mut min_cut: Option<isize> = None;
    while graph.node_count() > 1 {
        println!("Status: {}", graph.node_count());
        let (cut_of_the_phase, last_vertex_group_size) = min_cut_phase(graph);
        if min_cut.is_none() {
            min_cut = Some(cut_of_the_phase);
            group_product = Some(last_vertex_group_size * (graph_size - last_vertex_group_size));
        } else {
            if cut_of_the_phase < min_cut.unwrap() {
                min_cut = Some(cut_of_the_phase);
                group_product =
                    Some(last_vertex_group_size * (graph_size - last_vertex_group_size));
            }
        }
    }

    (min_cut.unwrap(), group_product.unwrap())
}

fn main() {
    let mut graph: graph_t = Graph::new_undirected();

    let input =
        fs::read_to_string("/home/kali/projects/aoc/rust23/day25/src/testcase.txt").unwrap();
    let mut name_to_node = HashMap::<String, node_t>::new();
    input.split('\n').for_each(|line| {
        let (node, adjs) = line.split_once(": ").unwrap();
        name_to_node
            .entry(node.to_string())
            .or_insert_with(|| graph.add_node(1));
        let adjs = adjs.split(' ').collect::<Vec<_>>();

        for &adj in adjs.iter() {
            name_to_node
                .entry(adj.to_string())
                .or_insert_with(|| graph.add_node(1));
            let node = name_to_node.get(node).unwrap();
            let adj_node = name_to_node.get(adj).unwrap();
            graph.add_edge(*node, *adj_node, 1);
        }
    });

    let (mincut, group_product) = min_cut(&mut graph);
    println!("min cut: {}\nans: {}", mincut, group_product);
}

// Runs in just under 2mins with --release
