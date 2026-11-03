use crate::{
    backends::{
        catalog, desktop_settings,
        directory::{self, JoinRequest},
        local_accounts,
    },
    components::*,
    i18n::tr,
};
use gtk::prelude::*;
use std::sync::mpsc;
fn task<F: FnOnce() -> Result<String, String> + Send + 'static>(
    status: gtk::Label,
    buttons: Vec<gtk::Button>,
    work: F,
) {
    let sensitivity: Vec<_> = buttons.iter().map(|b| b.is_sensitive()).collect();
    for b in &buttons {
        b.set_sensitive(false);
    }
    status.set_text(&tr("Operação em andamento…"));
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(work());
    });
    gtk::glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        match rx.try_recv() {
            Ok(result) => {
                status.set_text(&match result {
                    Ok(s) if s.is_empty() => tr("Nenhum domínio configurado."),
                    Ok(s) => s,
                    Err(e) => format!("{}: {e}", tr("Não foi possível concluir")),
                });
                for (b, sensitive) in buttons.iter().zip(&sensitivity) {
                    b.set_sensitive(*sensitive);
                }
                gtk::glib::ControlFlow::Break
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                status.set_text(&tr("A operação foi interrompida."));
                for (b, sensitive) in buttons.iter().zip(&sensitivity) {
                    b.set_sensitive(*sensitive);
                }
                gtk::glib::ControlFlow::Break
            }
            _ => gtk::glib::ControlFlow::Continue,
        }
    });
}
pub fn build() -> gtk::ScrolledWindow {
    let (scroll, body) = page("Contas", "Perfil de usuário e acesso a organizações.");
    body.append(&label("Usuários locais", "section-title"));
    let local = gtk::Box::new(gtk::Orientation::Vertical, 12);
    let local_status = label("", "secondary");
    populate_local(&local, &local_status);
    body.append(&local);
    let refresh_local = button("Atualizar usuários");
    let (l, s) = (local.clone(), local_status.clone());
    refresh_local.connect_clicked(move |_| populate_local(&l, &s));
    body.append(&refresh_local);
    body.append(&local_status);
    body.append(&label("Domínio corporativo", "section-title"));
    let card = card();
    card.append(&row(
        "Active Directory",
        "Ingresso do dispositivo no domínio da organização.",
        "security",
    ));
    let form = gtk::Box::new(gtk::Orientation::Vertical, 12);
    form.add_css_class("form");
    let hostname = gtk::Entry::builder()
        .text(desktop_settings::hostname())
        .build();
    accessible(&hostname, "Nome do computador");
    form.append(&label("Nome do computador", "row-title"));
    form.append(&hostname);
    let rename = button("Aplicar nome");
    rename.set_halign(gtk::Align::Start);
    form.append(&rename);
    let domain = gtk::Entry::builder()
        .placeholder_text("empresa.exemplo.com")
        .build();
    accessible(&domain, "Domínio");
    form.append(&label("Domínio", "row-title"));
    form.append(&domain);
    let admin = gtk::Entry::builder()
        .placeholder_text("administrador")
        .build();
    accessible(&admin, "Administrador do domínio");
    form.append(&label("Administrador do domínio", "row-title"));
    form.append(&admin);
    let password = gtk::PasswordEntry::builder().show_peek_icon(true).build();
    accessible(&password, "Senha do administrador");
    form.append(&label("Senha do administrador", "row-title"));
    form.append(&password);
    let short = gtk::CheckButton::with_label(&tr("Usar nome de usuário não qualificado"));
    form.append(&short);
    short.set_tooltip_text(Some(&tr("Entrar como usuario em vez de usuario@dominio.")));
    let weak = gtk::CheckButton::with_label(&tr("Aceitar criptografia legada (allow_weak_crypto)"));
    form.append(&weak);
    let rc4 = gtk::CheckButton::with_label(&tr("Compatibilidade RC4 (arcfour)"));
    form.append(&rc4);
    form.append(&label(
        "Criptografia legada reduz a segurança e depende do suporte Kerberos do sistema.",
        "secondary",
    ));
    let confirm = gtk::CheckButton::with_label(&tr(
        "Confirmo o ingresso deste computador no domínio informado.",
    ));
    form.append(&confirm);
    let actions = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let discover = button("Localizar domínio");
    let join = button("Ingressar no domínio");
    join.add_css_class("primary");
    actions.append(&discover);
    actions.append(&join);
    form.append(&actions);
    let status = label("", "secondary");
    form.append(&status);
    if !directory::available() {
        join.set_sensitive(false);
        status.set_text(&tr("Integração de domínio não instalada."));
    }
    let buttons = vec![discover.clone(), join.clone(), rename.clone()];
    let (s, h, bs) = (status.clone(), hostname.clone(), buttons.clone());
    rename.connect_clicked(move |_| {
        let name = h.text().trim().to_lowercase();
        h.set_text(&name);
        task(s.clone(), bs.clone(), move || {
            desktop_settings::set_hostname(&name)
        });
    });
    let (s, d) = (status.clone(), domain.clone());
    let bs = buttons.clone();
    discover.connect_clicked(move |_| {
        let domain = d.text().to_string();
        task(s.clone(), bs.clone(), move || directory::discover(&domain));
    });
    let (s, d, a, p, c, w, q) = (
        status.clone(),
        domain.clone(),
        admin.clone(),
        password.clone(),
        confirm.clone(),
        weak.clone(),
        short.clone(),
    );
    join.connect_clicked(move |_| {
        if !c.is_active() {
            s.set_text(&tr("Confirme o ingresso antes de continuar."));
            return;
        }
        if hostname.text().trim() != desktop_settings::hostname() {
            s.set_text(&tr(
                "Aplique o nome do computador antes de ingressar no domínio.",
            ));
            return;
        }
        let request = JoinRequest {
            domain: d.text().trim().to_lowercase(),
            administrator: a.text().trim().into(),
            password: p.text().into(),
            unqualified_names: q.is_active(),
            allow_weak_crypto: w.is_active(),
            allow_rc4: rc4.is_active(),
        };
        if !directory::valid_domain(&request.domain)
            || request.administrator.is_empty()
            || request.password.is_empty()
        {
            s.set_text(&tr("Preencha um domínio válido, administrador e senha."));
            return;
        }
        p.set_text("");
        task(s.clone(), buttons.clone(), move || directory::join(request));
    });
    card.append(&form);
    body.append(&card);
    let refresh = button("Consultar vínculo atual");
    let joined = label(
        "Consulte o domínio configurado neste computador.",
        "secondary",
    );
    let j = joined.clone();
    refresh.connect_clicked(move |b| task(j.clone(), vec![b.clone()], directory::status));
    body.append(&refresh);
    body.append(&joined);
    super::integrations::append(&body, "accounts");
    scroll
}

