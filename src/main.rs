use std::collections::HashMap;
use std::cmp::min;

fn tsp_bellman_held_karp(distances: Vec<Vec<i64>>) -> i64 {
    let n = distances.len();

    let mut dp: HashMap<(Vec<usize>, usize), i64> = HashMap::new();

    fn dp_rec(
        nodes: Vec<usize>,
        distances: &Vec<Vec<i64>>,
        dp: &mut HashMap<(Vec<usize>, usize), i64>,
    ) -> i64 {
        if nodes.len() == 2 {
            return distances[nodes[0]][nodes[1]];
        }

        let end = *nodes.last().unwrap();
        let end_idx = nodes.iter().position(|&x| x == end).unwrap();
        let mut nodes_wo_end = nodes[..end_idx].to_vec();
        nodes_wo_end.extend_from_slice(&nodes[end_idx + 1..nodes.len() - 1]);

        let mut min_dist: Option<i64> = None;

        for &node in &nodes_wo_end {
            let mut nodes_to_use = nodes_wo_end.clone();
            nodes_to_use.push(node);

            let candidate = dp_rec(nodes_to_use, distances, dp) + distances[node][end];
            if min_dist.is_none() || candidate < min_dist.unwrap() {
                min_dist = Some(candidate);
            }
        }

        let result = min_dist.unwrap();
        dp.insert((nodes.clone(), end), result);
        result
    }

    let mut min_dist: Option<i64> = None;

    for start_node in 0..n {
        let nodes: Vec<usize> = (0..n).chain(std::iter::once(start_node)).collect();
        let candidate = dp_rec(nodes, &distances, &mut dp) + distances[start_node][0];
        if min_dist.is_none() || candidate < min_dist.unwrap() {
            min_dist = Some(candidate);
        }
    }

    min_dist.unwrap()
}

fn main() {
    let distances = vec![
        vec![0, 10, 15, 20],
        vec![10, 0, 35, 25],
        vec![15, 35, 0, 30],
        vec![20, 25, 30, 0],
    ];

    let result = tsp_bellman_held_karp(distances);
    println!("Le coût minimum du TSP est : {}", result);
}
