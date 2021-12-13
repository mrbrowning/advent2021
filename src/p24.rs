use super::p23::*;
use super::util::*;

pub fn run(args: &[String]) {
    let file = get_file(args, 24).unwrap();
    let cavern_graph = parse_cavern_graph(file);

    let paths = cavern_graph.all_paths(true);
    println!("Answer is {}", paths.len());
}
