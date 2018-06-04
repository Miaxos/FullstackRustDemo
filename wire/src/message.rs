use user::UserResponse;
use chrono::NaiveDateTime;
use uuid::Uuid;
use identifiers::message::MessageUuid;
use identifiers::chat::ChatUuid;
use identifiers::user::UserUuid;



#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MessageResponse {
    pub id: MessageUuid,
    pub author: UserResponse,
    pub reply: Option<Box<MessageResponse>>,
    pub content: String,
    pub date: NaiveDateTime,
}



#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewMessageRequest {
    pub author_id: UserUuid,
    pub chat_id: ChatUuid,
    pub reply_id: Option<Uuid>,
    pub content: String,
}
