use log::{debug, error, info, warn};
use rusqlite::Connection;
use serde::Deserialize;
use std::error::Error;

pub trait Restorable {
    fn restore_table(&self, conn: Connection) -> Result<i32, Box<dyn Error>>;
}

#[derive(Debug, Deserialize)]
pub struct DomainList {
    pub domain_type: i32,
    pub list: Vec<Domain>,
}

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub id: i32,
    pub domain: String,
    pub enabled: i32,
    pub date_added: i64,
    pub comment: String,
}

impl Restorable for DomainList {
    fn restore_table(&self, conn: Connection) -> Result<i32, Box<dyn Error>> {
        debug!("restoring domainlist table");

        let sql = format!("INSERT OR IGNORE INTO domainlist (id,domain,enabled,date_added,comment,type) VALUES (:id,:domain,:enabled,:date_added,:comment,{});", self.domain_type);
        let mut stmt = conn.prepare(&sql)?;

        let record_count = self.list.len() as i32;
        debug!("starting to load {} records to domainlist", record_count);

        for record in &self.list {
            let result = stmt.execute_named(&[
                (":id", &record.id),
                (":domain", &record.domain),
                (":enabled", &record.enabled),
                (":date_added", &record.date_added),
                (":comment", &record.comment),
            ]);

            match result {
                Ok(_) => {}
                Err(e) => {
                    warn!("error while inserting an entry to domainlist table: {}", e);
                }
            }
        }

        Ok(record_count)
    }
}

#[derive(Debug, Deserialize)]
pub struct AdList {
    pub list: Vec<Ad>,
}

#[derive(Debug, Deserialize)]
pub struct Ad {
    pub id: i32,
    pub address: String,
    pub enabled: i32,
    pub date_added: i64,
    pub comment: String,
}

impl Restorable for AdList {
    fn restore_table(&self, conn: Connection) -> Result<i32, Box<dyn Error>> {
        debug!("restoring adlist table");

        let sql = "INSERT OR IGNORE INTO adlist (id,address,enabled,date_added,comment) VALUES (:id,:address,:enabled,:date_added,:comment);".to_string();
        let mut stmt = conn.prepare(&sql)?;

        let record_count = self.list.len() as i32;
        debug!("starting to load {} records to adlist", record_count);

        for record in &self.list {
            let result = stmt.execute_named(&[
                (":id", &record.id),
                (":address", &record.address),
                (":enabled", &record.enabled),
                (":date_added", &record.date_added),
                (":comment", &record.comment),
            ]);

            match result {
                Ok(_) => {}
                Err(e) => {
                    warn!("error while inserting an entry to adlist table: {}", e);
                }
            }
        }

        Ok(record_count)
    }
}

#[derive(Debug, Deserialize)]
pub struct DomainAuditList {
    pub list: Vec<DomainAuditEntry>,
}

#[derive(Debug, Deserialize)]
pub struct DomainAuditEntry {
    pub id: i32,
    pub domain: String,
    pub date_added: i64,
}

impl Restorable for DomainAuditList {
    fn restore_table(&self, conn: Connection) -> Result<i32, Box<dyn Error>> {
        debug!("restoring domain_audit table");

        let sql = "INSERT OR IGNORE INTO domain_audit (id,domain,date_added) VALUES (:id,:domain,:date_added);".to_string();
        let mut stmt = conn.prepare(&sql)?;

        let record_count = self.list.len() as i32;
        debug!("starting to load {} records to domain_audit", record_count);

        for record in &self.list {
            let result = stmt.execute_named(&[
                (":id", &record.id),
                (":domain", &record.domain),
                (":date_added", &record.date_added),
            ]);

            match result {
                Ok(_) => {}
                Err(e) => {
                    warn!(
                        "error while inserting an entry to domain_audit table: {}",
                        e
                    );
                }
            }
        }

        Ok(record_count)
    }
}

#[derive(Debug, Deserialize)]
pub struct GroupList {
    pub list: Vec<Group>,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub date_added: i64,
    pub description: String,
}

impl Restorable for GroupList {
    fn restore_table(&self, conn: Connection) -> Result<i32, Box<dyn Error>> {
        debug!("restoring group table");

        let sql =
            "INSERT OR IGNORE INTO \"group\" (id,name,date_added,description) VALUES (:id,:name,:date_added,:description);"
                .to_string();
        let mut stmt = conn.prepare(&sql)?;

        let record_count = self.list.len() as i32;
        debug!("starting to load {} records to group", record_count);

        for record in &self.list {
            let result = stmt.execute_named(&[
                (":id", &record.id),
                (":name", &record.name),
                (":date_added", &record.date_added),
                (":description", &record.description),
            ]);

            match result {
                Ok(_) => {}
                Err(e) => {
                    warn!("error while inserting an entry to group table: {}", e);
                }
            }
        }

        Ok(record_count)
    }
}

