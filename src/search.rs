use crate::dataset::CredentialEntry;
use std::collections::HashSet;

pub fn search_dataset(dataset: &[CredentialEntry], keywords: &[String]) -> Vec<CredentialEntry> {
    let keywords_set: HashSet<String> = keywords.iter().cloned().collect();
    dataset
        .iter()
        .filter(|entry| {
            keywords_set.iter().any(|kw| {
                entry.manufacturer.contains(kw)
                    || entry.model.contains(kw)
                    || entry.version.contains(kw)
                    || entry.role.contains(kw)
                    || entry.login.contains(kw)
                    || entry.password.contains(kw)
                    || entry.method.contains(kw)
                    || entry.source.contains(kw)
                    || entry.comment.contains(kw)
                    || entry.port.contains(kw)
                    || entry.address.contains(kw)
            })
        })
        .cloned()
        .collect()
}
