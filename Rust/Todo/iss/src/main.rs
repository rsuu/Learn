use bincode::{config::Configuration, Decode, Encode};
use time::OffsetDateTime;
const BIGCONFIG: Configuration = bincode::config::Configuration::standard();

fn main() {
    let iss = Issue {
        comment: Comment {
            author: User {
                uuid: *uuid::Uuid::new_v4().as_bytes(),
                name: "m".to_string(),
                email: "m".to_string(),
                is_assignee: false,
                is_contributor: false,
            },
            title: "m".to_string(),
            content: "m".to_string(),

            created_at: OffsetDateTime::now_utc().unix_timestamp_nanos(),
            update_at: OffsetDateTime::now_utc().unix_timestamp_nanos(),
            close_at: OffsetDateTime::now_utc().unix_timestamp_nanos(),
        },
        assignee: None,
        discussant: vec![User {
            uuid: *uuid::Uuid::new_v4().as_bytes(),
            name: "m".to_string(),
            email: "m".to_string(),
            is_assignee: false,
            is_contributor: false,
        }],
        metadata: Metadata {
            isopen: true,

            opendate: OffsetDateTime::now_utc().unix_timestamp_nanos(),
            closedate: None,

            is_locked: false,
            comments: 0,

            uuid: *uuid::Uuid::new_v4().as_bytes(),
        },
        label: Some(Label {
            labelmeta: vec![LabelMeta {
                name: "m".to_string(),
                tips: "m".to_string(),
            }],
        }), // Unlabeled, if None
        reply: Some(vec![Reply {
            user: User {
                uuid: *uuid::Uuid::new_v4().as_bytes(),
                name: "m".to_string(),
                email: "m".to_string(),
                is_assignee: false,
                is_contributor: false,
            },
            reply_to: User {
                uuid: *uuid::Uuid::new_v4().as_bytes(),
                name: "m".to_string(),
                email: "m".to_string(),
                is_assignee: false,
                is_contributor: false,
            },
            content: "m".to_string(),
            created_at: OffsetDateTime::now_utc().unix_timestamp_nanos(),
            update_at: OffsetDateTime::now_utc().unix_timestamp_nanos(),
        }]),
    };

    println!("{:#?}", iss);
    let a = OffsetDateTime::from_unix_timestamp(1642212463828388411);

    let en = bincode::encode_to_vec(&iss, BIGCONFIG).unwrap();
}

#[derive(PartialEq, Debug, Encode, Decode, Eq)]
pub struct Issue {
    pub comment: Comment, // questioner
    pub assignee: Option<Vec<User>>,
    pub discussant: Vec<User>,
    pub metadata: Metadata,
    pub label: Option<Label>, // Unlabeled, if None
    pub reply: Option<Vec<Reply>>,
}

#[derive(PartialEq, Debug, Encode, Decode, Eq)]
pub struct Metadata {
    isopen: bool,

    opendate: i128,
    closedate: Option<i128>,

    is_locked: bool,
    comments: u32,

    uuid: [u8; 16], // like link
}

#[derive(PartialEq, Debug, Encode, Decode, Eq)]
pub struct Comment {
    author: User,
    title: String,
    content: String,

    created_at: i128,
    update_at: i128,
    close_at: i128,
}

#[derive(PartialEq, Debug, Encode, Decode, Eq)]
pub struct User {
    uuid: [u8; 16],
    name: String,
    email: String,
    is_assignee: bool,
    is_contributor: bool,
}

#[derive(PartialEq, Debug, Encode, Decode, Eq)]
pub struct Reply {
    user: User,
    reply_to: User,
    content: String,
    created_at: i128,
    update_at: i128,
}

#[derive(PartialEq, Debug, Encode, Decode, Eq)]
pub struct Label {
    pub labelmeta: Vec<LabelMeta>,
}

#[derive(PartialEq, Debug, Encode, Decode, Eq)]
pub struct LabelMeta {
    pub name: String,
    pub tips: String,
}

impl Issue {
    //pub fn new() -> Self {}
}

impl Default for Label {
    fn default() -> Self {
        Label {
            labelmeta: vec![
                LabelMeta {
                    name: "bug".to_string(),
                    tips: "Something isn't working".to_string(),
                },
                LabelMeta {
                    name: "documentation".to_string(),
                    tips: "Improvements or additions to documentation".to_string(),
                },
                LabelMeta {
                    name: "duplicate".to_string(),
                    tips: "This issue or pull request already exists".to_string(),
                },
                LabelMeta {
                    name: "enhancement".to_string(),
                    tips: "New feature or request".to_string(),
                },
                LabelMeta {
                    name: "good first issue".to_string(),
                    tips: "Good for newcomers".to_string(),
                },
                LabelMeta {
                    name: "help wanted".to_string(),
                    tips: "Extra attention is needed".to_string(),
                },
                LabelMeta {
                    name: "invalid".to_string(),
                    tips: "This doesn't seem right".to_string(),
                },
                LabelMeta {
                    name: "question".to_string(),
                    tips: "Further information is requested".to_string(),
                },
                LabelMeta {
                    name: "wontfix".to_string(),
                    tips: "This will not be worked on".to_string(),
                },
            ],
        }
    }
}

struct S {
    a: [u8; 4],
}
impl bincode::Encode for S {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        bincode::enc::Encode::encode(&self.a, encoder)
    }
}

/*
http://smallcultfollowing.com/babysteps/blog/2015/01/14/little-orphan-impls/
+----------------------------------------------------------+---+---+---+---+---+
| Impl Header                                              | O | C | S | F | E |
+----------------------------------------------------------+---+---+---|---|---+
| impl Add<i32> for MyBigInt                               | X | X | X | X | X |
| impl Add<MyBigInt> for i32                               | X | X |   | X | X |
| impl<T> Add<T> for MyBigInt                              | X |   | X | X |   |
| impl<U> Add<MyBigInt> for U                              |   |   |   |   |   |
| impl<T> Modifier<MyType> for Vec<u8>                     | X | X |   | X | X |
| impl<T> Modifier<MyType> for Vec<T>                      |   |   |   |   |   |
| impl<'a, T> FromIterator<T> for Cow<'a, MyVec<T>, [T]>   | X | X | X | X | X |
| impl<'a, T> FromIterator<T> for Cow<'a, [T], MyVec<T>>   |   | X | X | X | X |
| impl<T> BorrowFrom<Rc<T>> for T                          |   | X |   |   | X |
| impl<T> Borrow<T> for Rc<T>                              | X | X | X | X | X |
| impl<H> Hash<H> for MyStruct                             | X |   | X | X | X |
| impl<I:Int,T> Index<I> for MyVec<T>                      | X |   | X | X | X |
+----------------------------------------------------------+---+---+---+---+---+
O - Ordered / C - Covered / S - Covered Self / F - Covered First
E - Explicit Declarations
*/