fn populate_local(body: &gtk::Box, status: &gtk::Label) {
    while let Some(child) = body.first_child() {
        body.remove(&child);
    }
    status.set_text("");
    match local_accounts::list() {
        Ok(accounts) => {
            let people = card();
            let services = card();
            let mut service_count = 0;
            for account in accounts {
                let title = if account.name.is_empty() {
                    account.username.clone()
                } else {
                    account.name
                };
                let r = row(
                    &title,
                    &format!("{} · UID {}", account.username, account.uid),
                    "accounts",
                );
                let manage = super::integrations::launch_button(catalog::users_target(), status);
                manage.set_label(&tr("Gerenciar ↗"));
                manage.set_tooltip_text(Some(&tr(
                    "Selecione esta conta no gerenciador de usuários para alterar nome, senha e grupos",
                )));
                accessible(
                    &manage,
                    &format!("{} {}", tr("Gerenciar"), account.username),
                );
                r.append(&manage);
                if account.system {
                    services.append(&r);
                    service_count += 1;
                } else {
                    people.append(&r);
                }
            }
            if people.first_child().is_none() {
                people.append(&row("Nenhuma conta de usuário local", "", "accounts"));
            }
            body.append(&people);
            let expander = gtk::Expander::builder()
                .label(format!("{} ({service_count})", tr("Contas de sistema")))
                .child(&services)
                .build();
            body.append(&expander);
        }
        Err(error) => status.set_text(&tr(&error)),
    }
}
