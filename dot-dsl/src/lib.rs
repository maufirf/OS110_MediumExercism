pub mod graph {
    extern crate maplit;

    use std::collections::HashMap;
    use std::cmp::PartialEq;
    //use std::clone::Clone;
    //use std::fmt;

    pub mod graph_items {
        pub mod edge {
            use std::fmt;
            use std::collections::HashMap;
            pub struct Edge{
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {from: from.to_string(), to: to.to_string(), attrs: HashMap::new() }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for tpl in attrs.iter() {
                        self.attrs.insert(tpl.0.to_string(), tpl.1.to_string());
                    }
                    //self.attrs = attrs.to_vec().into_iter().collect();
                    self
                }
            }
            impl PartialEq for Edge {
                fn eq (&self, other: &Edge) -> bool {
                    (self.from == other.from) && (self.to == other.to)
                }
            }
            impl Clone for Edge {
                fn clone(&self) -> Self {
                    let mut attrs: Vec<(&str, &str)> = Vec::new();
                    for (i, j) in self.attrs.iter() {
                        attrs.push((&i[..], &j[..]));
                    }
                    Edge::new(&self.from.clone(), &self.to.clone()).with_attrs(&attrs)
                    //.with_attrs(&self.attrs.into_iter().collect::<Vec<(String, String)>>();
                }
            }
            impl fmt::Debug for Edge {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "Node {{ {} -> {} }}", self.from, self.to)
                }
            }
            impl fmt::Display for Edge {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "Node {{ {} -> {} }}", self.from, self.to)
                }
            }
        }
        pub mod node {
            use std::fmt;
            use std::collections::HashMap;
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {name: name.to_string(), attrs: HashMap::new()}
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for tpl in attrs.iter() {
                        self.attrs.insert(tpl.0.to_string(), tpl.1.to_string());
                    }
                    self
                }
                pub fn expect(&self, msg: &str) -> &Self {
                    println!("{}", msg);
                    self
                }
                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    Some(&self.attrs.get(&key.to_string()).unwrap())
                }
            }
            impl PartialEq for Node {
                fn eq (&self, other: &Node) -> bool {
                    self.name == other.name
                }
            }
            impl Clone for Node {
                fn clone(&self) -> Self {
                    let mut attrs: Vec<(&str, &str)> = Vec::new();
                    for (i, j) in self.attrs.iter() {
                        attrs.push((&i[..], &j[..]));
                    }
                    Node::new(&self.name.clone()).with_attrs(&attrs)
                    //.with_attrs(&self.attrs.into_iter().collect::<Vec<(String, String)>>();
                }
            }
            impl fmt::Debug for Node {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "Node {{ {} }}", self.name)
                }
            }
            impl fmt::Display for Node {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "Node {{ {} }}", self.name)
                }
            }
        }
    }
    use graph_items::edge::Edge;
    use graph_items::node::Node;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {nodes: Vec::new(), edges: Vec::new(), attrs: HashMap::new()}
        }
        pub fn with_nodes(mut self, vec: &Vec<Node>) -> Self {
            self.nodes = vec.to_vec();
            self
        }
        pub fn with_edges(mut self, vec: &Vec<Edge>) -> Self {
            self.edges = vec.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for tpl in attrs.iter() {
                self.attrs.insert(tpl.0.to_string(), tpl.1.to_string());
            }
            self
        }
        pub fn get_node(&self, name: &str) -> &Node {
            self.nodes.iter().find(|node| &node.name[..] == name).unwrap()
        }
    }

    impl PartialEq for Graph {
        fn eq(&self, other: &Graph) -> bool {
            ((self.edges == other.edges) && (self.nodes == other.nodes)) && (self.attrs == other.attrs)
        }
    }
}
