use std::collections::HashMap;
use std::fs;


pub fn solve_part1(input: &str) -> u32{
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let line = line.trim(); // enlève \r et espaces
        if line.is_empty() {
            continue;
        }
        let (src, rest) = line.split_once(": ").unwrap();
        let voisins: Vec<&str> = rest.split_whitespace().collect();
        graph.insert(src, voisins);
    }

    graph["you"]
        .iter()
        .map(|v| count_part1(v, &graph))
        .sum()
}

fn count_part1(s: &str, graph: &HashMap<&str, Vec<&str>>) -> u32{

    let mut sum = 0;
    for i in graph[s].iter(){
        if *i == "out"{
            sum = 1;
        }else{
            sum += count_part1(i, graph);
        }
    }

    sum

}

pub fn solve_part2(input: &str) -> u64{
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let line = line.trim(); // enlève \r et espaces
        if line.is_empty() {
            continue;
        }
        let (src, rest) = line.split_once(": ").unwrap();
        let voisins: Vec<&str> = rest
            .split_whitespace()
            .collect();
        graph.insert(src, voisins);
    }

    let mut cache: HashMap<(&str, bool, bool), u64> = HashMap::new();
    count_part2("svr", &graph, false, false, &mut cache)
    
    
}

fn count_part2<'a>(s: &'a str, graph: &HashMap<&'a str, Vec<&'a str>>,dac: bool,fft: bool,cache: &mut HashMap<(&'a str, bool, bool), u64> ) -> u64{

    let dac = dac || s == "dac";
    let fft = fft || s == "fft";

    if s == "out" {
        return if dac && fft { 1 } else { 0 };
    }

    let key = (s, dac, fft);
    if let Some(&v) = cache.get(&key) {
        return v;
    }

    let mut sum = 0;
    if let Some(voisins) = graph.get(s) {
        for i in voisins.iter() {
            sum += count_part2(i, graph, dac, fft, cache);
        }
    }
    cache.insert(key, sum);
    sum

}