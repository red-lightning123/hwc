use crate::format;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct ResolvedFile {
    minifiles: HashMap<String, String>,
}

impl ResolvedFile {
    pub fn minifiles(&self) -> &HashMap<String, String> {
        &self.minifiles
    }

    fn new(minifiles: HashMap<String, String>) -> ResolvedFile {
        ResolvedFile { minifiles }
    }
}

impl TryFrom<format::File> for ResolvedFile {
    type Error = String;
    fn try_from(file: format::File) -> Result<Self, Self::Error> {
        let mut name_to_vertex = HashMap::new();
        let mut vertex_to_name = vec![];
        for (i, minifile) in file.minifiles().iter().enumerate() {
            let name = minifile.name().to_string();
            if name_to_vertex.get(&name).is_some() {
                return Err("duplicate file name".to_string());
            } else {
                name_to_vertex.insert(minifile.name().to_string(), i);
            }
            vertex_to_name.push(name);
        }

        let mut graph_children = vec![vec![]; file.minifiles().len()];
        for (vertex, minifile) in file.minifiles().iter().enumerate() {
            for item in minifile.content().items() {
                if let format::MinifileContentItem::Include(include) = item {
                    match name_to_vertex.get(include) {
                        Some(child) => graph_children[vertex].push(child),
                        None => {
                            return Err("undefined minifile in include".to_string());
                        }
                    }
                }
            }
            graph_children[vertex].sort();
            graph_children[vertex].dedup();
        }

        let mut graph_parents = vec![vec![]; file.minifiles().len()];
        for (vertex, vertex_children) in graph_children.iter().enumerate() {
            for child in vertex_children {
                graph_parents[**child].push(vertex);
            }
        }

        let mut reached = vec![false; file.minifiles().len()];

        for root in 0..reached.len() {
            if !reached[root] {
                reached[root] = true;
                let mut bfs = VecDeque::new();
                bfs.push_back(root);
                while let Some(vertex) = bfs.pop_front() {
                    for child in &graph_children[vertex] {
                        if !reached[**child] {
                            reached[**child] = true;
                            bfs.push_back(**child);
                        } else {
                            return Err("include cycle".to_string());
                        }
                    }
                }
            }
        }

        let mut dfs_order = vec![];

        for root in 0..file.minifiles().len() {
            if graph_parents[root].is_empty() {
                let mut dfs = vec![root]; // more of a tree scan
                dfs_order.push(root);
                while let Some(vertex) = dfs.pop() {
                    for child in &graph_children[vertex] {
                        dfs.push(**child);
                        dfs_order.push(**child);
                    }
                }
            }
        }

        let mut resolving_minifiles = vec![];

        for minifile in file.minifiles() {
            resolving_minifiles.push(ResolvingMinifile::new(
                minifile.name().to_string(),
                String::new(),
            ));
        }

        while let Some(vertex) = dfs_order.pop() {
            // popping as a stack like this reverses the dfs order, so that leaves get scanned first
            for item in file.minifiles()[vertex].content().items() {
                match item {
                    format::MinifileContentItem::Include(include) => {
                        if let Some(child) = name_to_vertex.get(include) {
                            let child_content = resolving_minifiles[*child].content.to_string();
                            resolving_minifiles[vertex].append_content(&child_content);
                        } else {
                            return Err("undefined minifile in include".to_string());
                            // this should technically never happen because it was checked during the creation of the include graph
                        }
                    }
                    format::MinifileContentItem::Text(text) => {
                        resolving_minifiles[vertex].append_content(text);
                    }
                }
            }
        }

        let mut resolved_minifiles = HashMap::new();

        for minifile in resolving_minifiles {
            resolved_minifiles.insert(minifile.name, minifile.content);
        }

        Ok(ResolvedFile::new(resolved_minifiles))
    }
}

#[derive(Debug)]
struct ResolvingMinifile {
    name: String,
    content: String,
}

impl ResolvingMinifile {
    fn new(name: String, content: String) -> ResolvingMinifile {
        ResolvingMinifile { name, content }
    }

    fn append_content(&mut self, new_content: &str) {
        self.content += new_content;
    }
}
