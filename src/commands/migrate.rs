use crate::settings::Settings;

pub fn migrate(settings: &Settings) -> anyhow::Result<()> {
    migration::migrate(&settings.database.url)?;

    Ok(())
}
