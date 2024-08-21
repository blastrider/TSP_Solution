use std::collections::HashMap;
use std::cmp::min;

fn get_key(subset: u64, end: usize) -> String {
    format!("{:064b}:{}", subset, end)
}

fn tsp_bellman_held_karp(distances: &Vec<Vec<i64>>, n: usize) -> (i64, Vec<usize>) {
    let mut dp: HashMap<String, i64> = HashMap::new();
    let mut parent: HashMap<String, usize> = HashMap::new();

    for i in 1..n {
        let key = get_key(1 << i, i);
        dp.insert(key.clone(), distances[0][i]);
        parent.insert(key.clone(), 0);
        println!("Initialisation: Clé = {}, Distance = {}, Parent = 0", key, distances[0][i]);
    }

    for r in 2..n {
        for subset in (1..(1 << n)).filter(|&s| (s as u64).count_ones() as usize == r) {
            for next in 1..n {
                if subset & (1 << next) == 0 {
                    continue;
                }

                let mut min_dist = i64::MAX;
                let mut best_end = None;

                for end in 1..n {
                    if end == next || subset & (1 << end) == 0 {
                        continue;
                    }

                    let prev_subset = subset ^ (1 << next);
                    let key = get_key(prev_subset as u64, end);
                    if let Some(&prev_dist) = dp.get(&key) {
                        let current_dist = prev_dist.saturating_add(distances[end][next]);
                        if current_dist < min_dist {
                            min_dist = current_dist;
                            best_end = Some(end);
                        }
                    }
                }

                if let Some(end) = best_end {
                    let key = get_key(subset as u64, next);
                    dp.insert(key.clone(), min_dist);
                    parent.insert(key.clone(), end);
                    println!("DP: Clé = {}, Distance = {}, Parent = {}", key, min_dist, end);
                }
            }
        }
    }

    let mut min_dist = i64::MAX;
    let mut last_index = 0;

    for i in 1..n {
        let key = get_key((1 << n) - 2, i);
        if let Some(&dist) = dp.get(&key) {
            let total_dist = dist.saturating_add(distances[i][0]);
            if total_dist < min_dist {
                min_dist = total_dist;
                last_index = i;
            }
        }
    }

    println!("Chemin trouvé avec coût minimum : {}, dernière ville : {}", min_dist, last_index);

    let mut path = Vec::new();
    let mut subset = (1 << n) - 2;
    path.push(last_index);

    for _ in 0..n - 2 {
        let key = get_key(subset as u64, last_index);
        if let Some(&parent_index) = parent.get(&key) {
            println!("Reconstruction: Clé = {}, Parent trouvé = {}", key, parent_index);
            last_index = parent_index;
            path.push(last_index);
            subset ^= 1 << last_index;
        } else {
            panic!("Erreur lors de la reconstruction du chemin. Valeur manquante pour clé: {}", key);
        }
    }

    path.push(0);
    path.reverse();

    (min_dist, path)
}

fn main() {
    let distances = vec![
        vec![0, 10, 15, 20],
        vec![10, 0, 35, 25],
        vec![15, 35, 0, 30],
        vec![20, 25, 30, 0],
    ];

    let n = distances.len();
    let (min_dist, path) = tsp_bellman_held_karp(&distances, n);

    println!("Distance minimale: {}", min_dist);
    println!("Chemin optimal: {:?}", path);
}
