use crate::question::*;
use identifiers::{
    bucket::BucketUuid,
    question::QuestionUuid,
    user::UserUuid,
};
use wire::{
    answer::AnswerResponse,
    question::*,
};
use crate::User;

impl From<QuestionData> for QuestionResponse {
    fn from(data: QuestionData) -> QuestionResponse {
        QuestionResponse {
            uuid: QuestionUuid(data.question.uuid),
            bucket_uuid: BucketUuid(data.question.bucket_uuid),
            question_text: data.question.question_text,
            author: data.user.clone().map(User::into),
            answers: data.answers.into_iter().map(AnswerResponse::from).collect(),
            on_floor: data.question.on_floor,
        }
    }
}

impl NewQuestion {
    pub fn attach_user_id(request: NewQuestionRequest, user_id: Option<UserUuid>) -> NewQuestion {
        NewQuestion {
            bucket_uuid: request.bucket_uuid.0,
            author_uuid: user_id.map(|u|u.0),
            question_text: request.question_text,
            on_floor: false, // by default, the question is in the bucket and not in the floor.
        }
    }
}
//impl From<NewQuestionRequest> for NewQuestion {
//    fn from(request: NewQuestionRequest) -> NewQuestion {
//        NewQuestion {
//            bucket_id: request.bucket_id,
//            author_id: request.author_id,
//            question_text: request.question_text,
//        }
//    }
//}
