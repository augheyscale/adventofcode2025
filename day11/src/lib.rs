use anyhow::Result;
use std::collections::HashMap;

pub type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

pub fn parse_data(data: &str) -> Result<Graph<'_>> {
    let mut node_map = Graph::new();

    // Read all the data into the node map.
    for line in data.lines() {
        let (name, connections) = line
            .split_once(": ")
            .ok_or_else(|| anyhow::anyhow!("Invalid line: {}", line))?;
        let connections = connections.split(" ").map(|c| c.trim()).collect::<Vec<_>>();
        node_map.insert(name, connections);
    }

    Ok(node_map)
}
