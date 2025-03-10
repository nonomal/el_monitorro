use super::Command;
use super::Message;
use crate::bot::telegram_client::Api;
use crate::db::telegram;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;

static COMMAND: &str = "/get_global_filter";

pub struct GetGlobalFilter {}

impl GetGlobalFilter {
    pub fn execute(db_pool: Pool<ConnectionManager<PgConnection>>, api: Api, message: Message) {
        Self {}.execute(db_pool, api, message);
    }

    fn get_global_template(&self, db_connection: &PgConnection, message: &Message) -> String {
        match telegram::find_chat(db_connection, message.chat.id) {
            None => "You don't have the global filter set".to_string(),
            Some(chat) => match chat.filter_words {
                None => "You don't have the global filter set".to_string(),
                Some(filter_words) => {
                    format!("Your global filter is \n {}", filter_words.join(", "))
                }
            },
        }
    }

    pub fn command() -> &'static str {
        COMMAND
    }
}

impl Command for GetGlobalFilter {
    fn response(
        &self,
        db_pool: Pool<ConnectionManager<PgConnection>>,
        message: &Message,
    ) -> String {
        match self.fetch_db_connection(db_pool) {
            Ok(connection) => self.get_global_template(&connection, message),
            Err(error_message) => error_message,
        }
    }

    fn command(&self) -> &str {
        Self::command()
    }
}
