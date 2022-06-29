pub struct User {

}
pub struct UserId(String);
pub struct UserRef {
	id: UserId,
	value: Option<User>,
}
impl UserRef {
	fn unwrap(self) -> User {
		self.value.unwrap()
	}
}
