/*
--- Part Two ---
The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel follow the instructions, but you've barely left your starting position. It's going to take significantly more steps to escape!

What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the laws of spacetime? Only one way to find out.

After examining the maps a bit longer, your attention is drawn to a curious fact: the number of nodes with names ending in A is equal to the number ending in Z! If you were a ghost, you'd probably just start at every node that ends with A and follow all of the paths at the same time until they all simultaneously end up at nodes that end with Z.

For example:

LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
Here, there are two starting nodes, 11A and 22A (because they both end with A). As you follow each left/right instruction, use that instruction to simultaneously navigate away from both nodes you're currently on. Repeat this process until all of the nodes you're currently on end with Z. (If only some of the nodes you're on end with Z, they act like any other node and you continue as normal.) In this example, you would proceed as follows:

Step 0: You are at 11A and 22A.
Step 1: You choose all of the left paths, leading you to 11B and 22B.
Step 2: You choose all of the right paths, leading you to 11Z and 22C.
Step 3: You choose all of the left paths, leading you to 11B and 22Z.
Step 4: You choose all of the right paths, leading you to 11Z and 22B.
Step 5: You choose all of the left paths, leading you to 11B and 22C.
Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
So, in this example, you end up entirely on nodes that end in Z after 6 steps.

Simultaneously start on every node that ends with A. How many steps does it take before you're only on nodes that end with Z?
*/

use std::collections::HashMap;

use regex::Regex;

fn parse_input(input: &str) -> (String, HashMap<String, (String, String)>) {
    let lines: Vec<&str> = input.lines().collect();
    let instructions = lines[0].to_string();
    let mut nodes = HashMap::new();

    let regex = Regex::new(r"\w{3}").unwrap();
    for line in lines.iter().skip(2) {
        let captures = regex.find_iter(line);
        let vec: Vec<&str> = captures.map(|m| m.as_str()).collect();
        nodes.insert(vec[0].to_string(), (vec[1].to_string(), vec[2].to_string()));
    }

    (instructions, nodes)
}

pub fn run(input: &str) -> i64 {
    let (instructions, nodes) = parse_input(input);
    get_steps(&nodes, &instructions, "..A", "..Z")
}

fn get_steps(
    nodes: &HashMap<String, (String, String)>,
    instructions: &str,
    from: &str,
    to: &str,
) -> i64 {
    let current = filter_nodes(nodes, from);
    let steps = process_instructions(nodes, instructions, &current, to);
    lcm_list(&steps)
}

fn filter_nodes(nodes: &HashMap<String, (String, String)>, pattern: &str) -> Vec<String> {
    let re = Regex::new(pattern).unwrap();
    nodes
        .keys()
        .filter(|node| re.is_match(node))
        .cloned()
        .collect()
}

fn process_instructions(
    nodes: &HashMap<String, (String, String)>,
    instructions: &str,
    current: &[String],
    to: &str,
) -> Vec<i64> {
    let re_to = Regex::new(to).unwrap();
    let mut steps: Vec<i64> = vec![0; current.len()];

    for (idx, node) in current.iter().enumerate() {
        let mut node = node;
        while !re_to.is_match(node) {
            let instruction = instructions
                .chars()
                .nth((steps[idx] % (instructions.len() as i64)) as usize)
                .unwrap();
            node = match instruction {
                'L' => &nodes[node].0,
                'R' => &nodes[node].1,
                _ => unreachable!(),
            };
            steps[idx] += 1;
        }
    }

    steps
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn lcm_list(list: &[i64]) -> i64 {
    list.iter().copied().reduce(|a, b| lcm(a, b)).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        let input = include_str!("../test_data_2.txt");
        let output = run(input);
        assert_eq!(output, 6);
    }
}
