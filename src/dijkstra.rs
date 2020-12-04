pub fn dijkstra(adjacency_matrix: Vec<Vec<i64>>, start: usize) -> Vec<i64> {
    let n = adjacency_matrix.len();

    let mut dist = vec![std::i64::MAX; n];
    let mut min_weight_heap = std::collections::BinaryHeap::new();
    dist[start] = 0;
    min_weight_heap.push(std::cmp::Reverse((dist[start], start)));

    while let Some(std::cmp::Reverse((weight, i))) = min_weight_heap.pop() {
        if dist[i] < weight {
            continue;
        }
        dist[i] = weight;

        for j in 0..n {
            let add = adjacency_matrix[i][j];
            if add != std::i64::MAX && dist[i] + add < dist[j] {
                dist[j] = dist[i] + add;
                min_weight_heap.push(std::cmp::Reverse((dist[j], j)));
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let adjacency_matrix = build_adjacency_matrix(
            7,
            vec![
                (0, 1, 3),
                (0, 2, 1),
                (0, 3, 9),
                (0, 4, 11),
                (1, 2, 1),
                (1, 5, 4),
                (2, 4, 2),
                (2, 5, 8),
                (2, 6, 7),
                (3, 4, 2),
                (4, 6, 3),
                (5, 6, 1),
            ],
        );

        assert_eq!(vec![0, 2, 1, 5, 3, 6, 6], dijkstra(adjacency_matrix, 0));
    }

    #[test]
    fn test_2() {
        let adjacency_matrix = build_adjacency_matrix(
            5,
            vec![
                (0, 1, 4),
                (0, 2, 2),
                (1, 2, 1),
                (1, 3, 2),
                (2, 3, 6),
                (2, 4, 8),
                (3, 4, 2),
            ],
        );

        assert_eq!(vec![0, 3, 2, 5, 7], dijkstra(adjacency_matrix, 0));
    }

    #[test]
    fn test_3() {
        let mut graph = vec![];
        for i in 0..10000 {
            graph.push((i, i + 1, 1));
        }
        let adjacency_matrix = build_adjacency_matrix(10001, graph);
        assert_eq!(10000, dijkstra(adjacency_matrix, 0)[10000]);
    }

    fn build_adjacency_matrix(n: usize, graph: Vec<(usize, usize, i64)>) -> Vec<Vec<i64>> {
        let mut adjacency_matrix = vec![vec![std::i64::MAX; n]; n];
        for &(from, to, weight) in graph.iter() {
            adjacency_matrix[from][to] = weight;
            adjacency_matrix[to][from] = weight;
        }
        adjacency_matrix
    }
}
