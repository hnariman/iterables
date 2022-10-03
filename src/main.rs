fn main() {
    imperative_arrays();
    declarative_arrays();

    imperative_vectors();
    declarative_vectors();

    map_investigation();
    inspecting_inspect();

    let closure = |val: u8| format!("\nPerfect formatting for casted: {}", (12 + val) as i32);
    println!("{}", closure(23));

    // Haskel style folding
    println!(
        "Ugly but declarative: {}",
        (1..3).map(|x| x * 2).fold(0, |a, b| a + b)
    );
}

fn imperative_arrays() {
    let boyband: [&str; 5] = ["John", "Joe", "Jack", "James", "Jeremy"];

    println!("\nImperative way");

    // array borowing is smooth, because we're holding data in stack memory
    for each in boyband {
        println!("And the Oscar goes to ... {}", each);
    }
    println!("Population of our program: {:?}", boyband);
}

fn declarative_arrays() {
    let boyband: [&str; 5] = ["John", "Joe", "Jack", "James", "Jeremy"];
    println!("\nDeclarative way");
    boyband.map(|each| println!("{}", each));
    println!("Population of our program: {:?}", boyband);
}

fn imperative_vectors() {
    let ladies = vec!["Jane", "Joan", "Jennifer", "Jolinda"];
    println!("\nImperative way");

    for each in &ladies {
        println!("Cutie .. {}", each);
    }

    println!("All the single ladies, {:?}", ladies);
}

fn declarative_vectors() {
    let ladies = vec!["Jane", "Joan", "Jennifer", "Jolinda"];
    println!("\nDeclarative way - vectors");
    // have to disable this variable since map returns Map
    let _ = ladies.iter().map(|each| println!(" isn't she {} ?", each));
    println!("All the single ladies, {:?}", ladies);
}

fn map_investigation() {
    let legends: [&str; 3] = ["A", "B", "C"];
    let result = legends.map(|one| format!("Letter: {}", one));
    println!("{:?}", result);
}

fn inspecting_inspect() {
    let list = vec![1, 2, 3, 4];
    println!("\n\nHere comes the inspector:");
    list.iter().map(|x| println!("is a {}", x));
}
