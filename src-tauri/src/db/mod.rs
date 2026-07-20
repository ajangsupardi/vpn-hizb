pub mod migrations;
pub mod schema;

use crate::models::VpnServer;
use rusqlite::{params, Connection, Result};
use std::path::Path;
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.run_migrations()?;
        Ok(db)
    }

    fn run_migrations(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        schema::create_tables(&conn)?;
        Ok(())
    }

    pub fn get_servers(&self) -> Result<Vec<VpnServer>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, filename, country, country_code, operator, hostname, ip, port, speed, ping, proto, created_at, updated_at FROM vpn_servers ORDER BY ping ASC",
        )?;
        let servers = stmt
            .query_map([], |row| {
                Ok(VpnServer {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    filename: row.get(2)?,
                    country: row.get(3)?,
                    country_code: row.get(4)?,
                    operator: row.get(5)?,
                    hostname: row.get(6)?,
                    ip: row.get(7)?,
                    port: row.get(8)?,
                    speed: row.get(9)?,
                    ping: row.get(10)?,
                    proto: row.get(11)?,
                    created_at: row.get(12)?,
                    updated_at: row.get(13)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;
        Ok(servers)
    }

    pub fn get_server_by_filename(&self, filename: &str) -> Result<Option<VpnServer>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, filename, country, country_code, operator, hostname, ip, port, speed, ping, proto, created_at, updated_at FROM vpn_servers WHERE filename = ?1",
        )?;
        let mut servers = stmt.query_map(params![filename], |row| {
            Ok(VpnServer {
                id: row.get(0)?,
                name: row.get(1)?,
                filename: row.get(2)?,
                country: row.get(3)?,
                country_code: row.get(4)?,
                operator: row.get(5)?,
                hostname: row.get(6)?,
                ip: row.get(7)?,
                port: row.get(8)?,
                speed: row.get(9)?,
                ping: row.get(10)?,
                proto: row.get(11)?,
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
            })
        })?;
        match servers.next() {
            Some(server) => Ok(Some(server?)),
            None => Ok(None),
        }
    }

    pub fn save_servers(&self, servers: &[VpnServer]) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch("DELETE FROM vpn_servers;")?;
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        for server in servers {
            conn.execute(
                "INSERT INTO vpn_servers (name, filename, country, country_code, operator, hostname, ip, port, speed, ping, proto, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                params![
                    server.name,
                    server.filename,
                    server.country,
                    server.country_code,
                    server.operator,
                    server.hostname,
                    server.ip,
                    server.port,
                    server.speed,
                    server.ping,
                    server.proto,
                    now,
                    now,
                ],
            )?;
        }
        Ok(())
    }

    pub fn server_exists(&self, filename: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let mut stmt =
            conn.prepare("SELECT COUNT(*) FROM vpn_servers WHERE filename = ?1")?;
        let count: i64 = stmt.query_row(params![filename], |row| row.get(0))?;
        Ok(count > 0)
    }
}
