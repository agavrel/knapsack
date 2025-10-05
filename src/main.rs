use std::collections::HashMap;
use std::env;
use std::fs;
use rand::seq::SliceRandom;
use rand::rng;

#[derive(Debug, Clone, Copy)]
struct Item {
    weight: i64,
    profit: i64,
}

fn knapsack_randomized(items: Vec<Item>, capacity: i64) -> (i64, Vec<usize>) {
    let n = items.len();
    if n == 0 { return (0, vec![]); }

    let mut indexed_items: Vec<(usize, Item)> = items.iter().copied().enumerate().collect();
    indexed_items.shuffle(&mut rng());
    let wmax = indexed_items.iter().map(|(_, i)| i.weight).max().unwrap();
    let mut prev = HashMap::new();
    prev.insert(0, (0i64, vec![]));

    for i in 1..=n {
        let (orig_idx, item) = indexed_items[i - 1];
        let mu = (i as f64 / n as f64 * capacity as f64) as i64;
        let delta = ((i as f64 * (n as f64).ln()).sqrt() * wmax as f64).ceil() as i64;
        let mut curr = HashMap::new();

        for j in (mu - delta).max(0)..=(mu + delta).min(capacity) {
            let skip = prev.get(&j).map(|(p, idx)| (*p, idx.clone()))
                .unwrap_or((i64::MIN, vec![]));
            let take = if j >= item.weight {
                prev.get(&(j - item.weight)).map(|(p, idx)| {
                    let mut new_idx = idx.clone();
                    new_idx.push(orig_idx);
                    (p + item.profit, new_idx)
                }).unwrap_or((i64::MIN, vec![]))
            } else {
                (i64::MIN, vec![])
            };
            curr.insert(j, if skip.0 >= take.0 { skip } else { take });
        }
        prev = curr;
    }

    prev.into_iter().max_by_key(|(_, (p, _))| *p)
        .map(|(_, v)| v).unwrap_or((0, vec![]))
}

fn parse_input(filename: &str) -> (Vec<Item>, i64) {
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let mut lines = content.lines();

    let capacity = lines.next()
        .and_then(|l| l.trim().parse().ok())
        .expect("Invalid capacity");

    let items = lines.filter_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let w = parts[0].parse().ok()?;
            let p = parts[1].parse().ok()?;
            Some(Item { weight: w, profit: p })
        } else {
            None
        }
    }).collect();

    (items, capacity)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        eprintln!("\nInput format:");
        eprintln!("  Line 1: capacity");
        eprintln!("  Remaining lines: weight profit");
        std::process::exit(1);
    }

    let (items, capacity) = parse_input(&args[1]);
    println!("Items: {}, Capacity: {}", items.len(), capacity);

    let (profit, selected) = knapsack_randomized(items.clone(), capacity);
    println!("Maximum profit: {}", profit);
    println!("Selected items (0-indexed): {:?}", selected);

    let total_weight: i64 = selected.iter().map(|&i| items[i].weight).sum();
    let total_profit: i64 = selected.iter().map(|&i| items[i].profit).sum();
    println!("Total weight: {}, Total profit: {}", total_weight, total_profit);
}