use serde::{de::Visitor, Deserialize};

#[derive(Deserialize, Debug)]
pub enum Role {
	Owner,
	Developer,
	Tester,
	Moderator,

	#[serde(rename = "Bug Hunter")]
	BugHunter,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RoleOrRoles {
	Role(Role),
	Roles(Vec<Role>),
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
