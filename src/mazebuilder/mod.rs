extern crate rand;
mod dfs;
mod kruskal;
mod maze;
mod dfs_imperfect;

use std::collections::HashSet;

pub enum Generator {
    DFS { height: u32, width: u32 },
    Kruskal { height: u32, width: u32 },
    DfsImperfect { height: u32, width: u32},
}

pub fn generate_maze(gen: Generator) -> HashSet<(u32, u32)> {
    let maze = match gen {
        Generator::DFS { height, width } => dfs::recursive_backtracker(height, width),
        Generator::Kruskal { height, width } => kruskal::kruskal(height, width),
        Generator::DfsImperfect { height, width} => dfs_imperfect::generate(height, width),
    };

    maze
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_maze_selector() {
        let dfs_test = Generator::DFS {
            height: 10,
            width: 10,
        };
        let kruskal_test = Generator::Kruskal {
            height: 10,
            width: 10,
        };

        let result1 = generate_maze(dfs_test);
        let result2 = generate_maze(kruskal_test);

        assert_eq!(result1.len(), result2.len());
    }

    #[test]
    fn test_dfs_10_10() {
        let test1 = dfs::recursive_backtracker(10, 10);
        assert_eq!(test1.len(), 49);
    }

    #[test]
    fn test_dfs_15_23() {
        let test2 = dfs::recursive_backtracker(15, 23);
        assert_eq!(test2.len(), 153);
    }

    #[test]
    fn test_dfs_100_100() {
        let test3 = dfs::recursive_backtracker(100, 100);
        assert_eq!(test3.len(), 4999);
    }

    #[test]
    fn test_kruskal_10_10() {
        let test1 = kruskal::kruskal(10, 10);
        assert_eq!(test1.len(), 49);
    }

    #[test]
    fn test_kruskal_15_23() {
        let test2 = kruskal::kruskal(15, 23);
        assert_eq!(test2.len(), 153);
    }

    #[test]
    fn test_kruskal_100_100() {
        let test3 = kruskal::kruskal(100, 100);
        assert_eq!(test3.len(), 4999);
    }

    // #[test]
    // fn test_dfs_imperfect_10_10() {
    //     let test1 = dfs_imperfect::generate(10, 10);
    //     assert_eq!(test1.len(), 49);
    // }

    // #[test]
    // fn test_dfs_imperfect_15_23() {
    //     let test2 = dfs_imperfect::generate(15, 23);
    //     assert_eq!(test2.len(), 153);
    // }

    // #[test]
    // fn test_dfs_imperfect_100_100() {
    //     let test3 = dfs_imperfect::generate(100, 100);
    //     assert_eq!(test3.len(), 4999);
    // }
}
