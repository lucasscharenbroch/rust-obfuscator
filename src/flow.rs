use syn::{ visit_mut::VisitMut, Block, Stmt, parse_quote, parse_file, Expr, Pat, PatIdent };
use quote::quote;

struct Obfuscator {
    loop_counter: u32,
}

#[cfg(test)]
mod flow_tests;

impl Obfuscator {
    fn new() -> Self {
        Self { loop_counter: 0 }
    }
    fn flow_obfuscate(&mut self, code: &str) -> String {
        let ast = parse_file(code).expect("Failed to parse code");
        let mut modified_ast = ast.clone();
        self.visit_file_mut(&mut modified_ast);
        let modified_code = quote!(#modified_ast).to_string();
        modified_code
    }
    //check to see if statement in block is dummy loop
    fn is_dummy_loop(stmt: &Stmt) -> bool {
        if let Stmt::Expr(Expr::Block(expr_block), _) = stmt {
            for stmt in &expr_block.block.stmts {
                if let Stmt::Local(local) = stmt {
                    if let Pat::Ident(PatIdent { ident, .. }) = &local.pat {
                        if ident == "_is_dummy_145" {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}

impl VisitMut for Obfuscator {
    fn visit_block_mut(&mut self, block: &mut Block) {
        //check if the block already contains the dummy loop
        if block.stmts.iter().any(|stmt| Self::is_dummy_loop(stmt)) || self.loop_counter % 3 != 0 {
            self.loop_counter += 1;
            return;
        }

        let dummy_loop: Stmt = parse_quote! {
            {
                let _is_dummy_145 = true;
                let mut _dummy_counter = 0;
                let _dummy_upper_bound = 100;
                loop {
                    if
                        (_dummy_counter % 13 == 0 && _dummy_counter % 7 == 0) ||
                        _dummy_counter > _dummy_upper_bound
                    {
                        break;
                    }
                    _dummy_counter += 1;
                }
            }
        };
        //insert loop at start of every block
        block.stmts.insert(0, dummy_loop);
        self.loop_counter += 1;
        syn::visit_mut::visit_block_mut(self, block);
    }
}