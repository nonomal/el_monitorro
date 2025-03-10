use super::Command;
use super::Message;
use crate::bot::telegram_client::Api;
use crate::db::telegram;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;

static COMMAND: &str = "/set_global_filter";

pub struct SetGlobalFilter {}

impl SetGlobalFilter {
    pub fn execute(db_pool: Pool<ConnectionManager<PgConnection>>, api: Api, message: Message) {
        Self {}.execute(db_pool, api, message);
    }

    fn set_global_template(
        &self,
        db_connection: &PgConnection,
        message: &Message,
        filter: String,
    ) -> String {
        let chat = match telegram::find_chat(db_connection, message.chat.id) {
            Some(chat) => chat,
            None => return "You don't have any subcriptions".to_string(),
        };

        if filter.is_empty() {
            return "Filter can not be empty".to_string();
        }

        let filter_words = match self.parse_filter(&filter) {
            Err(message) => return message,
            Ok(words) => words,
        };

        match telegram::set_global_filter(db_connection, &chat, Some(filter_words.clone())) {
            Ok(_) => format!(
                "The global filter was updated:\n\n{}",
                filter_words.join(", ")
            ),
            Err(_) => "Failed to update the filter".to_string(),
        }
    }

    pub fn command() -> &'static str {
        COMMAND
    }
}

impl Command for SetGlobalFilter {
    fn response(
        &self,
        db_pool: Pool<ConnectionManager<PgConnection>>,
        message: &Message,
    ) -> String {
        match self.fetch_db_connection(db_pool) {
            Ok(connection) => {
                let text = message.text.as_ref().unwrap();
                let argument = self.parse_argument(text);
                self.set_global_template(&connection, message, argument)
            }
            Err(error_message) => error_message,
        }
    }

    fn command(&self) -> &str {
        Self::command()
    }
}
