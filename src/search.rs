use crate::dataset::CredentialEntry;

pub fn search_dataset(dataset: &[CredentialEntry], keywords: &[String]) -> Vec<CredentialEntry> {
    dataset
        .iter()
        .filter(|entry| {
            keywords.iter().any(|kw| {
                let kw_lower = kw.to_lowercase();
                entry.manufacturer.to_lowercase().contains(&kw_lower)
                    || entry.model.to_lowercase().contains(&kw_lower)
                    || entry.version.to_lowercase().contains(&kw_lower)
                    || entry.role.to_lowercase().contains(&kw_lower)
                    || entry.login.to_lowercase().contains(&kw_lower)
                    || entry.password.to_lowercase().contains(&kw_lower)
                    || entry.method.to_lowercase().contains(&kw_lower)
                    || entry.source.to_lowercase().contains(&kw_lower)
                    || entry.comment.to_lowercase().contains(&kw_lower)
                    || entry.port.to_lowercase().contains(&kw_lower)
                    || entry.address.to_lowercase().contains(&kw_lower)
            })
        })
        .cloned()
        .collect()
}
