use crate::board::*;
use crate::heuristics::*;
use crate::min_heap::*;
use std::collections::*;
use std::time::Duration;

/// Statistics of the search, used to evaluate the performance of the search algorithms.
/// Feel free to add more fields to this struct if you need them.
pub struct Stats {
    /// Numbers of states expanded during search
    pub expanded: usize,
    /// Total runtime spend in the search.
    ///
    /// ```rust
    /// let start_time: Instant = std::time::Instant::now();
    /// // do something
    /// let runtime: Duration = start_time.elapsed();
    /// ```
    pub runtime: Duration,
}

impl Stats {
    /// Creates a new `Stats` instance with the given expanded states count and runtime.
    pub fn new(expanded: usize, runtime: Duration) -> Stats {
        Stats { expanded, runtime }
    }
}

pub fn search(init_state: Board) -> (Option<Vec<Direction>>, Stats) {
    // record the start time when starting the search (so we can later the time that elapsed since)
    let start = std::time::Instant::now();

    // frontier: MinHeap provide allows to store the states to explore, with associated priority
    let mut heap: MinHeap<Board> = MinHeap::new();

    // the standard library provides a HashMap, that can be used to store the cost and predecessors of each state
    // assocaciates each state on the frontier to the best cost to reach it
    let mut path_costs: HashMap<Board, u32> = HashMap::new();
    // assocaciates each state on the frontier to the its best parent state and the action to it (parent, action)
    let mut predecessors: HashMap<Board, (Board, Direction)> = HashMap::new();

    // keeps track all states that have been expanded
    let mut expanded: HashSet<Board> = HashSet::new();

    let mut cursor;

    heap.insert(init_state, 0);

    let mut i = 0;
    loop {
        i += 1;

        // if heap empty, return false TODO
        // No error handling so far for invalid results.
        if heap.is_empty() {
            println!("IS EMPTY!");
        }

        // Extract exploring
        let exploring_unwrapped = heap.pop().unwrap();
        
        // Checking hasn't been expanded. REDUNDANT?
        //if expanded.contains(&exploring_unwrapped) {continue;}

        // Add currently exploring to list of explored
        expanded.insert(exploring_unwrapped);

        // If we found goal, stop
        if exploring_unwrapped == Board::GOAL {
            println!("Found result!");
            println!("{exploring_unwrapped}");
            cursor = exploring_unwrapped.clone();
            break;
        }

        for direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            if let Some(temp_board) = exploring_unwrapped.apply(direction) {
                
                // If already explored, ignore
                if expanded.contains(&temp_board) {continue;}
                
                // Add elements to explore into the heaps
                heap.insert(temp_board, i);
                path_costs.insert(temp_board, i);
                predecessors.insert(temp_board, (exploring_unwrapped, direction));
            }
        }
    }

    // Recreate list of directions necessary for result
    let mut result = Vec::new();
    loop {
        
        // Print current step (for testing)
        //println!("{cursor}");

        if cursor == init_state {break;}
        
        // Get predecessor and move
        let pred = predecessors.get(&cursor).unwrap();
        result.push(pred.1);

        // Move cursor up
        cursor = pred.0;
    }

    // Reverse result for correct order
    result.reverse();


    // here is an example to measure the runtime and returns the statistics
    let runtime = start.elapsed();
    // example to construct a Stats instance
    let stats = Stats::new(0, runtime);
    // return the results and associated stats
    (Some(result), stats)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_search() {
        use super::*;

        // validates that search does return the optimal plan on the first 20 isntances

        for (expected_cost, init) in &INSTANCES[0..20] {
            let (path, stats) = search(*init);
            let path = path.expect("no plan");
            assert!(init.is_valid_plan(&path));
            assert_eq!(path.len(), *expected_cost as usize);
        }
    }
}
