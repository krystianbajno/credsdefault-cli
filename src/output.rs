use colored::*;
use crate::dataset::CredentialEntry;

pub fn print_results(results: &[CredentialEntry]) {
    for entry in results {
        let fields = [
            ("Manufacturer", &entry.manufacturer),
            ("Model", &entry.model),
            ("Version", &entry.version),
            ("Role", &entry.role),
            ("Login", &entry.login),
            ("Password", &entry.password),
            ("Method", &entry.method),
            ("Source", &entry.source),
            ("Comment", &entry.comment),
            ("Port", &entry.port),
            ("Address", &entry.address),
        ];

        for (label, value) in &fields {
            println!("{}: {}", label.blue(), value.green());
        }

        println!("{}", "\n------------------------\n".yellow());
    }
}
