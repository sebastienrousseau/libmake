use libmake::generator::generate_from_args;

fn main() -> std::io::Result<()> {
    let args = "--author=Me --output=my_library"
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Usage: {} <args>", args[0]);
        return Ok(());
    }

    let args_str = args[1..].join(" ");
    let result = generate_from_args(&args_str);

    match result {
        Ok(()) => println!("Successfully generated files!"),
        Err(err) => eprintln!("Error: {}", err),
    }

    Ok(())
}
