use async_trait::async_trait;
use poot::*;
use std::{sync::Arc, env, any::Any};
use dotenv::dotenv;
use rhai::{Engine, Dynamic, Scope, NativeCallContext};

#[test]
fn a(){
	let mut sum: f64 = 0.0;
	for i in 0..10000 {
	   let add = 1f64/((i*2+1) as f64);
	   if i %2 == 0 {
		  sum+=add;
	   } else {
		  sum-=add;
	   };
	};
	println!("{}", 4.0*sum);
}

struct BotEvents;
#[async_trait]
impl Events for BotEvents {
	async fn on_ready(&self, context: Context) {
		println!("DONE");
	}
	async fn on_post(&self, ctx: Context, post: Post) {
		const PREFIX: &str = "@62e308b3a52561e4fe8671a9(FerRoBot) ";
		if post.content.starts_with(PREFIX) {
			let c = &post.content[PREFIX.len()..];

			let res = {
				let mut engine = Engine::new();
				engine.register_get("author",|post: &mut Post|{
					post.author.clone()
				});
				engine.register_get("content",|post: &mut Post|{
					post.content.clone()
				});
				engine.register_get("id",|post: &mut Post|{
					post.id
				});
				
				macro_rules! to_string_for_id {
					($name: ident) => {
						engine.register_fn("to_string",|id: $name|{
							id.to_string()
						});
					};
				}
				to_string_for_id!(UserId);
				to_string_for_id!(ChatId);
				to_string_for_id!(PostId);
				engine.register_get("username", |user: &mut User| {
					user.username.clone()
				});
				// all my homies fucking hate rust clone clone clone
				{
					let _ctx = ctx.clone();
					engine.register_fn("create_chat", move |post: &mut Post, content: &str|{
						let ctx = _ctx.clone();
						let content = content.to_owned();
						let post = post.id.clone();
						tokio::spawn(async move {
							post.create_chat(&ctx, content.as_str()).await.unwrap()
						});
					});
				}
				let mut scope = Scope::new();
				scope.push("burgerbot", "Belongs in the dumpster");
				scope.push("photopjs", "Low effort trash library.");
				scope.push("post", post.clone());
				engine.on_progress(|count|{
					if count == 1_000_000 {
						Some(Dynamic::from("Runtime exceeds 1,000,000 operations; Terminated."))
					} else {
						None
					}
				});
				let result = engine.eval_with_scope::<Dynamic>(&mut scope, c);
				match result {
					Ok(r) => {
						r.to_string()
					}
					Err(e) => {
						let mut res = format!("{:?}", e.unwrap_inner());
						res.truncate(100);
						res
					}
				}
			};
			post.create_chat(&ctx, res.as_str()).await.unwrap();
		};
	}
}

#[tokio::main]
async fn main() {
	dotenv().ok();
    Client::new(
        Auth::Token {
            user_id: env::var("USER_ID").unwrap(),
            token: env::var("TOKEN").unwrap(),
        },
        BotEvents,
    )
    .await;
}
