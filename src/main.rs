use std::collections::HashMap;

fn get_key(subset: u64, end: usize) -> String {
    format!("{:064b}:{}", subset, end)
}

fn tsp_bellman_held_karp(distances: &Vec<Vec<i64>>, n: usize) -> (i64, Vec<usize>) {
    let mut dp: HashMap<String, i64> = HashMap::new();
    let mut parent: HashMap<String, usize> = HashMap::new();

    // Initialisation : distance de la ville 0 à chaque autre ville
    for i in 1..n {
        let key = get_key(1 << i, i);
        dp.insert(key.clone(), distances[0][i]);
        parent.insert(key.clone(), 0);
        println!("Initialisation: Clé = {}, Distance = {}, Parent = 0", key, distances[0][i]);
    }

    // Boucle pour remplir la table de programmation dynamique
    for r in 2..n {
        println!("Phase de construction pour r = {}", r);
        for subset in (1..(1 << n)).filter(|&s| (s as u64).count_ones() as usize == r) {
            println!("  Traitement du sous-ensemble: {:064b}", subset);
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
                    } else {
                        println!("    Clé manquante pendant la construction: {}", key);
                    }
                }

                if let Some(end) = best_end {
                    let key = get_key(subset as u64, next);
                    dp.insert(key.clone(), min_dist);
                    parent.insert(key.clone(), end);
                    println!("    DP: Clé = {}, Distance = {}, Parent = {}", key, min_dist, end);
                } else {
                    println!("    Aucun parent trouvé pour next = {}, subset = {:064b}", next, subset);
                }
            }
        }
    }

    // Calcul de la distance minimale en revenant à la ville de départ
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

    // Reconstruction du chemin optimal
    let mut path = Vec::new();
    let mut subset = (1 << n) - 2;
    path.push(last_index);

    while subset != 0 {
        let key = get_key(subset as u64, last_index);
        if let Some(&parent_index) = parent.get(&key) {
            println!("Reconstruction: Clé = {}, Parent trouvé = {}", key, parent_index);
            last_index = parent_index;
            path.push(last_index);
            subset ^= 1 << last_index;
        } else {
            println!("Erreur lors de la reconstruction du chemin. Clé manquante: {}", key);
            break; // On sort de la boucle si on ne trouve pas la clé pour éviter un panic.
        }
    }

    path.push(0);
    path.reverse();

    (min_dist, path)
}

fn main() {
    let distances = vec![
        vec![0, 10, 15],  // Distances de la ville 0 vers les autres villes
        vec![10, 0, 35],  // Distances de la ville 1 vers les autres villes
        vec![15, 35, 0],  // Distances de la ville 2 vers les autres villes
    ];

    let n = distances.len();
    let (min_dist, path) = tsp_bellman_held_karp(&distances, n);

    println!("Distance minimale: {}", min_dist);
    println!("Chemin optimal: {:?}", path);
}
