mod tree;
use tree::binary_tree::Node;
use std::collections::HashMap;


fn main() {
    let mut char_map = HashMap::new();
    let src = "asdada";
    let src_len = src.len();

    src.chars().map(|c| {
        char_map
            .insert(c, 1)
            .and_then(|count| char_map.insert(c, count + 1))
    }).collect::<Vec<_>>();

    let node_list = char_map.iter()
        .map(|(k, v)| { Node::new(Some(*k), *v as f32 / src_len as f32)} )
        .collect::<Vec<Node>>();
    let root_node = build_tree(node_list);
    println!("root_node = {:#?}", root_node);

}


fn build_tree(list: Vec<Node>) -> Node {
    let mut temp_list = combine_node(list);
    while temp_list.len() > 1 {
        temp_list = combine_node(temp_list);
    }
    return temp_list[0].clone();
}

fn combine_node(mut list: Vec<Node>) -> Vec<Node> {
    list.sort_by(|a, b| a.pow.partial_cmp(&b.pow).unwrap());
    let mut temp_list = vec![];
    {
        let (v1, v2) = list.split_at_mut(2);
        temp_list.push(link(v1[0].clone(), v1[1].clone()));
        temp_list.append(&mut v2.to_vec());
    }
    temp_list
}

fn link(n1: Node, n2: Node) -> Node {
    let mut node = Node::new(None, n1.pow + n2.pow);
    if n1.pow < n2.pow {
        node.left = Some(Box::new(n1));
        node.right = Some(Box::new(n2));
    } else {
        node.left = Some(Box::new(n2));
        node.right = Some(Box::new(n1));
    }
    node
}
