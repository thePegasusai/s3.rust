// Access control: A production-ready S3 storage system would need to provide a way to control access to data by implementing user and role-based access control. This could be achieved by using libraries such as serde and chrono'
    
extern crate serde;
    extern crate chrono;

    use self::serde::{Deserialize, Serialize 
};
use self::chrono::{Duration, Utc};

struct AccessControl {
    owner: String,
    grants: Vec<Grant>,
}

struct Grant {
    grantee: String,
    permissions: Vec<Permission>,
    expiration: Option<Utc>,
}

enum Permission {
    Read,
    Write,
    ReadAcp,
    WriteAcp,
    FullControl,
}

impl AccessControl {
    fn check_permission(&self, grantee: &str, permission: Permission) -> bool {
        self.grants
            .iter()
            .filter(|grant| grant.grantee == grantee)
            .any(|grant| {
                if let Some(expiration) = grant.expiration {
                    Utc::now() < expiration
                } else {
                    true
                }
            })
            .filter(|grant| grant.permissions.contains(&permission))
            .count()
            > 0
    }
}
