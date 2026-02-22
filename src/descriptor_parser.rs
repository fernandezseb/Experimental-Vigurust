
pub struct Descriptor {
    pub args: Vec<String>,
    pub return_type: String
}

fn split_descriptor(descriptor: &str) -> (&str, &str) {
    let index = descriptor.find(')').unwrap();
    (&descriptor[1..index], &descriptor[(index+1)..])
}

fn get_types(descriptor: &str) -> Vec<String> {
    let mut args = Vec::new();
    
    let mut i: usize = 0;
    let mut array_count: usize = 0;

    while i < descriptor.len() {
        let c = descriptor.chars().nth(i).unwrap();
        //println!("{}",c);
        match c {
            'L' => {
                let location = (&descriptor[i..]).find(';').unwrap();
                let class = &descriptor[(i+1)..(i+location)];
                let token = format!("L{}{};", class, std::iter::repeat("[").take(array_count).collect::<String>());
                args.push(token);
                array_count = 0;
                i += class.len() + 2;
            },
            '[' => {
                array_count += 1;
                i += 1;
            },
            _ => {
                let token = format!("{}{}", c, std::iter::repeat("[").take(array_count).collect::<String>());
                args.push(token);
                array_count = 0;
                i += 1;
            }
        }
    }

    args
}

pub fn parse_descriptor(descriptor: String) -> Descriptor {
    let (args_part, return_part) = split_descriptor(&descriptor);
    let args = get_types(args_part);
    let return_type = String::from(get_types(return_part).get(0).unwrap());
    //println!("{} {} {}", descriptor, args_part, return_part);
    Descriptor{args: args, return_type: return_type}
}