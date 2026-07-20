use rusqlite::Result;

pub struct Migration;

impl Migration {
    pub fn run(_conn: &rusqlite::Connection) -> Result<()> {
        Ok(())
    }
}
