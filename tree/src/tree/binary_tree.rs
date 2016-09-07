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
}
#[derive(Debug, Clone)]
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
    input: &'static str,
    pos: usize,
}

impl HuffmanCoder {
    pub fn new(src: &'static str) -> HuffmanCoder {
        let mut tree = HuffmanTree::build_tree(src);
        tree.build_ecode_map();
        HuffmanCoder {
            tree: tree,
            pos: 0,
            input: "",
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
        let mut res = String::new();
        while self.eof() {
            match self.tree.root {
                Some(_) => {
                    let str = self.consumeWhile(Self::is_accept);
                    res = res + &str;
                },
                None => println!("root is null"),
            }
        }
        res
    }
    fn is_accept(node: Node, c: char) -> bool {
        if let Some(ch) = node.key {
            if ch == c {
                return true;
            }

        }
        match (node.left.clone(), node.right.clone()) {
            (Some(ref left), Some(ref right)) => Self::is_accept(**left, c) && Self::is_accept(**right, c),
            (Some(ref left), _) => Self::is_accept(**left, c),
            (_, Some(ref right)) => Self::is_accept(**right, c),
            (_, _) => false,
        }
    }
    fn consumeWhile<F>(&mut self, test: F) -> String
        where F: Fn(Node, char) -> bool {
        let mut res = String::new();
        while !self.eof() && test(self.tree.root.unwrap(), self.next_char()) {
            res.push(self.consume_char());
        }
        res
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

}





