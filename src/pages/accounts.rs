use crate::{
    backends::directory::{self, JoinRequest},
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
                for b in &buttons {
                    b.set_sensitive(true);
                }
                gtk::glib::ControlFlow::Break
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                status.set_text(&tr("A operação foi interrompida."));
                for b in &buttons {
                    b.set_sensitive(true);
                }
                gtk::glib::ControlFlow::Break
            }
            _ => gtk::glib::ControlFlow::Continue,
        }
    });
}
pub fn build() -> gtk::ScrolledWindow {
    let (scroll, body) = page("Contas", "Perfil de usuário e acesso a organizações.");
    let local = card();
    local.append(&row(
        &gtk::glib::user_name().to_string_lossy(),
        "Usuário da sessão atual",
        "accounts",
    ));
    body.append(&local);
    body.append(&label("Acessar trabalho ou escola", "section-title"));
    let card = card();
    card.append(&row(
        "Active Directory",
        "Ingresso do dispositivo no domínio da organização.",
        "security",
    ));
    let form = gtk::Box::new(gtk::Orientation::Vertical, 12);
    form.add_css_class("form");
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
    form.append(&label("Permite entrar como usuario em vez de usuario@dominio. Nomes locais iguais podem causar ambiguidade.","secondary"));
    let weak = gtk::CheckButton::with_label(&tr("Aceitar criptografia legada (allow_weak_crypto)"));
    form.append(&weak);
    form.append(&label("Reduz a proteção do Kerberos para compatibilidade com domínios antigos. Não reativa algoritmos removidos da biblioteca Kerberos.","secondary"));
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
    let status = label(
        "A autorização de administrador local será solicitada pelo sistema.",
        "notice",
    );
    form.append(&status);
    if !directory::available() {
        join.set_sensitive(false);
        status.set_text(&tr("Instale o auxiliar Monolito e as dependências de domínio para habilitar o ingresso. Consulte o README."));
    }
    let buttons = vec![discover.clone(), join.clone()];
    let (s, d) = (status.clone(), domain.clone());
    let bs = buttons.clone();
    discover.connect_clicked(move |_| {
        let domain = d.text().to_string();
        let join_enabled = bs[1].is_sensitive();
        let active = if join_enabled {
            bs.clone()
        } else {
            vec![bs[0].clone()]
        };
        task(s.clone(), active, move || directory::discover(&domain));
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
        let request = JoinRequest {
            domain: d.text().trim().to_lowercase(),
            administrator: a.text().trim().into(),
            password: p.text().into(),
            unqualified_names: q.is_active(),
            allow_weak_crypto: w.is_active(),
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
    scroll
}
