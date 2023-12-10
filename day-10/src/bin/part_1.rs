use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

#[derive(Clone, Debug)]
struct Field {
    matrix: Vec<Vec<char>>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Edge {
    from: (usize, usize),
    to: (usize, usize),
    weight: usize,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Node {
    id: char,
    pos: (usize, usize),
}

#[derive(Clone, Debug)]
struct Graph {
    nodes: HashSet<Node>,
    edges: HashSet<Edge>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashSet::new(),
        }
    }
}

fn parse_input_to_matrix(input: &str) -> Field {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    Field { matrix }
}

fn find_start_point(field: &Field) -> (usize, usize) {
    for (y, row) in field.matrix.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 'S' {
                return (x, y);
            }
        }
    }
    panic!("No start point found")
}

fn check_neighbours(field: &Field, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    let dir = Vec::from([(-1, 0), (0, -1), (1, 0), (0, 1)]);

    for (dx, dy) in dir {
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if nx < 0 || ny < 0 {
            continue;
        }
        let (nx, ny) = (nx as usize, ny as usize);
        if nx >= field.matrix[0].len() || ny >= field.matrix.len() {
            continue;
        }
        if field.matrix[ny][nx] == '.' {
            continue;
        }
        neighbours.push((nx, ny));
    }
    neighbours
}

fn create_graph(field: &Field) -> Graph {
    let mut graph = Graph::new();
    let (start_x, start_y) = find_start_point(field);

    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push((start_x, start_y));

    while !queue.is_empty() {
        let (x, y) = queue.pop().unwrap();
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        let node = Node {
            id: field.matrix[y][x],
            pos: (x, y),
        };
        graph.nodes.insert(node);

        let neighbours = check_neighbours(field, (x, y));
        for (nx, ny) in neighbours {
            let edge = Edge {
                from: (x, y),
                to: (nx, ny),
                weight: 1,
            };
            graph.edges.insert(edge);
            queue.push((nx, ny));
        }
    }

    graph
}

fn find_furthest_node(graph: &Graph, start: &Node) -> Node {
    let mut furthest_node = start;
    let mut max_distance = 0.0;

    for node in &graph.nodes {
        let distance = ((node.pos.0 as i32 - start.pos.0 as i32).pow(2) as f64
            + (node.pos.1 as i32 - start.pos.1 as i32).pow(2) as f64)
            .sqrt();

        if distance > max_distance {
            max_distance = distance;
            furthest_node = node;
        }
    }

    furthest_node.clone()
}

fn find_longest_path(graph: &Graph, from: Node, to: &Node) -> Vec<Node> {
    if from == *to {
        return vec![from];
    }

    let mut path = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push(from.clone());

    while !queue.is_empty() {
        let node = queue.pop().unwrap();
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node.clone());

        path.push(node.clone());
        if node == *to {
            break;
        }

        let neighbours = graph
            .edges
            .iter()
            .filter(|edge| edge.from == node.pos)
            .map(|edge| {
                graph
                    .nodes
                    .iter()
                    .find(|&node_| node_.pos == edge.to && node != *node_)
                    .unwrap()
            })
            .collect::<Vec<_>>();

        for neighbour in neighbours {
            queue.push(neighbour.clone());
        }
    }

    path
}

fn part_1(input: &str) -> usize {
    let field = parse_input_to_matrix(input);
    let graph = create_graph(&field);

    let start = graph.nodes.iter().find(|node| node.id == 'S').unwrap();

    let result = find_furthest_node(&graph, start);

    let path = find_longest_path(&graph, start.clone(), &result);

    path.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let result = part_1(input);
        assert_eq!(result, 4)
    }

    // #[test]
    fn test_2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let result = part_1(input);
        assert_eq!(result, 8)
    }
}
