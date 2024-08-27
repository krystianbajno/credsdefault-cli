use colored::*;
use crate::dataset::CredentialEntry;

pub fn print_results(results: &[CredentialEntry]) {
    for entry in results {
        println!("{}: {}", "Manufacturer:".blue(), entry.manufacturer.green());
        println!("{}: {}", "Model:".blue(), entry.model.green());
        println!("{}: {}", "Version:".blue(), entry.version.green());
        println!("{}: {}", "Role:".blue(), entry.role.green());
        println!("{}: {}", "Login".blue(), entry.login.green());
        println!("{}: {}", "Password".blue(), entry.password.green());
        println!("{}: {}", "Method:".blue(),  entry.method.green());
        println!("{}: {}", "Source:".blue(), entry.source.green());
        println!("{}: {}", "Comment:".blue(), entry.comment.green());
        println!("{}: {}", "Port:".blue(), entry.port.to_string().green());
        println!("{}: {}", "Address:".blue(), entry.address.green());
        println!("{}", "\n------------------------\n".yellow());
    }
}
