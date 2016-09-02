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
}