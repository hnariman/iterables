fn main() {
    imperative_arrays();
    declarative_arrays();

    imperative_vectors();
    declarative_vectors();

    map_investigation();

    let closure = |val: u8| format!("\nPerfect formatting for casted: {}", (12 + val) as i32);
    println!("{}", closure(23));

    // Haskel style folding
    println!(
        "Ugly but declarative: {}",
        (1..3).map(|x| x * 2).fold(0, |a, b| a + b)
    );
}

fn imperative_arrays() {
    let list: [&str; 5] = ["A", "B", "C", "D", "E"];

    println!("\nImperative way");

    // array borowing is smooth, because we're holding data in stack memory
    for each in list {
        println!("And the Oscar goes to ... {}", each);
    }
    println!("Population of our program: {:?}", list);
}

fn declarative_arrays() {
    let list: [&str; 5] = ["A", "B", "C", "D", "E"];
    println!("\nDeclarative way");
    list.map(|each| println!("{}", each));
    println!("Population of our program: {:?}", list);
}

fn imperative_vectors() {
    let list = vec!["A", "B", "C", "D"];
    println!("\nImperative way");

    for each in &list {
        println!("Letter ... {}", each);
    }

    println!("list {:?}", list);
}

fn declarative_vectors() {
    let list = vec!["A", "B", "C", "D"];
    println!("\nDeclarative way - vectors");
    // have to disable this variable since map returns Map
    let _ = list.iter().map(|each| println!(" isn't she {} ?", each));
    println!("Element: {:?}", list);
}

fn map_investigation() {
    let legends: [&str; 3] = ["A", "B", "C"];
    let result = legends.map(|one| format!("Letter: {}", one));
    println!("{:?}", result);
}

