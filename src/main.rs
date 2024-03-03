pub mod networks;
use networks::Node;

fn create_network<'a>(network: &'a networks::NetworkArena<'a>) -> Vec<&'a Node> {
    let a = Node::new("a", network);
    let b = Node::new("b", network);
    let c = Node::new("c", network);
    networks::attach_to(& a, & b);
    networks::attach_to(& a, & c);
    let d = Node::new("d", network);
    networks::attach_to(b, d);
    networks::attach_to(d, a);
    vec![&a, &b, &c, &d]
}

fn print_label(node: &Node) {
    println!("Reached {}", node.label);
}

fn main() {
    let network = networks::NetworkArena::new();
    let nodes = create_network(&network);
    for source in &nodes {
        for target in source.out_nodes() {
            println!("{}: {} -> {}", source.label, source.label, target.label)
        }

        for target in source.in_nodes() {
            println!("{}: {} -> {}", source.label, source.label, target.label)
        }
    }

    networks::dfs(&print_label, nodes[0])
}
