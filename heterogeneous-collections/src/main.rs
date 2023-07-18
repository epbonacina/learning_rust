trait PrettyPrintable {
    fn pretty_print(&self) -> String;
}

struct Foo {
    name: String,
    foo_id: u32,
}


struct Bar {
    name: String,
    bar_id: u32,
}

impl PrettyPrintable for Foo {
    fn pretty_print(&self) -> String {
        return format!("[FOO {}] ** {} **", self.foo_id, self.name);
    }
}

impl PrettyPrintable for Bar {
    fn pretty_print(&self) -> String {
        return format!("[BAR {}] == {} ==", self.bar_id, self.name);
    }
}

// fn print_pretty_printables(printables: Vec<&dyn PrettyPrintable>) {
fn print_pretty_printables(printables: &[&dyn PrettyPrintable]) {
    for printable in printables {
        println!("{}", printable.pretty_print());
    }
}

fn main() {
    let f = Foo {name: "Fooled".to_string(), foo_id: 110};
    let b = Bar {name: "Barred".to_string(), bar_id: 220};

    let printables: Vec<&dyn PrettyPrintable> = vec![&f, &b];

    print_pretty_printables(&printables);
}
