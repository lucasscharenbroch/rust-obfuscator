use syn::{ visit_mut::VisitMut, Block, Stmt, parse_file, Expr, Pat, PatIdent, parse_quote };
use quote::quote;
use rand::{ Rng, thread_rng };
use rand::seq::SliceRandom;

struct FlowObfuscator {
    loop_counter: u32,
}

#[cfg(test)]
mod flow_tests;

impl FlowObfuscator {
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
    //helper to generate random dummy loop
    fn generate_dummy_loop() -> Stmt {
        let mut rng = thread_rng();

        let initial_value = rng.gen_range(1..=10);
        let increment_value = rng.gen_range(1..=5);
        let add_extra_dummy_variable = rng.gen_bool(0.5);

        let mut statements = vec![
            quote! { let mut _dummy_counter = #initial_value; },
            quote! { let _dummy_increment = #increment_value; },
            quote! { let _dummy_upper_bound = 100; }
        ];

        //add extra dummy variable occasionally
        if add_extra_dummy_variable {
            let extra_dummy_value = rng.gen_range(1..=10);
            statements.push(quote! { let _extra_dummy_var = #extra_dummy_value; });
        }

        //randomize the order of variable assignments
        statements.shuffle(&mut rng);

        let loop_block =
            quote! {
        loop {
            if _dummy_counter > _dummy_upper_bound{
                break;
            }
            //prevent compiler optimizations
            unsafe {
                std::ptr::write_volatile(&mut _dummy_counter, _dummy_counter + _dummy_increment);
            }
        }
    };

        parse_quote! {
        {
            let _is_dummy_145 = true;
            #(#statements)*
            #loop_block
        }
    }
    }
}

impl VisitMut for FlowObfuscator {
    fn visit_block_mut(&mut self, block: &mut Block) {
        //check if the block already contains the dummy loop
        if block.stmts.iter().any(|stmt| Self::is_dummy_loop(stmt)) || self.loop_counter % 3 != 0 {
            self.loop_counter += 1;
            return;
        }

        let dummy_loop = Self::generate_dummy_loop();
        //insert loop at start of every block
        block.stmts.insert(0, dummy_loop);
        self.loop_counter += 1;
        syn::visit_mut::visit_block_mut(self, block);
    }
}
