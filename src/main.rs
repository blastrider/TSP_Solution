use std::collections::HashMap;

fn tsp_bellman_held_karp(distances: Vec<Vec<i64>>) -> (i64, Vec<usize>) {
    let n = distances.len();

    let mut dp: HashMap<(Vec<usize>, usize), (i64, Vec<usize>)> = HashMap::new();

    fn dp_rec(
        nodes: Vec<usize>,
        distances: &Vec<Vec<i64>>,
        dp: &mut HashMap<(Vec<usize>, usize), (i64, Vec<usize>)>,
    ) -> (i64, Vec<usize>) {
        if nodes.len() == 2 {
            return (distances[nodes[0]][nodes[1]], vec![nodes[0]]);
        }

        let end = *nodes.last().unwrap();
        let end_idx = nodes.iter().position(|&x| x == end).unwrap();
        let mut nodes_wo_end = nodes[..end_idx].to_vec();
        nodes_wo_end.extend_from_slice(&nodes[end_idx + 1..nodes.len() - 1]);

        let mut min_dist: Option<(i64, Vec<usize>)> = None;

        for &node in &nodes_wo_end {
            let mut nodes_to_use = nodes_wo_end.clone();
            nodes_to_use.push(node);

            let (cost, mut path) = dp_rec(nodes_to_use, distances, dp);
            let candidate = (cost + distances[node][end], {
                path.push(end);
                path
            });

            if min_dist.is_none() || candidate.0 < min_dist.as_ref().unwrap().0 {
                min_dist = Some(candidate);
            }
        }

        let result = min_dist.unwrap();
        dp.insert((nodes.clone(), end), result.clone());
        result
    }

    let mut min_result: Option<(i64, Vec<usize>)> = None;

    for start_node in 0..n {
        let nodes: Vec<usize> = (0..n).chain(std::iter::once(start_node)).collect();
        let (cost, mut path) = dp_rec(nodes, &distances, &mut dp);
        let candidate = (cost + distances[start_node][0], {
            path.push(0);
            path
        });

        if min_result.is_none() || candidate.0 < min_result.as_ref().unwrap().0 {
            min_result = Some(candidate);
        }
    }

    let (min_cost, min_path) = min_result.unwrap();
    (min_cost, min_path)
}

fn main() {
    // Distances en kilomètres entre Paris, Thoiry, Versailles et Trappes
    let distances = vec![
        vec![0, 46, 18, 28],  // Paris
        vec![46, 0, 29, 22],  // Thoiry
        vec![18, 29, 0, 8],   // Versailles
        vec![28, 22, 8, 0],   // Trappes
    ];

    let villes = vec!["Paris", "Thoiry", "Versailles", "Trappes"];
    
    let (cost, path) = tsp_bellman_held_karp(distances);
    println!("Le coût minimum du TSP est : {} km", cost);
    println!("L'ordre des villes visitées est : {:?}", path.iter().map(|&i| villes[i]).collect::<Vec<_>>());
}
