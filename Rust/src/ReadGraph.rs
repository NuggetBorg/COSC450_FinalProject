use std::collections::HashMap;
use std::fs;
 

pub type Graph = HashMap<String, Vec<(String, f64)>>;
 

pub struct ReadGraph 
{
    edge_count: usize,
}
 

impl ReadGraph 
{
    pub fn new() -> Self 
    {
        ReadGraph {edge_count: 0}
    }
 
    //Reads in the CVS graph file it is passed when called in main
    //Returns a graph the is constucted like the readed in file just without ','
    pub fn read_file(&mut self, path: &str) -> Graph 
    {
        let mut graph = Graph::new();
        self.edge_count = 0;
 
        
        let text = match fs::read_to_string(path) 
        {
            Ok(t) => t,
            Err(_) => 
            {
                println!("File not found");
                return graph;
            }
        };
 
        
        for (i, currline) in text.lines().enumerate() 
        {
            let currline = currline.trim();
 
            //skip the header and blank lines
            if i == 0 || currline.is_empty() 
            {
                continue;
            }
 
            //Split currect line by ',' into a list of parts to get its values
            let parts: Vec<&str> = currline.split(',').map(|p| p.trim()).collect();
            if parts.len() != 3 
            {
                println!("Skipping line {}: expected 3 fields", i + 1);
                continue;
            }
 
           
            let weight: f64 = parts[2].parse().unwrap();
            let from = parts[0].to_string();
            let to = parts[1].to_string();
 
            graph.entry(to.clone()).or_default(); 
            graph.entry(from).or_default().push((to, weight)); //add the point to the graph
            self.edge_count += 1;
        }
 
        graph
    }
 
}