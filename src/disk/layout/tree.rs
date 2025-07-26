use std::{collections::BTreeMap, ffi::OsString, path::Path};

#[derive(Debug, Clone)]
pub enum TreeNode {
    Dir(BTreeMap<OsString, TreeNode>),
    File { length: u64 },
}

#[derive(Debug, Clone)]
pub struct TreeLayout {
    pub root_dir: OsString,
    pub tree: TreeNode,
    pub length: u64,
}

impl TreeLayout {
    pub fn new<D: Into<OsString>>(root_dir: D) -> Self {
        Self {
            root_dir: root_dir.into(),
            tree: TreeNode::Dir(BTreeMap::new()),
            length: 0,
        }
    }

    pub fn add_file<P: AsRef<Path>>(&mut self, path: P, length: u64) {
        let components: Vec<_> = path.as_ref().iter().collect();
        let Some((last, parents)) = components.split_last() else {
            return;
        };
        let mut cursor = &mut self.tree;
        for component in parents {
            if let TreeNode::Dir(ref mut dir) = cursor {
                cursor = dir
                    .entry((*component).to_owned())
                    .or_insert_with(|| TreeNode::Dir(BTreeMap::new()));
            }
        }
        if let TreeNode::Dir(ref mut dir) = cursor {
            if dir
                .insert((*last).to_owned(), TreeNode::File { length })
                .is_none()
            {
                self.length += length;
            }
        }
    }
}
