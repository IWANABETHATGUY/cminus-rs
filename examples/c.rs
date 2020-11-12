use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{
    Buffer, BufferedStandardStream, ColorChoice, StandardStream,
};
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    term,
};
fn main() -> Result<(), std::io::Error> {
    // `files::SimpleFile` and `files::SimpleFiles` help you get up and running with
    // `codespan-reporting` quickly! More complicated use cases can be supported
    // by creating custom implementations of the `files::Files` trait.

    let mut files = SimpleFiles::new();

    let file_id = files.add(
        "FizzBuzz.fun",
        unindent::unindent(
            r#"
            module FizzBuzz where

            fizz₁ : Nat → String
            fizz₁ num = case (mod num 5) (mod num 3) of
                0 0 => "FizzBuzz"
                0 _ => "Fizz"
                _ 0 => "Buzz"
                _ _ => num

            fizz₂ : Nat → String
            fizz₂ num =
                case (mod num 5) (mod num 3) of
                    0 0 => "FizzBuzz"
                    0 _ => "Fizz"
                    _ 0 => "Buzz"
                    _ _ => num
        "#,
        ),
    );

    // We normally recommend creating a custom diagnostic data type for your
    // application, and then converting that to `codespan-reporting`'s diagnostic
    // type, but for the sake of this example we construct it directly.

    let diagnostic = Diagnostic::error()
        .with_message("`case` clauses have incompatible types")
        .with_code("E0308")
        .with_labels(vec![
            Label::primary(file_id, 328..331).with_message("expected `String`, found `Nat`"),
            Label::secondary(file_id, 211..331)
                .with_message("`case` clauses have incompatible types"),
            Label::secondary(file_id, 258..268)
                .with_message("this is found to be of type `String`"),
            Label::secondary(file_id, 284..290)
                .with_message("this is found to be of type `String`"),
            Label::secondary(file_id, 306..312)
                .with_message("this is found to be of type `String`"),
            Label::secondary(file_id, 186..192).with_message("expected type `String` found here"),
        ])
        .with_notes(vec![unindent::unindent(
            "
            expected type `String`
                found type `Nat`
        ",
        )]);

    // We now set up the writer and configuration, and then finally render the
    // diagnostic to standard error.

    let mut writer = Buffer::no_color();
    let config = codespan_reporting::term::Config::default();

    term::emit(&mut writer, &config, &files, &diagnostic)?;
    println!("{}", std::str::from_utf8(writer.as_slice()).unwrap());
    Ok(())
}
