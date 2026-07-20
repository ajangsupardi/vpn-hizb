use rusqlite::Result;

pub fn create_tables(conn: &rusqlite::Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS vpn_servers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            filename TEXT NOT NULL UNIQUE,
            country TEXT NOT NULL,
            country_code TEXT NOT NULL,
            operator TEXT,
            hostname TEXT NOT NULL,
            ip TEXT NOT NULL,
            port INTEGER NOT NULL DEFAULT 443,
            speed INTEGER NOT NULL DEFAULT 0,
            ping INTEGER NOT NULL DEFAULT 0,
            proto TEXT NOT NULL DEFAULT 'tcp',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_vpn_servers_country_code ON vpn_servers(country_code);
        CREATE INDEX IF NOT EXISTS idx_vpn_servers_proto ON vpn_servers(proto);
        ",
    )?;
    Ok(())
}
