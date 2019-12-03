use ego_tree::iter::Edge::*;
use scraper::{Html, Node, Selector};

struct State {
    code: bool,
    small_code: bool,
}

fn main() {
    let mut args = std::env::args();
    args.next(); // name of the binary
    let url = args.next().expect("give me an url as argument");

    let html = reqwest::get(&url).unwrap().text().unwrap();

    let fragment = Html::parse_fragment(&html);
    let selector = Selector::parse(".day-desc").unwrap();

    let article = match fragment.select(&selector).next() {
        Some(el) => el,
        None => {
            println!("Error in the page");
            return;
        }
    };

    let mut state = State {
        code: false,
        small_code: false,
    };

    for element in article.traverse() {
        match element {
            Open(el) => handle_open(&mut state, el.value()),
            Close(el) => handle_close(&mut state, el.value()),
        }
    }
}

fn handle_open(state: &mut State, node: &Node) {
    let el = match node {
        Node::Element(el) => el,
        Node::Text(t) => {
            print!("{}", t.text.trim());
            return;
        }
        n => panic!("Unknown open node: {:?}", n),
    };
    if state.code || state.small_code {
        return;
    }
    match el.name() {
        "article" => (),
        "h2" => print!("## "),
        "p" => print!("\n"),
        "a" => print!(" ["),
        "span" => print!(" *"),
        "em" => print!(" **"),
        "code" => {
            print!(" `");
            state.small_code = true;
        }
        "pre" => {
            print!("```\n");
            state.code = true;
        }
        "ul" => print!("\n"),
        "li" => print!("- "),
        el => panic!("Unknown open element: {:?}", el),
    }
}

fn handle_close(state: &mut State, node: &Node) {
    let el = match node {
        Node::Element(el) => el,
        Node::Text(_) => return,
        n => panic!("Unknown close node: {:?}", n),
    };
    // this will work only for well formed documents
    if state.code | state.small_code {
        match el.name() {
            "pre" => {
                print!("\n```\n");
                state.code = false;
            }
            "code" if state.code == false => {
                print!("` ");
                state.small_code = false;
            }
            _ => (),
        }
    } else {
        match el.name() {
            "article" => (),
            "h2" => print!("\n"),
            "p" => print!("\n"),
            "a" => print!("]({}) ", el.attr("href").unwrap_or("invalid link")),
            "span" => print!("* "),
            "em" => print!("** "),
            "code" => print!("` "),
            "ul" => print!("\n"),
            "li" => print!("\n"),
            el => panic!("Unknown close element: {:?}", el),
        }
    }
}
