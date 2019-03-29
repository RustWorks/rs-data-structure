#![allow(dead_code)]

use rs_data_structure::*;
use tree::bintree::BinTree;
use tree::bin_search_tree::BinSearchTree;

fn display_bin_tree() {
    let seq = "ABC##DE#G##F###";
    let tree = BinTree::from_seq_pre(seq.chars().into_iter(), &'#');
    println!("{}", tree);
}

fn display_bin_search_tree() {
    let mut tree = BinSearchTree::new();
    tree.insert(5);
    tree.insert(2);
    tree.insert(3);
    tree.insert(9);
    tree.insert(9);
    tree.insert(10);
    println!("{}", tree);
}

fn main() {
    display_bin_search_tree();
    use expr::*;
    use std::str::FromStr;
    let infix_expr: Expr<Infix> = Expr::from_str("1+2*(5-3)").unwrap();
    let pre :Expr<Prefix> = infix_expr.into();
    println!("{}", format!("{}", pre));
}
