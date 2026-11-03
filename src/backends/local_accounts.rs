#[derive(Debug, PartialEq)]
pub struct LocalAccount {
    pub username: String,
    pub name: String,
    pub uid: u32,
    pub system: bool,
}

// Read the local database, never NSS/getent: SSSD/winbind accounts must not appear.
pub fn list() -> Result<Vec<LocalAccount>, String> {
    let passwd = std::fs::read_to_string("/etc/passwd")
        .map_err(|_| "Não foi possível consultar as contas locais.".to_string())?;
    let defs = std::fs::read_to_string("/etc/login.defs").unwrap_or_default();
    let minimum = defs
        .lines()
        .find_map(|line| {
            let mut fields = line.split_whitespace();
            (fields.next()? == "UID_MIN")
                .then(|| fields.next()?.parse::<u32>().ok())
                .flatten()
        })
        .unwrap_or(1000);
    Ok(parse(&passwd, minimum))
}

fn parse(text: &str, minimum: u32) -> Vec<LocalAccount> {
    let mut accounts: Vec<_> = text
        .lines()
        .filter_map(|line| {
            let fields: Vec<_> = line.split(':').collect();
            if fields.len() != 7 || fields[0].is_empty() || fields[0].starts_with(['+', '-']) {
                return None;
            }
            let uid = fields[2].parse().ok()?;
            Some(LocalAccount {
                username: fields[0].into(),
                name: fields[4].split(',').next().unwrap_or_default().into(),
                uid,
                system: uid < minimum || uid == 65534,
            })
        })
        .collect();
    accounts.sort_by(|a, b| (a.system, &a.username).cmp(&(b.system, &b.username)));
    accounts
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn enumerates_only_local_records_and_preserves_service_accounts() {
        let text = "root:x:0:0:root:/root:/bin/bash\nalice:x:1000:1000:Alice,Room:/home/alice:/bin/bash\n+aduser::::::\ninvalid\nnobody:x:65534:65534::/:/usr/sbin/nologin\n";
        let accounts = parse(text, 1000);
        assert_eq!(accounts.len(), 3);
        assert_eq!(accounts[0].name, "Alice");
        assert!(!accounts[0].system);
        assert!(accounts[1..].iter().all(|a| a.system));
    }
}
