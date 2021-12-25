use std::collections::{HashMap, HashSet};

fn is_lowercase(s: &str) -> bool {
    s.chars().all(|x| x.is_lowercase())
}

fn find_paths<'a>(
    nodes: &HashMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
    cave: &'a str,
    mut can_visit_twice: bool,
) -> usize {
    if cave == "end" {
        return 1;
    }
    let mut should_remove = true;
    if is_lowercase(cave) && !visited.insert(cave) {
        if cave != "start" && can_visit_twice {
            can_visit_twice = false;
            should_remove = false;
        } else {
            return 0;
        }
    }
    let edges = if let Some(list) = nodes.get(cave) {
        list
    } else {
        return 0;
    };
    let paths = edges
        .iter()
        .map(|e| find_paths(&nodes, visited, e, can_visit_twice))
        .sum::<usize>();
    if should_remove == true {
        visited.remove(cave);
    }
    paths
}

pub fn run() {
    let input = include_str!("../inputs/day12.txt");
    let edges = input.lines().filter_map(|line| line.split_once('-'));
    let mut nodes = HashMap::new();
    for (a, b) in edges {
        {
            let node = nodes.entry(a).or_insert(Vec::new());
            node.push(b);
        }
        {
            let node = nodes.entry(b).or_insert(Vec::new());
            node.push(a);
        }
    }
    let mut visited = HashSet::new();
    let paths = find_paths(&nodes, &mut visited, "start", false);
    println!("Day 12 part 1: {paths}");

    let paths2 = find_paths(&nodes, &mut visited, "start", true);
    println!("Day 12 part 2: {paths2}");
}
