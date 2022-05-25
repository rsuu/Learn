use std::error::Error;
use std::fs::File;
use std::io::Read;
fn main() {
    run().unwrap();
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<_>>();

    let mut file = File::open(&args[1])?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let ast = syn::parse_file(&content)?;
    if let Some(ref shebang) = ast.shebang {
        println!("{}", shebang);
    }
    println!("{} items", &ast.items.len());
    println!("{:#?}", &ast);

    Ok(())
}
