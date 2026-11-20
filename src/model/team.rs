use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::model::{
    id::{TeamId, UserId},
    macros::{int_enum, str_enum},
    user::User,
    util::ImageHash,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Team {
    pub icon: Option<ImageHash>,

    pub id: TeamId,

    pub members: Vec<Member>,

    pub name: CompactString,

    pub owner_user_id: UserId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "TeamMember")]
pub struct Member {
    pub membership_state: MembershipState,

    pub team_id: TeamId,

    pub user: User,

    pub role: MemberRole,
}

int_enum! {




    pub enum MembershipState: u8 {

        Invited = 1,

        Accepted = 2,
    }
}

str_enum! {



    #[doc(alias = "TeamMemberRole")]
    pub enum MemberRole {

        Admin = "admin",

        Developer = "developer",

        ReadOnly = "read_only",
    }
}
