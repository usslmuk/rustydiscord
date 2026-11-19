use std::sync::Arc;

pub(crate) fn client_config() -> rustls::ClientConfig {
    rustls::ClientConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
        .with_safe_default_protocol_versions()
        .expect("the ring provider supports the default protocol versions")
        .with_root_certificates(root_store())
        .with_no_client_auth()
}

pub(crate) fn root_store() -> rustls::RootCertStore {
    let mut roots = rustls::RootCertStore::empty();
    roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    #[cfg(feature = "native-roots")]
    add_native_roots(&mut roots);
    roots
}

#[cfg(feature = "native-roots")]
fn add_native_roots(roots: &mut rustls::RootCertStore) {
    let loaded = rustls_native_certs::load_native_certs();
    for error in &loaded.errors {
        tracing::warn!(%error, "could not load system root certificates");
    }
    let (added, ignored) = roots.add_parsable_certificates(loaded.certs);
    if ignored > 0 {
        tracing::warn!(
            ignored,
            "skipped system root certificates that could not be parsed"
        );
    }
    tracing::debug!(added, "trusting system root certificates");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_roots_are_always_trusted() {
        let roots = root_store();
        assert!(roots.len() >= webpki_roots::TLS_SERVER_ROOTS.len());
    }
}
