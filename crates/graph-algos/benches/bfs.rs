use std::time::Duration;

use bench_util::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion, SamplingMode};
use graph::prelude::*;
use graph_algos::bfs::{bfs_directed, bfs_undirected};

fn bfs(c: &mut Criterion) {
    let graph500_scale = 22;
    let edge_list_file = create_graph_500(graph500_scale).unwrap();
    println!("Downloading graph to {edge_list_file:?} ...");

    let directed_graph: DirectedCsrGraph<usize> = GraphBuilder::new()
        .file_format(EdgeListInput::default())
        .path(edge_list_file)
        .build()
        .unwrap();

    let node_count = directed_graph.node_count().index();
    let edge_count = directed_graph.edge_count().index();
    println!("Loaded graph with {node_count} nodes and {edge_count} edges.");

    let undirected_graph: UndirectedCsrGraph<usize> =
        directed_graph.to_undirected(CsrLayout::default());

    // Start from the node with the highest degree:
    let source_id = (0..node_count)
        .max_by_key(|node_id| {
            let out_degree = directed_graph.out_degree(*node_id);
            let in_degree = directed_graph.in_degree(*node_id);
            out_degree + in_degree
        })
        .unwrap();

    let count = bfs_directed(&directed_graph, [source_id], Direction::Undirected).count();
    println!("bfs from node {source_id} visited {count} nodes",);

    let mut group = c.benchmark_group("bfs");
    group
        .sample_size(10)
        .measurement_time(Duration::from_secs(200))
        .sampling_mode(SamplingMode::Flat);

    group.bench_function("directed", |b| {
        b.iter(|| {
            for id in bfs_directed(&directed_graph, [source_id], Direction::Undirected) {
                black_box(id);
            }
        })
    });

    group.bench_function("undirected", |b| {
        b.iter(|| {
            for id in bfs_undirected(&undirected_graph, [source_id]) {
                black_box(id);
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bfs);
criterion_main!(benches);
