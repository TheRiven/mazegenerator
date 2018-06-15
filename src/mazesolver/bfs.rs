use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn breadth_first_search(
    start: (u32, u32),
    end: (u32, u32),
    maze: &HashSet<(u32, u32)>,
) -> Option<Vec<&(u32, u32)>> {
    // A FIFO Set
    let mut open_set: VecDeque<&(u32, u32)> = VecDeque::new();

    // And empy set to maintain visited nodes
    let mut closed_set: HashSet<&(u32, u32)> = HashSet::new();

    // Hashmap containing meta info for path formation
    let mut meta: HashMap<&(u32, u32), &(u32, u32)> = HashMap::new();

    // Setup
    let root = maze.get(&start).expect("Unable to find starting node in maze!");
    meta.insert(&root, &root);
    open_set.push_back(root);

    // For each node on the current level expand and process, if no children (leaf, then unwind)
    while !open_set.is_empty() {
        let current_node = open_set.pop_front().unwrap();

        // if we found the node we wanted stop and emit a path
        if *current_node == end {
            return Some(construct_path(current_node, meta));
        }

        // For each node get the children (connected nodes)
        for child_node in get_node_neighbours(*current_node, maze) {
            // if the node has already been processed (in the closed set) skip it
            if closed_set.contains(&child_node) {
                continue;
            }

            // if it has not already been queued, add it to the open set
            // and store the "meta data" to access it
            if !open_set.contains(&child_node) {
                // create link in the meta data for these nodes
                meta.insert(&child_node, &current_node);
                // queue up the node for a future pass
                open_set.push_back(child_node);
            }
        }

        // When we have finished processing this root node add it to the closed set
        closed_set.insert(current_node);
    }

    None
}

fn get_node_neighbours(node: (u32, u32), maze: &HashSet<(u32, u32)>) -> Vec<&(u32, u32)> {
    let mut neighbours: Vec<&(u32, u32)> = Vec::new();

    let north = (node.0, node.1 - 1);
    let east = (node.0 + 1, node.1);
    let south = (node.0, node.1 + 1);
    let west = (node.0 - 1, node.1);

    if maze.contains(&north) {
        neighbours.push(maze.get(&north).unwrap());
    }

    if maze.contains(&east) {
        neighbours.push(maze.get(&east).unwrap());
    }

    if maze.contains(&south) {
        neighbours.push(maze.get(&south).unwrap());
    }

    if maze.contains(&west) {
        neighbours.push(maze.get(&west).unwrap());
    }

    neighbours
}

fn construct_path<'a>(
    node: &'a (u32, u32),
    meta: HashMap<&'a (u32, u32), &'a (u32, u32)>,
) -> Vec<&'a (u32, u32)> {
    // list of vectors that make up the path back to the start
    let mut path: Vec<&(u32, u32)> = Vec::new();

    // get the parent of the current node from the meta data Hashmap
    let mut current_node = node;
    let mut parent = meta.get(current_node).unwrap();

    // While the parent is not the current node (indicating that we are at the root/start)
    // add the current node to the path and then move up to the parent
    while **parent != *current_node {
        path.push(current_node);
        current_node = *parent;
        parent = meta.get(current_node).unwrap();
    }

    // add the final node to the path (this should be the start of the maze)
    path.push(current_node);

    path
}
