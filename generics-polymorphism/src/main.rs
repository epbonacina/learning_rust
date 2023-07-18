trait Named {
    fn get_name(&self) -> String;
}

struct Foo {
    name: String,
}

struct Bar {
    name: String,
}

impl Named for Foo {
    fn get_name(&self) -> String {
        return format!("Foo {}", self.name);
    }
}

impl Named for Bar {
    fn get_name(&self) -> String {
        return format!("Bar {}", self.name);
    }
}

// fn handle_named_object<T>(named_object: &T)
// where T: Named {
fn handle_named_object<T: Named>(named_object: &T) {
    println!("[handle_named_object] {}", named_object.get_name());
}

fn handle_named_objects<T: Named>(named_objects: &[&T]) {
    for (i, named) in named_objects.iter().enumerate() {
        println!("{}. {}", i, named.get_name());
    }
}

fn transform_name(named_object: &impl Named) -> impl Named {
    let parts: Vec<String> = named_object.get_name()
        .split_ascii_whitespace()
        .map(|part| part.to_string())
        .collect();
    let transformed = parts.get(1).unwrap().to_uppercase();
    let determinant = named_object.get_name().chars().next().unwrap(); 
    match determinant {
        'F' => return Foo { name: transformed },
        _ => return Foo { name: "Unknown".to_string() }
    }
}

fn main() {
    let foo_1 = Foo { name: "Foolish".to_string() };
    let foo_2 = Foo { name: "Foorish".to_string() };
    let bar_1 = Bar { name: "Barring".to_string() };

    handle_named_object(&foo_1);
    handle_named_object(&bar_1);

    println!();

    // let named_objects: Vec<&dyn Named> = vec![&foo, &bar];
    handle_named_objects(&[&foo_1, &foo_2]);
    handle_named_objects(&[&bar_1]);

    println!();

    let transformed_foo_1 = transform_name(&foo_1);
    let transformed_bar_1 = transform_name(&bar_1);
    println!("Transformed foo_1: {}", transformed_foo_1.get_name());
    println!("Transformed bar_1: {}", transformed_bar_1.get_name());
}
