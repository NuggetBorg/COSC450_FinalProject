
#[path = "ReadGraph.rs"] //This is so that it will look for the correct file neme 
mod read_graph; //So the class can use the ReadGraphs's class methods

use read_graph::{Graph};
use std::collections::{HashMap, HashSet, VecDeque};

//Queue of the correct path need to make from point a to point b
type CorrectPath = VecDeque<String>;
 
 //Main method this reads the file in from the samples folder
 //Takes the file and puts it through the ReadGraph class
 //Prints the graph to user in way that it shows each place in the graph and there neighbors
 //promps user to select there start and end point
 //calls shortest_path class to get the shortest path and the total weight/cost
 //prints the path along with the total weight/cost
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

    let result = shortest_path(&graph, &start, &end);

    
    if result.is_none()
    {
        println!("Path is not possible");
    }
    else
    {

        let (cost, mut path) = result.unwrap();
        println!("The total cost for starting at {} and ending at {} is: {:.1}", start, end, cost);
        println!();

        let mut first_time = true;
        while let Some(place) = path.pop_front()
        {
            if first_time
            {
                println!("Start: {}", place);
                first_time = false;
            }
            else if path.is_empty()
            {
                println!("End:   {}", place);
            }
            else
            {
                println!("Go to: {}", place);
            }
        }
        
    }
}

//Finds the shortest path possible and adds it to a queue called correct path
//Keeps track of the total weight/cost of the trip
fn shortest_path(graph: &Graph, start: &str, end: &str) ->Option<(f64, CorrectPath)>
{
   let mut dist: HashMap<String, f64> = HashMap::new();    //Cheapest cost found 
    let mut prev: HashMap<String, String> = HashMap::new(); //Prevouse place 
    let mut visited: HashSet<String> = HashSet::new();      //Places that have been visited 

    dist.insert(start.to_string(), 0.0); //Adds the start and 0.0 since there is no cost for starting 

    let mut found = false;

    while !found
    {
        //Resets the curr and the best back to nothing 
        let mut curr: Option<String> = None;
        let mut best = f64::INFINITY;

        for (name, d) in &dist
        {
            if !visited.contains(name) && *d < best
            {
                best = *d;
                curr = Some(name.clone());
            }
        }

        //Can not be possible, so returns nothing
        if curr.is_none()
        {
            return None;
        }
        let curr = curr.unwrap();

        if curr == end
        {
            found = true; //Found the end and the path is found
        }
        else
        {
            visited.insert(curr.clone());

            for (neighbor, weight) in &graph[&curr]
            {
                let new_dist = best + weight; 
                if !dist.contains_key(neighbor) || new_dist < dist[neighbor]
                {
                     dist.insert(neighbor.clone(), new_dist);
                     prev.insert(neighbor.clone(), curr.clone());
                }
            }


        }
    }

    //Reconstrucks path and puts it in a queue 
    let mut path = CorrectPath::new();
    let mut node = end.to_string();
    path.push_front(node.clone());
    while node != start
    {
        node = prev[&node].clone();
        path.push_front(node.clone());
    }

    Some((dist[end], path))
}

