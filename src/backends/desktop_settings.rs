use gtk::{gio, glib, prelude::*};

pub fn setting(schema: &str, key: &str, kind: &str) -> Option<gio::Settings> {
    let schema = gio::SettingsSchemaSource::default()?.lookup(schema, true)?;
    if schema.path().is_none()
        || !schema.has_key(key)
        || schema.key(key).value_type().as_str() != kind
    {
        return None;
    }
    Some(gio::Settings::new_full(
        &schema,
        None::<&gio::SettingsBackend>,
        None,
    ))
}

pub fn hostname() -> String {
    std::fs::read_to_string("/proc/sys/kernel/hostname")
        .or_else(|_| std::fs::read_to_string("/etc/hostname"))
        .unwrap_or_default()
        .trim()
        .into()
}

pub fn valid_hostname(value: &str) -> bool {
    // A single DNS label, at most 15 bytes, is interoperable with AD/NetBIOS.
    !value.is_empty()
        && value.len() <= 15
        && !value.starts_with('-')
        && !value.ends_with('-')
        && value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'-')
        && value.bytes().any(|c| c.is_ascii_alphabetic())
}

pub fn set_hostname(value: &str) -> Result<String, String> {
    if !valid_hostname(value) {
        return Err(
            "Use até 15 letras, números ou hífens, começando e terminando com letra ou número."
                .into(),
        );
    }
    let connection = gio::bus_get_sync(gio::BusType::System, None::<&gio::Cancellable>)
        .map_err(|_| "O serviço de nomes do computador está indisponível.")?;
    apply_hostname(&connection, value)
}

fn apply_hostname(connection: &gio::DBusConnection, value: &str) -> Result<String, String> {
    connection
        .call_sync(
            Some("org.freedesktop.hostname1"),
            "/org/freedesktop/hostname1",
            "org.freedesktop.hostname1",
            "SetStaticHostname",
            Some(&(value, true).to_variant()),
            None,
            gio::DBusCallFlags::ALLOW_INTERACTIVE_AUTHORIZATION,
            120_000,
            None::<&gio::Cancellable>,
        )
        .map_err(|_| "Não foi possível alterar o nome. Verifique a autorização administrativa.")?;
    connection.call_sync(Some("org.freedesktop.hostname1"), "/org/freedesktop/hostname1",
        "org.freedesktop.hostname1", "SetHostname", Some(&(value, true).to_variant()),
        None, gio::DBusCallFlags::ALLOW_INTERACTIVE_AUTHORIZATION, 120_000, None::<&gio::Cancellable>)
        .map_err(|_| "Nome permanente alterado, mas o nome ativo não foi confirmado. Aplique novamente antes do ingresso.")?;
    for property in ["StaticHostname", "Hostname"] {
        let reply = connection
            .call_sync(
                Some("org.freedesktop.hostname1"),
                "/org/freedesktop/hostname1",
                "org.freedesktop.DBus.Properties",
                "Get",
                Some(&("org.freedesktop.hostname1", property).to_variant()),
                None,
                gio::DBusCallFlags::NONE,
                5000,
                None::<&gio::Cancellable>,
            )
            .map_err(|_| "O pedido foi enviado, mas não foi possível confirmar o nome atual.")?;
        let (actual,) = reply
            .get::<(glib::Variant,)>()
            .ok_or("Resposta inválida do serviço de nomes.")?;
        if actual.str() != Some(value) {
            return Err("O serviço não confirmou o nome solicitado.".into());
        }
    }
    Ok("Nome do computador atualizado.".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hostname_is_ad_compatible() {
        for name in ["office-01", "PC", "a23456789012345"] {
            assert!(valid_hostname(name));
        }
        for name in [
            "",
            "123",
            "-pc",
            "pc-",
            "pc.local",
            "pc;id",
            "a234567890123456",
            "é",
        ] {
            assert!(!valid_hostname(name));
        }
    }

    #[test]
    #[ignore = "requires an isolated dbus-run-session; never uses the system bus"]
    fn hostname_service_contract() {
        let (tx, rx) = std::sync::mpsc::channel();
        let worker = std::thread::spawn(move || {
            let context = glib::MainContext::new();
            context.with_thread_default(|| {
                let connection = gio::bus_get_sync(gio::BusType::Session, None::<&gio::Cancellable>).unwrap();
                connection.call_sync(Some("org.freedesktop.DBus"), "/org/freedesktop/DBus", "org.freedesktop.DBus", "RequestName",
                    Some(&("org.freedesktop.hostname1", 4u32).to_variant()), None, gio::DBusCallFlags::NONE, 5000, None::<&gio::Cancellable>).unwrap();
                let info = gio::DBusNodeInfo::for_xml(r#"<node><interface name="org.freedesktop.hostname1">
                    <method name="SetStaticHostname"><arg type="s" direction="in"/><arg type="b" direction="in"/></method>
                    <method name="SetHostname"><arg type="s" direction="in"/><arg type="b" direction="in"/></method>
                    <property name="StaticHostname" type="s" access="read"/><property name="Hostname" type="s" access="read"/>
                    </interface></node>"#).unwrap();
                let values = std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::<String, String>::new()));
                let state = values.clone();
                let registration = connection.register_object("/org/freedesktop/hostname1", &info.interfaces()[0])
                    .method_call(move |_, _, _, _, method, args, reply| {
                        let (name, interactive) = args.get::<(String, bool)>().unwrap();
                        assert!(interactive);
                        if name == "denied" || (name == "partial" && method == "SetHostname") {
                            reply.return_dbus_error("org.freedesktop.DBus.Error.AccessDenied", "Denied by test service");
                        } else {
                            let key = if method == "SetHostname" { "Hostname" } else { "StaticHostname" };
                            state.borrow_mut().insert(key.into(), name);
                            reply.return_value(Some(&().to_variant()));
                        }
                    })
                    .property(move |_, _, _, _, key| values.borrow().get(key).cloned().unwrap_or_default().to_variant())
                    .build().unwrap();
                let loop_ = glib::MainLoop::new(Some(&context), false);
                tx.send((connection.clone(), loop_.clone())).unwrap();
                loop_.run();
                connection.unregister_object(registration).unwrap();
            }).unwrap();
        });
        let (connection, loop_) = rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        assert!(apply_hostname(&connection, "office-01").is_ok());
        assert!(apply_hostname(&connection, "denied")
            .unwrap_err()
            .contains("autorização"));
        assert!(apply_hostname(&connection, "partial")
            .unwrap_err()
            .contains("Nome permanente alterado"));
        loop_.quit();
        worker.join().unwrap();
    }
}
