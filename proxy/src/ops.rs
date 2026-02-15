mod zone_editor;

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new().init()?;

    let domain = std::env::var("DOMAIN").map_err(std::io::Error::other)?;

    let zone_editor = crate::zone_editor::ZoneEditor::new(
        &domain,
        &std::fs::read_to_string("/secrets/cloudflare_dns_api")?,
    )?;

    let directory = acme_micro::Directory::from_url(acme_micro::DirectoryUrl::LetsEncrypt)?;
    let account = directory.register_account(vec![])?;

    let mut order = account.new_order(&domain, &[])?;

    let authorizations = order.authorizations()?;
    let authorization = authorizations.get(0).ok_or(anyhow::anyhow!(
        "expected exactly one authorization in the order!"
    ))?;

    let dns_challenge = authorization
        .dns_challenge()
        .ok_or(anyhow::anyhow!("expected a dns challenge"))?;

    let dns_proof = dns_challenge.dns_proof()?;

    zone_editor.publish_acme_proof(&dns_proof)?;

    log::debug!("looking for _acme-challenge.{domain} = {dns_proof}");
    // TODO: this sleep is trash; we're waiting for cloudflare to publish the TXT record because
    // dns_challenge.validate() can only be called once (???)
    std::thread::sleep(std::time::Duration::from_secs(10));
    let _ = dns_challenge.validate(std::time::Duration::from_millis(5000));
    order.refresh()?;

    let csr_order = order
        .confirm_validations()
        .ok_or(anyhow::anyhow!("expected validations to be confirmed"))?;

    let pkey_pri = acme_micro::create_p384_key()?;

    let certificate_order =
        csr_order.finalize_pkey(pkey_pri, std::time::Duration::from_millis(5000))?;
    let certificate = certificate_order.download_cert()?;

    std::fs::write(
        format!("/secrets/{domain}.certificate.pem"),
        certificate.certificate(),
    )?;
    std::fs::write(
        format!("/secrets/{domain}.certificate.key"),
        certificate.private_key(),
    )?;

    Ok(())
}
