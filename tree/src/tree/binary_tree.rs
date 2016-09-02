use std::collections::HashMap;
pub type Link = Option<Box<Node>>;
#[derive(Debug, Clone)]
pub struct Node {
    pub pow: f32,
    pub key: Option<char>,
    pub left: Link,
    pub right: Link,
}

impl Drop for Node {
    fn drop(&mut self) {
//        println!("self.left = {:#?}", self.left);
    }
}

impl Node {
    pub fn new(key: Option<char>, pow: f32) -> Self {
        Node {
            pow: pow,
            key: key,
            left: None,
            right: None,
        }
    }
    /// 将 pow 字段的值相加并返回一个新 Node
    pub fn add_pow(&self, node: Node) -> Self {
        let mut ret = Node::new(None, self.pow + node.pow);
        if self.pow < node.pow {
            ret.left = Some(Box::new(self.clone()));
            ret.right = Some(Box::new(node));
        } else {
            ret.left = Some(Box::new(node));
            ret.right = Some(Box::new(self.clone()));
        }
        ret
    }
    fn get_path_code(&self, code: String, vec:&mut Vec<(char, String)>) {
        if let Some(ref c) = self.key {
            vec.push((*c, code.clone()));
        }
        if let Some(ref left) = self.left {
            left.get_path_code(code.clone() + &"0", vec);
        }
        if let Some(ref right) = self.right {
            right.get_path_code(code.clone() + &"1", vec);
        }
    }

    fn get_char(&self, code: String) -> char {
        for c in code.chars() {
            if let Some(k) = self.key {
                if k == c {
                    return k.clone();
                }
            } else {
               let ch =  match (c, self.left.clone(), self.right.clone()) {
                    ('0', Some(left), _) => {
                        left.get_char(code[1..].to_string())
                    },
                    ('1', _, Some(right)) => {
                        right.get_char(code[1..].to_string())
                    },
                    (_, _, _) => ' ',
                };
                return ch;
            }
        }
        return ' ';

    }
}
#[derive(Debug)]
pub struct HuffmanTree {
    root: Option<Node>,
    ecode_map: HashMap<char, String>,
}

impl HuffmanTree {
    pub fn new(root: Node) -> Self {
        HuffmanTree {
            root: Some(root),
            ecode_map: HashMap::new(),
        }
    }
    /// 创建一个 Node 列表, pow = 某个字符出现的次数 / 字符串总长度
    /// key 为该字符
    /// pub struct Node {
    ///    pub pow: f32,
    ///    pub key: Option<char>,
    ///    pub left: Link,
    ///    pub right: Link,
    /// }
    ///
    fn build_pow_list(src: &'static str) -> Vec<Node> {
        let src_len = src.len();
        let mut char_map = HashMap::new();
        src.chars().map(|c| {
            char_map
                .insert(c, 1)
                .and_then(|count| char_map.insert(c, count + 1))
        }).collect::<Vec<_>>();

        let node_list = char_map.iter()
            .map(|(k, v)| { Node::new(Some(*k), *v as f32 / src_len as f32)} )
            .collect::<Vec<Node>>();

        node_list
    }
    /// 根据字符串中每个字符串出现频率的权重建立一颗树
    fn build_tree(src: &'static str) -> HuffmanTree {

        // 根据字符串创建一个 node 列表
        let node_list = Self::build_pow_list(src);

        let mut temp_list = Self::combine_node(node_list);
        while temp_list.len() > 1 {
            temp_list = Self::combine_node(temp_list);
        }
        return HuffmanTree::new(temp_list[0].clone());
    }

    /// 将 Node 列表按权重从小到大排序, 取出前两个相加, 并与剩下的元素组成新列表
    fn combine_node(mut list: Vec<Node>) -> Vec<Node> {
        list.sort_by(|a, b| a.pow.partial_cmp(&b.pow).unwrap());
        let mut temp_list = vec![];
        {
            let (v1, v2) = list.split_at_mut(2);
            temp_list.push(v1[0].add_pow(v1[1].clone()));
            temp_list.append(&mut v2.to_vec());
        }
        temp_list
    }

    fn build_ecode_map(&mut self) {
        let mut paths = vec![];
        if let Some(ref root) = self.root {
            root.get_path_code("".to_string(), &mut paths);
        }
        for path in paths.iter() {
            self.ecode_map.insert(path.0.clone(), path.1.clone());
        }
    }



}

pub struct HuffmanCoder {
    tree: HuffmanTree,
}

impl HuffmanCoder {
    pub fn new(src: &'static str) -> HuffmanCoder {
        let mut tree = HuffmanTree::build_tree(src);
        tree.build_ecode_map();
        HuffmanCoder {
            tree: tree,
        }
    }
    pub fn ecode(&mut self, src: &'static str) -> String {
        let mut ret = "".to_string();
        for c in src.chars() {
            if let Some(v)  = self.tree.ecode_map.get(&c) {
                ret = ret + &v;
            }
        }
        ret
    }
    pub fn decode(&mut self, src: &'static str) -> String {
        let mut ret = "".to_string();
        let mut search = "".to_string();
        for c in src.chars() {
            search = search + &(c.to_string());
            if let Some(k) = self.tree.ecode_map.get(search) {
                ret = ret + &k;
                search = "".to_string();
            }
        }
        ret
    }
}




