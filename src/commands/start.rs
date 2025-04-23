use crate::settings::Settings;

pub fn start(settings: &Settings) {
    println!(
        "Starting SIGE api on http://{}:{}!",
        settings.address.host, settings.address.port
    );

    web::start(
        &settings.address.host,
        settings.address.port,
        &settings.database.url,
    );
}
