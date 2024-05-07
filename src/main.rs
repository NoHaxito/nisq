use clap::{arg, Command};
use console::style;

fn main() {
    let matches = Command::new("nisq")
        .version("1.0")
        .about("Lorem ipsum")
        .arg(arg!([directory] "Lorem ipsum dolor sit amet"))
        .arg(
            arg!(
                -t --template <NAME> "Template name from templates directory"
            )
            .required(false),
        )
        .get_matches();

    match matches.args_present() {
        true => {
            let template = matches.get_one::<String>("template");
            let directory = matches.get_one::<String>("directory");
            let _result = cmd(directory, template);
        }
        false => {
            println!("No one argument present");
        }
    }
}

fn cmd(directory: Option<&String>, template: Option<&String>) -> std::io::Result<()> {
    cliclack::intro(style(" nisq ").on_cyan().black())?;
    let arguments_overview = format!(
        "{dir}{tmpl}",
        dir = if !directory.is_none() {
            format!("Directory: {:?}\n", directory.unwrap())
        } else {
            "".to_string()
        },
        tmpl = if !template.is_none() {
            format!("Template: {:?}", template.unwrap())
        } else {
            "".to_string()
        }
    );
    cliclack::note("Arguments", arguments_overview)?;
    cliclack::outro(format!("Done �"))?;
    Ok(())
}
