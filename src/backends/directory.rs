use gtk::prelude::*;
use serde::Serialize;
use std::{
    io::Write,
    process::{Command, Stdio},
};
pub const HELPER: &str = "/usr/libexec/monolito-directory-helper";
pub const SSSD_CONFIG: &str = "/etc/sssd/sssd.conf";
#[derive(Serialize)]
pub struct JoinRequest {
    pub domain: String,
    pub administrator: String,
    pub password: String,
    pub unqualified_names: bool,
    pub allow_weak_crypto: bool,
    pub allow_rc4: bool,
}
pub fn valid_domain(s: &str) -> bool {
    s.len() <= 253
        && s.contains('.')
        && s.split('.').all(|p| {
            !p.is_empty()
                && p.len() <= 63
                && !p.starts_with('-')
                && !p.ends_with('-')
                && p.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        })
}
pub fn discover(domain: &str) -> Result<String, String> {
    if !valid_domain(domain) {
        return Err("Informe um domínio DNS válido, como empresa.exemplo.com".into());
    }
    super::command("realm", &["discover", "--name-only", domain])
}
pub fn status() -> Result<String, String> {
    super::command("realm", &["list", "--name-only"])
}
pub fn available() -> bool {
    std::path::Path::new(HELPER).is_file()
}
pub fn open_sssd_editor() -> Result<String, String> {
    if !std::path::Path::new(SSSD_CONFIG).is_file() {
        return Err("O arquivo sssd.conf ainda não existe.".into());
    }
    let context = gtk::gdk::Display::default().map(|display| display.app_launch_context());
    gtk::gio::AppInfo::launch_default_for_uri(
        "admin:///etc/sssd/sssd.conf",
        context.as_ref(),
    )
    .map_err(|_| {
        "Não foi possível abrir o editor administrativo. Verifique GVfs e o editor de texto padrão."
            .to_string()
    })?;
    Ok("Editor administrativo aberto para sssd.conf.".into())
}
pub fn join(request: JoinRequest) -> Result<String, String> {
    if !valid_domain(&request.domain)
        || request.administrator.trim().is_empty()
        || request.password.is_empty()
    {
        return Err("Preencha domínio, administrador e senha.".into());
    }
    let mut child = Command::new("pkexec")
        .arg(HELPER)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| "Não foi possível iniciar a autorização polkit.")?;
    let data = serde_json::to_vec(&request).map_err(|e| e.to_string())?;
    child
        .stdin
        .take()
        .ok_or("Entrada indisponível")?
        .write_all(&data)
        .map_err(|_| "A autorização foi cancelada ou recusada.")?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok("Ingresso concluído. Encerre a sessão antes de usar sua conta de domínio.".into())
    } else if out.status.code() == Some(10) {
        Err("O ingresso ocorreu, mas a configuração local falhou. Consulte o administrador antes de tentar novamente; o computador pode já pertencer ao domínio.".into())
    } else {
        Err("Ingresso não concluído. Verifique autorização, credenciais, DNS e horário.".into())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn domain_validation() {
        for s in ["a.example", "EXAMPLE.COM", "dc-1.empresa.com"] {
            assert!(valid_domain(s));
        }
        for s in [
            "--help",
            "bad domain.com",
            "a..com",
            "-a.com",
            "a.com\n",
            "a/.com",
            "",
        ] {
            assert!(!valid_domain(s));
        }
    }
}
