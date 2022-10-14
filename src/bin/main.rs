use anyhow::Result;
use celestus::database::Database;

fn main() -> Result<()> {
    let mut db = Database::new();
    db.prepare()?;

    Ok(())
}
