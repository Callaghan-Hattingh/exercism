use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut vector = Vec::new();
    for num in factors {
        for multiple in 1..=limit {
            let sum = num * multiple;
            if sum < limit {
                vector.push(sum)
            } else if sum > limit {
                break;
            }
        }
    }
    let unique_vector: Vec<_> = vector
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let sum: u32 = unique_vector.iter().sum();
    sum
}
