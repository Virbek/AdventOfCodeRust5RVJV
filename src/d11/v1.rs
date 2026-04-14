use std::collections::HashMap;
use std::fs;


pub fn solve_part1(input: &str) -> u32{
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let line = line.trim(); // enlève \r et espaces
        if line.is_empty() {
            continue;
        }
        let (src, rest) = line.split_once(": ").unwrap();
        let voisins: Vec<String> = rest
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        graph.insert(src.to_string(), voisins);
    }

    let mut sum = 0;
    for i in graph["you"].iter(){
        sum += count_part1(i, &graph);
    }
    
    sum
}

pub fn count_part1(s: &String, graph: &HashMap<String, Vec<String>>) -> u32{

    let mut sum = 0;
    for i in graph[s].iter(){
        if i == "out"{
            sum = 1;
        }else{
            sum += count_part1(i, &graph);
        }
    }

    return sum;

}

pub fn solve_part2(input: &str) -> u64{
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let line = line.trim(); // enlève \r et espaces
        if line.is_empty() {
            continue;
        }
        let (src, rest) = line.split_once(": ").unwrap();
        let voisins: Vec<String> = rest
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        graph.insert(src.to_string(), voisins);
    }

    let mut cache: HashMap<(String, bool, bool), u64> = HashMap::new();
    let sum = count_part2(&"svr".to_string(), &graph, false, false, &mut cache);
    
    sum
}

pub fn count_part2(s: &String, graph: &HashMap<String, Vec<String>>,dac: bool,fft: bool,cache: &mut HashMap<(String, bool, bool), u64> ) -> u64{

    let dac = dac || s == "dac";
    let fft = fft || s == "fft";

    if s == "out" {
        return if dac && fft { 1 } else { 0 };
    }

    let key = (s.clone(), dac, fft);
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