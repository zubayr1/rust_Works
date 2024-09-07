pub fn print_elements(elements: &[String]) {
    for item in elements {
        println!("{:?}", item);
    }
}

fn shorten_strings(element: &mut String) -> &str {
    element
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter().map(|el| el.to_uppercase()).collect()
}

pub fn print_elements_adapt(elements: &[String]) {
    elements.iter().for_each(|el| {
        println!("Without adapt: {}", el);
    });

    let elements1 = to_uppercase(&elements);

    elements1
        .into_iter()
        .map(|el| el.to_owned() + " " + &el)
        .for_each(|el| {
            // el.truncate(1);
            println!("With adapt: {}", el);
        });
}