#[derive(Debug, Deserialize)]
pub struct ClientList {
    pub list: Vec<Client>,
}

#[derive(Debug, Deserialize)]
pub struct Client {
    pub id: i32,
    pub ip: String,
    pub date_added: i64,
    pub comment: String,
}

impl Restorable for ClientList {
    fn restore_table(&self, conn: Connection) -> Result<i32, Box<dyn Error>> {
        debug!("restoring client table");

        let sql =
            "INSERT OR IGNORE INTO client (id,ip,date_added,comment) VALUES (:id,:ip,:date_added,:comment);"
                .to_string();
        let mut stmt = conn.prepare(&sql)?;

        let record_count = self.list.len() as i32;
        debug!("starting to load {} records to client", record_count);

        for record in &self.list {
            let result = stmt.execute_named(&[
                (":id", &record.id),
                (":ip", &record.ip),
                (":date_added", &record.date_added),
                (":comment", &record.comment),
            ]);

            match result {
                Ok(_) => {}
                Err(e) => {
                    warn!("error while inserting an entry to group table: {}", e);
                }
            }
        }

        Ok(record_count)
    }
}

#[derive(Debug, Deserialize)]
pub struct ClientGroupAssignmentList {
    pub list: Vec<ClientGroupAssignment>,
}

#[derive(Debug, Deserialize)]
pub struct ClientGroupAssignment {
    pub client_id: i32,
    pub group_id: i32,
}

impl Restorable for ClientGroupAssignmentList {
    fn restore_table(&self, conn: Connection) -> Result<i32, Box<dyn Error>> {
        debug!("restoring client_by_group table");

        let sql =
            "INSERT OR IGNORE INTO client_by_group (client_id,group_id) VALUES (:client_id,:group_id);"
                .to_string();
        let mut stmt = conn.prepare(&sql)?;

        let record_count = self.list.len() as i32;
        debug!(
            "starting to load {} records to client_by_group",
            record_count
        );

        for record in &self.list {
            let result = stmt.execute_named(&[
                (":client_id", &record.client_id),
                (":group_id", &record.group_id),
            ]);

            match result {
                Ok(_) => {}
                Err(e) => {
                    warn!(
                        "error while inserting an entry to client_by_group table: {}",
                        e
                    );
                }
            }
        }

        Ok(record_count)
    }
}

#[derive(Debug, Deserialize)]
pub struct DomainListGroupAssignmentList {
    pub list: Vec<DomainListGroupAssignment>,
}

#[derive(Debug, Deserialize)]
pub struct DomainListGroupAssignment {
    pub domainlist_id: i32,
    pub group_id: i32,
}

impl Restorable for DomainListGroupAssignmentList {
    fn restore_table(&self, conn: Connection) -> Result<i32, Box<dyn Error>> {
        debug!("restoring domainlist_by_group table");

        let sql =
            "INSERT OR IGNORE INTO domainlist_by_group (domainlist_id,group_id) VALUES (:domainlist_id,:group_id);"
                .to_string();
        let mut stmt = conn.prepare(&sql)?;

        let record_count = self.list.len() as i32;
        debug!(
            "starting to load {} records to domainlist_by_group",
            record_count
        );

        for record in &self.list {
            let result = stmt.execute_named(&[
                (":domainlist_id", &record.domainlist_id),
                (":group_id", &record.group_id),
            ]);

            match result {
                Ok(_) => {}
                Err(e) => {
                    warn!(
                        "error while inserting an entry to domainlist_by_group table: {}",
                        e
                    );
                }
            }
        }

        Ok(record_count)
    }
}

#[derive(Debug, Deserialize)]
pub struct AdListGroupAssignmentList {
    pub list: Vec<AdListGroupAssignment>,
}

#[derive(Debug, Deserialize)]
pub struct AdListGroupAssignment {
    pub adlist_id: i32,
    pub group_id: i32,
}

impl Restorable for AdListGroupAssignmentList {
    fn restore_table(&self, conn: Connection) -> Result<i32, Box<dyn Error>> {
        debug!("restoring adlist_by_group table");

        let sql =
            "INSERT OR IGNORE INTO adlist_by_group (adlist_id,group_id) VALUES (:adlist_id,:group_id);"
                .to_string();
        let mut stmt = conn.prepare(&sql)?;

        let record_count = self.list.len() as i32;
        debug!(
            "starting to load {} records to adlist_by_group",
            record_count
        );

        for record in &self.list {
            let result = stmt.execute_named(&[
                (":adlist_id", &record.adlist_id),
                (":group_id", &record.group_id),
            ]);

            match result {
                Ok(_) => {}
                Err(e) => {
                    warn!(
                        "error while inserting an entry to adlist_by_group table: {}",
                        e
                    );
                }
            }
        }

        Ok(record_count)
    }
}
