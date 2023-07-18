trait Named {
    fn get_name(&self) -> String;
    fn update_name(&mut self, new_name: &str);
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
    fn update_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
}

impl Named for Bar {
    fn get_name(&self) -> String {
        return format!("Bar {}", self.name);
    }
    fn update_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
}

// fn handle_named_object<T>(named_object: &T)
// where T: Named {
fn print_named_object<T: Named>(named_object: &T) {
    println!("[handle_named_object] {}", named_object.get_name());
}

fn print_named_objects<T: Named>(named_objects: &[&T]) {
    for (i, named) in named_objects.iter().enumerate() {
        println!("{}. {}", i, named.get_name());
    }
}

fn transform_name(named_object: &mut impl Named) {
    let parts: Vec<String> = named_object.get_name()
        .split_ascii_whitespace()
        .map(|part| part.to_string())
        .collect();
    named_object.update_name(&parts.get(1).unwrap().to_uppercase());
}

fn generic_transform_name<T: Named>(named_object: &mut T) {
    let parts: Vec<String> = named_object.get_name()
        .split_ascii_whitespace()
        .map(|part| part.to_string())
        .collect();
    named_object.update_name(&parts.get(1).unwrap().to_uppercase());
}

fn main() {
    let mut foo_1 = Foo { name: "Foolish".to_string() };
    let mut foo_2 = Foo { name: "Foorish".to_string() };
    let mut bar_1 = Bar { name: "Barring".to_string() };

    print_named_object(&foo_1);
    print_named_object(&bar_1);

    println!();

    // let named_objects: Vec<&dyn Named> = vec![&foo, &bar];
    print_named_objects(&[&foo_1, &foo_2]);
    print_named_objects(&[&bar_1]);

    println!();

    transform_name(&mut foo_1);
    transform_name(&mut foo_2);
    transform_name(&mut bar_1);
    println!("Transformed foo_1: {}", foo_1.get_name());
    println!("Transformed foo_2: {}", foo_2.get_name());
    println!("Transformed bar_1: {}", bar_1.get_name());

    println!();

    let mut foo_3 = Foo { name: "Fookish".to_string() };
    let mut bar_2 = Bar { name: "Barting".to_string() };

    print_named_object(&foo_3);
    print_named_object(&bar_2);

    generic_transform_name(&mut foo_3);
    generic_transform_name(&mut bar_2);

    println!();

    print_named_object(&foo_3);
    print_named_object(&bar_2);
}
