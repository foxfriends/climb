use sexp::{Atom, Sexp};

/// A very simple Tree structure.
#[derive(Debug)]
pub struct Tree {
    label: String,
    children: Vec<Tree>,
}

impl Tree {
    pub fn size(&self) -> (usize, usize) {
        let (w, h) = self.children
            .iter()
            .map(Self::size)
            .fold((0, 0), |(w, h), (cw, ch)| (usize::max(w, cw), h + ch));
        (w + 1, usize::max(h, 1))
    }
}

impl From<Sexp> for Tree {
    fn from(sexp: Sexp) -> Self {
        match sexp {
            Sexp::Atom(atom) => Tree { label: to_label(atom), children: vec![] },
            Sexp::List(sexps) => {
                match &sexps[0] {
                    Sexp::Atom(atom) => {
                        Tree {
                            label: to_label(atom.clone()),
                            children: sexps.into_iter().skip(1).map(Into::into).collect(),
                        }
                    }
                    _ => {
                        Tree {
                            label: String::from("<···>"),
                            children: sexps.into_iter().map(Into::into).collect(),
                        }
                    }
                }
            }
        }
    }
}

impl<'a> IntoIterator for &'a Tree {
    type IntoIter = TreeTableIter<'a>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter { TreeTableIter::new(self) }
}

fn to_label(atom: Atom) -> String {
    match atom {
        Atom::S(label) => label,
        Atom::I(value) => value.to_string(),
        Atom::F(value) => value.to_string(),
    }
}

pub struct TreeTableIter<'a> {
    tree: &'a Tree,
    row: usize,
    end: usize,
}

impl<'a> TreeTableIter<'a> {
    fn new(tree: &'a Tree) -> Self {
        Self {
            tree,
            row: 0,
            end: tree.size().1,
        }
    }
}

pub struct TreeRowIter<'a> {
    tree: Option<&'a Tree>,
    offset: usize,
}

impl<'a> TreeRowIter<'a> {
    fn new(mut tree: &'a Tree, mut row: usize) -> Self {
        let mut offset = 0;
        'outer: while row != 0 {
            for child in &tree.children {
                let (.., h) = child.size();
                if h > row {
                    offset += 1;
                    tree = child;
                    continue 'outer;
                } else {
                    row -= h;
                }
            }
            panic!("row out of range for tree");
        }

        Self {
            tree: Some(tree),
            offset,
        }
    }
}

impl<'a> Iterator for TreeTableIter<'a> {
    type Item = TreeRowIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.end {
            return None;
        }
        let item = TreeRowIter::new(self.tree, self.row);
        self.row += 1;
        Some(item)
    }
}

impl<'a> Iterator for TreeRowIter<'a> {
    type Item = Option<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset > 0 {
            self.offset -= 1;
            return Some(None);
        }

        match self.tree.take() {
            Some(tree) => {
                self.tree = tree.children.first();
                Some(Some(tree.label.as_str()))
            }
            None => None,
        }
    }
}
