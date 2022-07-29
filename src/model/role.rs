use serde::{Deserialize};

#[derive(Deserialize, Debug,Clone,Copy)]
pub enum Role {
    Owner,
    Developer,
    Tester,
    Moderator,
	Contributor,
	Verified,
    #[serde(rename = "Bug Hunter")]
    BugHunter,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RoleOrRoles {
    Role(Role),
    Roles(Vec<Role>),
}

impl Default for RoleOrRoles {
	fn default() -> Self {
		RoleOrRoles::Roles(vec![])
	}
}

impl Into<Vec<Role>> for RoleOrRoles {
    fn into(self) -> Vec<Role> {
        match self {
            RoleOrRoles::Role(role) => vec![role],
            RoleOrRoles::Roles(roles) => roles,
        }
    }
}

#[test]
fn test() {
    let roles: RoleOrRoles = serde_json::from_str(
        r#"
		"Owner"
	"#,
    )
    .unwrap();
    println!("{:#?}", roles);
}
