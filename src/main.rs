mod inout;
mod ir;
mod lexer;
mod parser;
mod tests;
mod types;
use crate::types::VarV;
use inout::{ask_to_do_smth, get_way_to_run, read_json, Vocabulary};
use parser::parse_program;
use types::FlowListener;
use std::cell::RefCell;
// use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
fn main() {
    let vocabulary: Vocabulary = read_json("vocabulary.json".to_string());
    let mut tokens: Vec<types::TokenV> = lexer::tokenize_code(get_way_to_run(), &vocabulary.keywords);
    if ask_to_do_smth("debug")
    {    
        if ask_to_do_smth("print tokens") {
        println!("{:?}", tokens);
        }
        let tree: Rc<types::StatementV> = parse_program(&mut tokens, &RefCell::new(FlowListener::Console));
        assert_eq!(tokens.len(), 0);
        if ask_to_do_smth("print tree") {
            inout::print_tree(tree.clone().into(), 0);
        }
        if ask_to_do_smth("convert to ir") {
            let mut ir = vec![];
            ir::ast_to_ir(
                tree.clone().into(),
                &mut ir,
            );
            if ask_to_do_smth("print ir") {
                println!("{:#?}", &ir);
            }
            if ask_to_do_smth("run ir") {
                let mut env = HashMap::new();
                ir::execute(ir, &mut env);
            }
        }
    }
    else {
        let tree: Rc<types::StatementV> = parse_program(&mut tokens, &RefCell::new(FlowListener::Console));
        assert_eq!(tokens.len(), 0);
        let mut ir: Vec<ir::IR> = vec![];
        ir::ast_to_ir(tree.clone().into(), &mut ir);
        let mut env: HashMap<String, VarV> = HashMap::new();
        ir::execute(ir, &mut env);
    }
}
