use calc::syntax;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
        println!("{:#?}", syntax::parser::parser(&args[1]));
    }
}
