#[derive(Debug)]
enum Lang {
    English,
    Spanish,
    Chinese,
    Texan,
    French,
    German,
}

struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
    let mut v: Vec<Greeting> = Vec::new();

    let g: Greeting = Greeting {
        lang: Lang::English,
        message: String::from("Hello WasmEdge!"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Spanish,
        message: String::from("Hola WasmEdge!"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Texan,
        message: String::from("Howdy WasmEdge!"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Chinese,
        message: String::from("WasmEdge 你好!"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::French,
        message: String::from("Bonjour WasmEdge!"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::German,
        message: String::from("guten tag WasmEdge!"),
    };
    v.push(g);

    for (_, e) in v.iter().enumerate() {
        println!("{:?} {}", e.lang, e.message);
    }
    match v
        .iter()
        .position(|greeting| matches!(greeting.lang, Lang::French))
    {
        Some(x) => println!("Found: {:?} {}", v[x].lang, v[x].message),
        None => println!("{:?} Not found", Lang::French),
    }
}
