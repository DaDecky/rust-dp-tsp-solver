

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader}; 
use std::path::Path;
use std::process; 
use tsp_stima::solve_tsp_dynamic_programming;
use tsp_stima::common::INF;

fn parse_matrix_from_file(file_path: &str) -> Result<Vec<Vec<i32>>, String> {
    let path = Path::new(file_path);
    let file = File::open(path)
        .map_err(|e| format!("Gagal membuka file '{}': {}", file_path, e))?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let mut num_cols_expected: Option<usize> = None;

    for (line_idx, line_result) in reader.lines().enumerate() {
        let line_number = line_idx + 1;
        let line_str = line_result
            .map_err(|e| format!("Gagal membaca baris {} dari file: {}", line_number, e))?;

        
        if line_str.trim().is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = line_str.split_whitespace().collect();
        
        
        if parts.is_empty() { 
            continue;
        }

        let mut row: Vec<i32> = Vec::new();
        for s in parts {
            match s.parse::<i32>() {
                Ok(val) => row.push(val),
                Err(e) => return Err(format!("Gagal parse integer '{}' pada baris {}: {}", s, line_number, e)),
            }
        }

        
        if let Some(expected_cols) = num_cols_expected {
            if row.len() != expected_cols {
                return Err(format!(
                    "Format matriks tidak konsisten: baris {} memiliki {} kolom, diharapkan {}",
                    line_number, row.len(), expected_cols
                ));
            }
        } else {
            
            if row.is_empty() { 
                
                
                
                return Err(format!("Baris data pertama (baris {}) tidak valid atau kosong setelah parsing.", line_number));
            }
            num_cols_expected = Some(row.len());
        }
        matrix.push(row);
    }

    if matrix.is_empty() {
        return Err("File tidak berisi data matriks yang valid (kosong atau hanya baris kosong).".to_string());
    }

    
    let num_rows = matrix.len();
    
    let num_cols = num_cols_expected.unwrap_or(0); 

    if num_cols == 0 && num_rows > 0 { 
        return Err("Matriks yang diparsing memiliki 0 kolom meskipun ada baris.".to_string());
    }

    if num_rows != num_cols {
        return Err(format!(
            "Matriks tidak persegi: {} baris dan {} kolom.",
            num_rows, num_cols
        ));
    }

    Ok(matrix)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Mode default: Menjalankan contoh bawaan karena tidak ada path file yang diberikan.");
        eprintln!("Penggunaan: {} <path_ke_file_matriks_adjacency>", args[0]);
        run_default_examples();
        process::exit(0); 
    }

    let file_path = &args[1];
    println!("Membaca matriks dari file: {}", file_path);

    match parse_matrix_from_file(file_path) {
        Ok(graph_from_file) => {
            if graph_from_file.is_empty() {
                eprintln!("Error: Matriks yang diparsing dari file ternyata kosong.");
                process::exit(1);
            }
            println!(
                "Matriks yang diparsing ({}x{}):",
                graph_from_file.len(),
                graph_from_file[0].len() 
            );
            for row in graph_from_file.iter() {
                println!("  {:?}", row);
            }

            println!("\nMenyelesaikan TSP untuk matriks dari file...");
            match solve_tsp_dynamic_programming(&graph_from_file) {
                Some((cost, path)) => {
                    println!("  Biaya Minimum Tur: {}", cost);
                    println!("  Jalur: {:?}", path);
                }
                None => {
                    println!("  Tidak ditemukan solusi tur untuk matriks dari file.");
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            process::exit(1);
        }
    }
}


fn run_default_examples() {
    println!("\n--- Contoh Bawaan ---");
    
    let graph1 = vec![
        vec![INF, 10, 15, 20],
        vec![10, INF, 35, 25],
        vec![15, 35, INF, 30],
        vec![20, 25, 30, INF],
    ];
    println!("\nGraf Contoh 1:");
    match solve_tsp_dynamic_programming(&graph1) {
        Some((cost, path)) => {
            println!("  Biaya Minimum Tur: {}", cost);
            println!("  Jalur: {:?}", path);
        }
        None => { println!("  Tidak ditemukan solusi tur."); }
    }

    let graph2 = vec![
        vec![INF, 2, INF, 6, INF],
        vec![2, INF, 3, 8, 5],
        vec![INF, 3, INF, INF, 7],
        vec![6, 8, INF, INF, 9],
        vec![INF, 5, 7, 9, INF],
    ];
    println!("\nGraf Contoh 2:");
    match solve_tsp_dynamic_programming(&graph2) {
        Some((cost, path)) => {
            println!("  Biaya Minimum Tur: {}", cost);
            println!("  Jalur: {:?}", path);
        }
        None => { println!("  Tidak ditemukan solusi tur."); }
    }
    
}