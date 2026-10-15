
#[path = "ReadGraph.rs"] //This is so that it will look for the correct file neme 
mod read_graph; //So the class can use the ReadGraphs's class methods
 
use std::io::{self, Write};
use read_graph::{Graph, ReadGraph};
 
fn main()
{
    //Gets the CSV file from my computer and calls it path 
    let path = std::env::args().nth(1).unwrap_or_else(|| "/Users/jacobbeeman/Desktop/COSC450_FinalProject/Samples/city_roads.csv".to_string());
 
    
    let mut reader = read_graph::ReadGraph::new();
    let graph = reader.read_file(&path);
 
    //Sorts the names of the places
    let mut names: Vec<&String> = graph.keys().collect();
    names.sort();
 
    // print each place, then its neighbors indented underneath
    for name in names
    {
        println!("{}", name);
        for (neighbor, weight) in &graph[name]
        {
            
            println!("  Can go to - {} and will cost ({:.1})", neighbor, weight);//prints neighnors and there weights to get to each neighbor
        }
    }

    let mut start = String::new();

    while start.is_empty()
    {
        println!("Where are you starting from? ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        for name in graph.keys()
        {
            if name.eq_ignore_ascii_case(&input)
            {
                start = name.clone();
            }
        }

        if start.is_empty()
        {
            println!("That place is not in the graph.");
        }
    }

    let mut end = String::new();

    while end.is_empty()
    {
        print!("Where are you ending at? ");
        println!("");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
       

        for name in graph.keys()
        {
            if name.eq_ignore_ascii_case(&input)
            {
                end = name.clone();
            }
        }

        if end.is_empty()
        {
            println!("That place is not in the graph.");
        }
    }

    if start.eq_ignore_ascii_case(&end)
    {
        println!("{} is the same as {} and will cost 0", start, end)
    }
    else
    {
        println!("Going from {} to {} will cost", start, end);
    }
}

fn shortest_path()
{
    //noting yet
}

