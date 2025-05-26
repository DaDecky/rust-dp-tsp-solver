pub mod common; 
mod solver;     

pub fn solve_tsp_dynamic_programming(graph: &Vec<Vec<i32>>) -> Option<(i32, Vec<usize>)> {
    let n = graph.len();

    if n == 0 {
        return None; 
    }


    let start_node: usize = 0; 

    
    let mut tsp_solver_instance = solver::TspSolver::new(graph, start_node);
    tsp_solver_instance.solve()
}
