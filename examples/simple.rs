use yansi::Color;
use ariadne::{Report, ReportKind, Label, Source, Config};

fn main() {
    Report::build(ReportKind::Error, (), 34)
        .with_message("Incompatible types")
        .with_label(Label::new(32..33).with_message("This is of type Nat"))
        .with_label(Label::new(42..45).with_message("This is of type Str"))
        .finish()
        .print(Source::from(include_str!("sample.tao")))
        .unwrap();

    const SOURCE: &str = "a b c d e f";
    // also supports labels with no messages to only emphasis on some areas
    Report::build(ReportKind::Error, (), 34)
        .with_message("Incompatible types")
        .with_config(Config::default().with_compact(true))
        .with_label(Label::new(0..1).with_color(Color::Red))
        .with_label(Label::new(2..3).with_color(Color::Blue).with_message("This is another test").with_order(1))
        .with_label(Label::new(4..5).with_color(Color::Green))
        .with_label(Label::new(7..9).with_color(Color::Cyan).with_message("This is a test"))
        .finish()
        .print(Source::from(SOURCE))
        .unwrap();
}
