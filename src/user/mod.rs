mod types;

use http::Method;
pub use types::*;

use crate::{Service, NONE_BODY};

impl Service {
    pub fn user(&self) -> UserService {
        UserService { service: self.clone() }
    }
}

pub struct UserService {
    service: Service,
}

impl UserService {
    pub async fn get_users(&self) -> anyhow::Result<Vec<User>> {
        self.service
            .request_data(Method::GET, format!("/api/get-users?owner={}", self.service.org_name()), NONE_BODY)
            .await?
            .into_data_default()
    }

    pub async fn get_sorted_users(&self, sorter: String, limit: i32) -> anyhow::Result<Vec<User>> {
        self.service
            .request_data(
                Method::GET,
                format!("/api/get-sorted-users?owner={}&sorter={sorter}&limit={limit}", self.service.org_name()),
                NONE_BODY,
            )
            .await?
            .into_data_default()
    }

    pub async fn get_user_count(&self, is_online: QueryUserSet) -> anyhow::Result<i64> {
        self.service
            .request_data(
                Method::GET,
                format!("/api/get-user-count?owner={}&isOnline={}", self.service.org_name(), is_online),
                NONE_BODY,
            )
            .await?
            .into_data_default()
    }

    pub async fn get_user(&self, name: String) -> anyhow::Result<Option<User>> {
        self.service
            .request_data(Method::GET, format!("/api/get-user?id={}/{}", self.service.org_name(), name), NONE_BODY)
            .await?
            .into_data()
    }

    pub async fn get_user_by_email(&self, email: String) -> anyhow::Result<Option<User>> {
        self.service
            .request_data(
                Method::GET,
                format!("/api/get-user?owner={}&email={}", self.service.org_name(), email),
                NONE_BODY,
            )
            .await?
            .into_data()
    }

    pub async fn get_user_by_phone(&self, phone: String) -> anyhow::Result<Option<User>> {
        self.service
            .request_data(
                Method::GET,
                format!("/api/get-user?owner={}&phone={}", self.service.org_name(), phone),
                NONE_BODY,
            )
            .await?
            .into_data()
    }

    pub async fn get_user_by_user_id(&self, user_id: String) -> anyhow::Result<Option<User>> {
        self.service
            .request_data(
                Method::GET,
                format!("/api/get-user?owner={}&userId={}", self.service.org_name(), user_id),
                NONE_BODY,
            )
            .await?
            .into_data()
    }

    pub async fn modify_user(&self, args: ModifyUserArgs) -> anyhow::Result<UserOpAction> {
        let mut url_path = format!("/api/{}?id={}/{}", args.action, args.user.owner, args.user.name,);
        if !args.columns.is_empty() {
            url_path += &format!("&columns={}", args.columns.join(","));
        }
        self.service
            .request_data(Method::POST, url_path, Some(&args.user))
            .await?
            .into_data_default()
    }

    pub async fn add_user(&self, user: User, columns: Vec<String>) -> anyhow::Result<UserOpAction> {
        self.modify_user(ModifyUserArgs {
            action: UserAction::Add,
            user,
            columns,
        })
        .await
    }

    pub async fn delete_user(&self, user: User, columns: Vec<String>) -> anyhow::Result<UserOpAction> {
        self.modify_user(ModifyUserArgs {
            action: UserAction::Delete,
            user,
            columns,
        })
        .await
    }

    pub async fn update_user(&self, user: User, columns: Vec<String>) -> anyhow::Result<UserOpAction> {
        self.modify_user(ModifyUserArgs {
            action: UserAction::Update,
            user,
            columns,
        })
        .await
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_user_json() {
        let json_data = r#"
    {
        "owner": "example_owner",
        "name": "example_name",
        "createdTime": "2022-01-01T00:00:00Z",
        "updatedTime": "2022-01-01T00:00:00Z",
        
        "id": "example_id",
        "type": "example_type",
        "password": "example_password",
        "passwordSalt": "example_salt",
        "passwordType": "example_type",
        "displayName": "Example User",
        "firstName": "First",
        "lastName": "Last",
        "avatar": "example_avatar",
        "avatarType": "example_type",
        "permanentAvatar": "example_perm_avatar",
        "email": "example@example.com",
        "emailVerified": true,
        "phone": "123456789",
        "countryCode": "example_cc",
        "region": "example_region",
        "location": "Example Location",
        "address": ["Example Address"],
        "affiliation": "example_affiliation",
        "title": "example_title",
        "idCardType": "example_card_type",
        "idCard": "example_card",
        "homepage": "https://example.com",
        "bio": "Example bio",
        "tag": "example_tag",
        "language": "en",
        "gender": "M",
        "birthday": "1990-01-01",
        "education": "example_education",
        "score": 100,
        "karma": 10,
        "ranking": 1,
        "isDefaultAvatar": true,
        "isOnline": true,
        "isAdmin": true,
        "isForbidden": false,
        "isDeleted": false,
        "signupApplication": "example_signup_app",
        "hash": "example_hash",
        "preHash": "example_pre_hash",
        
        "github": "example_github",
        "google": "example_google",
        "qq": "example_qq",
        "wechat": "example_wechat",
        "facebook": "example_facebook",
        "dingtalk": "example_dingtalk",
        "weibo": "example_weibo",
        "gitee": "example_gitee",
        "linkedin": "example_linkedin",
        "wecom": "example_wecom",
        "lark": "example_lark",
        "gitlab": "example_gitlab",
        "ldap": "example_ldap",

        "properties": {
            "additionalProp1": "value1",
            "additionalProp2": "value2",
            "additionalProp3": "value3"
        },
        "groups": ["ExampleGroup"],
        "lastSigninWrongTime": "2022-01-01T00:00:00Z",
        "signinWrongTimes": 0
    }
    "#;

        let casdoor_user: User = serde_json::from_str(json_data).expect("JSON parsing failed");
        println!("{:?}", casdoor_user);
    }
}
