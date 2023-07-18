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

// fn print_pretty_printables(printables: &Vec<Box<dyn PrettyPrintable>>) {
fn print_pretty_printables(printables: &[Box<dyn PrettyPrintable>]) {
    for printable in printables {
        println!("{}", printable.pretty_print());
    }
}

fn make_pretty_printables() -> Vec<Box<dyn PrettyPrintable>> {
    let foo = Foo {name: "Fooled".to_string(), foo_id: 110};
    let bar = Bar {name: "Barred".to_string(), bar_id: 220};

    let printables: Vec<Box<dyn PrettyPrintable>> = vec![Box::new(foo), Box::new(bar)];
    printables
}

fn main() {
    let printables = make_pretty_printables();
    print_pretty_printables(&printables);
}
