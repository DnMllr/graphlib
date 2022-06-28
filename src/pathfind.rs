use std::{
    collections::{BinaryHeap, HashMap},
    io::{stdin, BufRead, BufReader},
};

use smallvec::SmallVec;

pub fn pathfind(start: usize, end: usize) -> color_eyre::Result<()> {
    let stdin = stdin();
    let mut graph = Graph::new(BufReader::new(stdin.lock()));
    if let Some(path) = astar(&mut graph, start, end)? {
        println!("path found: {:?}", path);
    } else {
        println!("no path found");
    }

    Ok(())
}

fn astar<T: BufRead>(
    graph: &mut Graph<T>,
    start: usize,
    end: usize,
) -> color_eyre::Result<Option<SmallVec<[usize; 32]>>> {
    let mut search = SearchState::new(start, end);
    let mut buffer = SmallVec::new();

    while let Some(index) = search.next() {
        buffer.clear();
        buffer.extend_from_slice(
            search
                .path(index)
                .expect("we put this node here, this will definitely succeed"),
        );

        if search.is_done(index) {
            buffer.push(index);
            return Ok(Some(buffer));
        }

        let node = graph.node_at(index)?.expect("invalid input");

        for child in node {
            search.visit(*child, &buffer, index);
        }
    }

    Ok(None)
}

struct SearchState {
    end: usize,
    queue: BinaryHeap<Node>,
    costs: HashMap<usize, SmallVec<[usize; 8]>>,
}

impl SearchState {
    pub fn new(start: usize, end: usize) -> Self {
        let mut search = Self {
            end,
            queue: BinaryHeap::new(),
            costs: HashMap::new(),
        };

        search.costs.insert(start, Default::default());
        search.queue.push(Node {
            index: start,
            priority: 0,
        });

        search
    }

    pub fn visit(&mut self, index: usize, previous_path: &[usize], parent: usize) {
        if self.should_update_position(index, previous_path) {
            let new_path = self.costs.entry(index).or_default();
            new_path.clear();
            new_path.extend_from_slice(previous_path);
            new_path.push(parent);

            self.queue.push(Node {
                index,
                priority: new_path.len() + self.distance_from_end(index),
            })
        }
    }

    pub fn path(&self, index: usize) -> Option<&[usize]> {
        self.costs.get(&index).map(AsRef::as_ref)
    }

    pub fn is_done(&self, index: usize) -> bool {
        self.end == index
    }

    fn should_update_position(&self, index: usize, path: &[usize]) -> bool {
        self.costs
            .get(&index)
            .map(|p| (path.len() + 1) < p.len())
            .unwrap_or(true)
    }

    fn distance_from_end(&self, index: usize) -> usize {
        self.end.abs_diff(index)
    }
}

impl Iterator for SearchState {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop().map(|n| n.index)
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Node {
    index: usize,
    priority: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

enum GraphStatus {
    Incomplete,
    Done,
}

struct Graph<T: BufRead> {
    input: T,
    done: bool,
    nodes: Vec<Vec<usize>>,
    buf: String,
}

impl<T: BufRead> Graph<T> {
    fn new(input: T) -> Self {
        Self {
            input,
            done: false,
            nodes: Default::default(),
            buf: String::new(),
        }
    }

    fn node_at(&mut self, index: usize) -> color_eyre::Result<Option<&[usize]>> {
        self.read_until(index)?;
        Ok(self.nodes.get(index).map(AsRef::as_ref))
    }

    fn read_until(&mut self, index: usize) -> color_eyre::Result<GraphStatus> {
        if self.done {
            return Ok(GraphStatus::Done);
        }

        while self.nodes.len() <= index && !self.done {
            self.done = self.read_next_line_into_buffer()?;
            let children = self.parse_node_from_buffer()?;
            self.nodes.push(children);
        }

        Ok(GraphStatus::Incomplete)
    }

    fn read_next_line_into_buffer(&mut self) -> color_eyre::Result<bool> {
        self.buf.clear();
        Ok(self.input.read_line(&mut self.buf)? == 0)
    }

    fn parse_node_from_buffer(&mut self) -> color_eyre::Result<Vec<usize>> {
        let mut node = Vec::new();
        for n in self.buf.split_whitespace() {
            let child = n.parse()?;
            node.push(child);
        }

        self.buf.clear();

        Ok(node)
    }
}

// // let's be a good unix citizen and read the entire pipe even if we don't do anything with it.
// impl<T: BufRead> Drop for Graph<T> {
//     fn drop(&mut self) {
//         let mut buf = [0; 1 << 16];
//         while self.input.read(&mut buf).unwrap() > 0 {}
//     }
// }
