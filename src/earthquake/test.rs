// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[cfg(test)]
mod test {
    use crate::{common::Lang, fetch::fetch};

    #[tokio::test]
    async fn message_test() {
        use crate::earthquake::message::Message;

        let message: Message = fetch(Lang::tc).await.unwrap();
        println!("{:?}", message);
    }
}
