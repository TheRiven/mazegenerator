use std::collections::{HashMap, HashSet};
use std::f32;

type Node = (u32, u32);

pub fn solve(start: Node, end: Node, maze: &HashSet<Node>) -> Option<Vec<&Node>> {
    // Setup
    let start_node = maze.get(&start).expect("Start node not found!");
    let goal_node = maze.get(&end).expect("End node not found!");

    // Set of evaluated nodes
    let mut closed_set: HashSet<&Node> = HashSet::new();

    // Set of known nodes that have not yet been evaluated
    let mut open_set = HashSet::new();
    open_set.insert(start_node);

    // Map of each node and how best to get to it
    let mut came_from: HashMap<&Node, &Node> = HashMap::new();

    // Map of nodes and the cost of getting to them from the start
    let mut g_score = HashMap::new();

    // Map of total costs for each node from the start
    let mut f_score = HashMap::new();

    // Fill the maps with the nodes
    maze.iter().for_each(|node| {
        g_score.insert(node, f32::INFINITY);
        f_score.insert(node, f32::INFINITY);
    });

    // set the start g score to zero
    g_score.insert(start_node, 0f32);

    // work out the total score for the first node by using
    // the g_score + heuristic cost algorithim
    f_score.insert(
        start_node,
        g_score[start_node] + heuristic_cost(start_node, goal_node),
    );

    println!("Finding Path with A*");
    // Start looping through the open set
    while open_set.len() > 0 {
        let current = find_lowest_fcost(&open_set, &f_score).expect("No lowest Scoring node found!");

        if current == goal_node {
            println!("End found, generating path.");
            return Some(construct_path(came_from, current, goal_node));
        }

        open_set.remove(current);
        closed_set.insert(current);

        for neighbour in get_node_neighbours(current, maze) {
            // check if the neighbour has already been looked at
            if closed_set.contains(neighbour) {
                continue;
            }

            // Add it to the list to be looked at if it is not already in there
            if open_set.contains(neighbour) == false {
                open_set.insert(neighbour);
            }

            // work out the distance from start to a neighbour
            let score = g_score[current] + distance_between(*current, *neighbour);
            if score >= g_score[neighbour] {
                continue; // This is not a better path
            }

            // This is the best path for now, update the records
            came_from.insert(neighbour, current);
            g_score.insert(neighbour, score);
            f_score.insert(
                neighbour,
                g_score[neighbour] + heuristic_cost(neighbour, goal_node),
            );
        }
    }

    // if we got here then we did not find a path!
    None
}

fn find_lowest_fcost<'a>(
    nodes: &HashSet<&'a Node>,
    f_costs: &HashMap<&'a Node, f32>,
) -> Option<&'a Node> {
    let mut lowest_cost = f32::INFINITY;
    let mut result = None;

    for node in nodes {
        if f_costs[node] < lowest_cost {
            lowest_cost = f_costs[node];
            result = Some(*node);
        }
    }

    result
}

fn distance_between(_node: Node, _other: Node) -> f32 {
    1f32
}

fn heuristic_cost(node: &Node, goal: &Node) -> f32 {
    let p1 = node.0;
    let p2 = node.1;
    let q1 = goal.0;
    let q2 = goal.1;

    // Pythagorean theorem to work out euclidian distance to the end
    let a = (q1 - p1).pow(2) as f32;
    let b = (q2 - p2).pow(2) as f32;
    let result = (a + b).sqrt();

    result
}

fn construct_path<'a>(
    came_from: HashMap<&'a Node, &'a Node>,
    mut current: &'a Node,
    end_node: &'a Node,
) -> Vec<&'a Node> {
    let mut path = Vec::new();
    path.push(end_node);

    while came_from.contains_key(current) {
        current = came_from[current];
        path.push(current);
    }

    path
}

fn get_node_neighbours<'a>(current: &Node, maze: &'a HashSet<Node>) -> Vec<&'a Node> {
    let mut neighbours = Vec::new();

    if let Some(node) = get_north(current, maze) {
        neighbours.push(node);
    };

    if let Some(node) = get_east(current, maze) {
        neighbours.push(node);
    };

    if let Some(node) = get_south(current, maze) {
        neighbours.push(node);
    };

    if let Some(node) = get_west(current, maze) {
        neighbours.push(node);
    };

    neighbours
}

fn get_north<'a>(node: &Node, maze: &'a HashSet<Node>) -> Option<&'a Node> {
    let node = *node;

    if node.1 > 1 {
        let north = (node.0, node.1 - 1);

        if let Some(node) = maze.get(&north) {
            return Some(node);
        };
    }

    None
}

fn get_east<'a>(node: &Node, maze: &'a HashSet<Node>) -> Option<&'a Node> {
    let node = *node;

    let east = (node.0 + 1, node.1);
    if let Some(node) = maze.get(&east) {
        return Some(node);
    };

    None
}

fn get_south<'a>(node: &Node, maze: &'a HashSet<Node>) -> Option<&'a Node> {
    let node = *node;

    let south = (node.0, node.1 + 1);
    if let Some(node) = maze.get(&south) {
        return Some(node);
    };

    None
}

fn get_west<'a>(node: &Node, maze: &'a HashSet<Node>) -> Option<&'a Node> {
    let node = *node;

    if node.0 > 1 {
        let west = (node.0 - 1, node.1);

        if let Some(node) = maze.get(&west) {
            return Some(node);
        };
    }

    None
}
