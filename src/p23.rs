use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::util::*;

pub struct CavernGraph {
    adjacencies: HashMap<String, HashSet<String>>,
}

impl CavernGraph {
    pub fn new(adjacencies: Vec<(String, String)>) -> Self {
        let mut adjacency_map: HashMap<String, HashSet<String>> = HashMap::new();

        for (start, end) in adjacencies {
            if !adjacency_map.contains_key(&start) {
                adjacency_map.insert(start.clone(), HashSet::new());
            }
            if !adjacency_map.contains_key(&end) {
                adjacency_map.insert(end.clone(), HashSet::new());
            }

            adjacency_map.get_mut(&start).unwrap().insert(end.clone());
            adjacency_map.get_mut(&end).unwrap().insert(start);
        }

        CavernGraph {
            adjacencies: adjacency_map,
        }
    }

    fn is_small_cavern(cavern: &str) -> bool {
        cavern != "end" && cavern.to_lowercase() == cavern
    }

    pub fn all_paths(&self, allow_twice: bool) -> HashSet<String> {
        let mut paths: Vec<String> = vec![];
        let mut path: Vec<String> = vec!["start".to_owned()];

        self.visit(&mut path, &mut paths, allow_twice);

        paths.into_iter().collect()
    }

    fn counts(path: &Vec<String>) -> HashMap<String, u8> {
        let mut counts: HashMap<String, u8> = HashMap::new();
        for s in path.iter().filter(|c| Self::is_small_cavern(c)) {
            counts.insert(s.clone(), counts.get(s).unwrap_or(&0) + 1);
        }

        counts
    }

    fn is_valid_dest(&self, dest: &str, path: &Vec<String>, allow_twice: bool) -> bool {
        if dest == "start" {
            return false;
        }

        if allow_twice {
            !(Self::is_small_cavern(dest)
                && path.iter().any(|path_component| path_component == dest)
                && Self::counts(path).iter().any(|(_, c)| *c > 1))
        } else {
            !(Self::is_small_cavern(dest)
                && path.iter().any(|path_component| path_component == dest))
        }
    }

    fn visit(&self, path: &mut Vec<String>, paths: &mut Vec<String>, allow_twice: bool) {
        let cavern = &path[path.len() - 1];
        if cavern == "end" {
            paths.push(path.join(","));
            return;
        }

        if allow_twice && Self::is_small_cavern(cavern) {
            let counts = Self::counts(path);
            let appears_twice = counts
                .iter()
                .filter_map(|(s, c)| if *c > 1 { Some(s) } else { None })
                .count();

            if appears_twice > 1 {
                return;
            }
        }

        let neighbors: Vec<&String> = self
            .adjacencies
            .get(cavern)
            .unwrap()
            .iter()
            .filter(|neighbor| self.is_valid_dest(neighbor, path, allow_twice))
            .collect();

        for dest in neighbors {
            path.push(dest.to_owned());
            self.visit(path, paths, allow_twice);
            path.pop();
        }
    }
}

pub fn parse_cavern_graph(file: BufReader<File>) -> CavernGraph {
    let adjacencies: Vec<(String, String)> = file
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut splits = line.split("-");
            (
                splits.next().unwrap().to_owned(),
                splits.next().unwrap().to_owned(),
            )
        })
        .collect();

    CavernGraph::new(adjacencies)
}

pub fn run(args: &[String]) {
    let file = get_file(args, 23).unwrap();
    let cavern_graph = parse_cavern_graph(file);

    let paths = cavern_graph.all_paths(false);
    println!("Answer is {}", paths.len());
}
