use crate::common::INF; 

#[derive(Debug)] 
pub(crate) struct TspSolver<'a> {
    graph: &'a Vec<Vec<i32>>,
    n: usize,
    start_node: usize,
    dp: Vec<Vec<i32>>,
    parent: Vec<Vec<usize>>,
}

impl<'a> TspSolver<'a> {    
    pub(crate) fn new(graph: &'a Vec<Vec<i32>>, start_node: usize) -> Self {
        let n = graph.len();
        
        let mut dp = vec![vec![INF; n]; 1 << n];
        let parent = vec![vec![0; n]; 1 << n]; 

        
        dp[1 << start_node][start_node] = 0;
        
        TspSolver {
            graph,
            n,
            start_node,
            dp,
            parent,
        }
    }

    pub(crate) fn solve(&mut self) -> Option<(i32, Vec<usize>)> {
        self.compute_dp_tables();

        let final_mask = (1 << self.n) - 1;
        if let Some((min_tour_cost, last_node_idx)) = self.find_minimum_tour_details(final_mask) {
            let path = self.reconstruct_tour(last_node_idx, final_mask);
            Some((min_tour_cost, path))
        } else {
            None
        }
    }

    
    fn compute_dp_tables(&mut self) {
        for mask in 1..(1 << self.n) {
            for u in 0..self.n { 
                if (mask >> u) & 1 == 1 { 
                    if self.dp[mask][u] == INF { 
                        continue;
                    }
                    for v in 0..self.n { 
                        if !((mask >> v) & 1 == 1) { 
                            if self.graph[u][v] != INF { 
                                let next_mask = mask | (1 << v);
                                let new_cost = self.dp[mask][u] + self.graph[u][v];

                                if new_cost < self.dp[next_mask][v] {
                                    self.dp[next_mask][v] = new_cost;
                                    self.parent[next_mask][v] = u;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    
    fn find_minimum_tour_details(&self, final_mask: usize) -> Option<(i32, usize)> {
        let mut min_tour_cost = INF;
        let mut best_last_node: Option<usize> = None;

        for i in 0..self.n { 
            if self.dp[final_mask][i] != INF && self.graph[i][self.start_node] != INF {
                let current_total_cost = self.dp[final_mask][i] + self.graph[i][self.start_node];
                if current_total_cost < min_tour_cost {
                    min_tour_cost = current_total_cost;
                    best_last_node = Some(i);
                }
            }
        }

        if min_tour_cost == INF {
            None
        } else {
            Some((min_tour_cost, best_last_node.unwrap()))
        }
    }

    
    fn reconstruct_tour(&self, last_node_idx: usize, final_mask: usize) -> Vec<usize> {
        let mut path = Vec::with_capacity(self.n + 1);
        let mut current_city_idx = last_node_idx;
        let mut current_mask_for_reconstruction = final_mask;

        for _ in 0..self.n {
            path.push(current_city_idx);
            if current_mask_for_reconstruction == (1 << self.start_node) && current_city_idx == self.start_node {
                break; 
            }
            let prev_city_idx = self.parent[current_mask_for_reconstruction][current_city_idx];
            current_mask_for_reconstruction ^= 1 << current_city_idx;
            current_city_idx = prev_city_idx;
        }
        
        path.reverse(); 
        path.push(self.start_node); 
        path
    }
}