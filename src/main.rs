mod parse;
#[macro_use] extern crate prettytable;

use clap::{App, Arg, SubCommand};
use prettytable::{Cell, Row, Table};
use std::fs;

fn main() {
    let mut app = App::new("book-mode")
        .version("0.1")
        .author("Katherina Fey. <kookie@spacekookie.de>")
        .about("A ledger for books")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Read books from FILE")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("list").about("list books"));
    let matches = app.clone().get_matches();

    let file = matches.value_of("file").expect("Missing books file");
    let contents = fs::read_to_string(file).expect("Failed to read books file");

    let books = parse::Books::parse(contents.as_str());

    match matches.subcommand_name() {
        Some("list") => list_books_cmd(&books),
        _ => app.print_help().unwrap(),
    }
}

fn list_books_cmd(books: &parse::Books) {
    let mut table = Table::new();

    let rows = books.inner.iter().map(|book| {
        let genre = book.genre.clone().unwrap_or("-".into());
        let isbn = book.isbn.clone().unwrap_or("-".into());

        Row::new(vec![
            Cell::new(&book.name),
            Cell::new(&genre),
            Cell::new(&isbn),
        ])
    });

    table.set_titles(row!["Title", "Genre", "ISBN"]);

    for row in rows {
        table.add_row(row);
    }

    table.printstd();
}
