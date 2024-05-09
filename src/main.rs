use std::fs;

use clap::{arg, Command};
use cliclack::{confirm, input, intro, note, outro, select};
use console::style;
mod utils;
use utils::{check_if_directory_empty, check_if_package_manager_installed};

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
        .arg(
            arg!(
                -p --package <NAME> "Package manager to use."
            )
            .required(false),
        )
        .get_matches();

    match matches.args_present() {
        true => {
            let template = matches.get_one::<String>("template");
            let directory = matches.get_one::<String>("directory");
            let package_manager = matches.get_one::<String>("package");
            let _result = cmd(directory, template, package_manager);
        }
        false => {
            let _result = cmd(None, None, None);
        }
    }
}

fn cmd(dir: Option<&String>, tmpl: Option<&String>, pkgm: Option<&String>) -> std::io::Result<()> {
    ctrlc::set_handler(move || {}).expect("setting Ctrl-C handler");
    intro(style(" nisq ").on_cyan().black())?;

    let package_managers = vec![
        ("npm", "npm", ""),
        ("yarn", "yarn", ""),
        ("pnpm", "pnpm", ""),
        ("bun", "bun", ""),
    ];

    let templates_directory = fs::read_dir("./templates")?;
    let mut templates = vec![];

    for entries in templates_directory {
        let entry_name = entries.unwrap().file_name().to_str().unwrap().to_string();
        templates.push((entry_name.clone(), entry_name, "".to_string()));
    }

    let mut directory = if !dir.is_none() {
        dir.unwrap().to_string()
    } else {
        "".to_string()
    };
    let mut template = if !tmpl.is_none() {
        tmpl.unwrap().to_string()
    } else {
        "".to_string()
    };
    let mut package_manager = if !pkgm.is_none() {
        pkgm.unwrap().to_string()
    } else {
        "".to_string()
    };
    let mut overwrite_dir = false;
    if directory.is_empty() {
        directory = input("Where should we create your project?")
            .placeholder("./sparkling-solid")
            .validate(|input: &String| {
                if input.is_empty() {
                    Err("Please enter a path.")
                } else if !input.starts_with(".") {
                    Err("Please enter a relative path")
                } else {
                    Ok(())
                }
            })
            .interact()?;
    } else {
        // validate if directory is empty or not
        if !check_if_directory_empty(directory.clone()).unwrap() {
            overwrite_dir = confirm(format!(
                " Directory {directory} is not empty. Do you want to overwrite it? "
            ))
            .interact()?;
        }
        // note("Directory", directory.clone())?;
    }

    if template.is_empty() {
        template = select(format!("Wich template you want to use?"))
            .items(&templates)
            .interact()?
            .to_string();
    } else {
        // validate if template exists or not
        let exists =
            templates.contains(&(template.to_string(), template.to_string(), "".to_string()));

        if !exists {
            cliclack::outro(
                style(format!(
                    " Template {template} not found, please select a valid template "
                ))
                .white()
                .on_red(),
            )?;
            panic!("");
        } else {
            // note("Template", template.clone())?;
        }
    }
    let run_install = confirm("Do you want to install dependencies?")
        .initial_value(true)
        .interact()?;
    if run_install && package_manager.is_empty() {
        package_manager = select("Select wich package manager you want to use:")
            .items(&package_managers)
            .interact()?
            .to_string();
        // run install dependencies command here
    } else if run_install && !package_manager.is_empty() {
        // run install dependencies command here
        let exists = package_managers.contains(&(&package_manager, &package_manager, ""));

        if !exists {
            outro(
                style(format!(
                    " Package manager {package_manager} not found, please select a valid package manager "
                ))
                .white()
                .on_red(),
            )?;
            panic!("");
        } else {
            // check if package manager is installed and throw if not
            let is_installed = check_if_package_manager_installed(&package_manager).unwrap();
            if !is_installed {
                cliclack::outro(
                    style(format!(
                        " Package manager {package_manager} is not installed, please install it "
                    ))
                    .on_red()
                    .white(),
                )?;
                panic!("");
            }
            // note("Package manager", package_manager.clone())?;
        }
    }

    let arguments_overview = format!(
        "Directory: {directory}\nOverwrite directory: {overwrite_dir}\nTemplate: {template}\nInstall dependencies: {run_install}\nPackage manager: {package_manager}",
    );
    note("Arguments", arguments_overview)?;
    outro(format!("Done ✨"))?;

    Ok(())
}
