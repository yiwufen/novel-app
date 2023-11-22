use crate::opi::*;

#[tokio::test]
async fn test_async_function() {
    let messages = vec![Message {
        role: "user".to_string(),
        content: "Hello, this is a test?".to_string(),
    }];
    let oaibody = OAIbody {
        model: "gpt-3.5-turbo".to_string(),
        messages: messages,
    };
    let resp = sentmessages(&oaibody).await;
    println!("{:?}", resp);
    assert_eq!(resp.is_ok(), true);
}
