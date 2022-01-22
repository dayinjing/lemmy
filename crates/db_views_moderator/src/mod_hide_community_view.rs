use diesel::{result::Error, *};
use lemmy_db_schema::{
  limit_and_offset,
  newtypes::CommunityId,
  schema::{community, mod_hide_community, person},
  source::{
    community::{Community, CommunitySafe},
    moderator::ModHideCommunity,
    person::{Person, PersonSafe},
  },
  traits::{ToSafe, ViewToVec},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModHideCommunityView {
  pub mod_hide_community: ModHideCommunity,
  pub admin: PersonSafe,
  pub community: CommunitySafe,
}

type ModHideCommunityViewTuple = (ModHideCommunity, PersonSafe, CommunitySafe);

impl ModHideCommunityView {
  pub fn list(
    conn: &PgConnection,
    community_id: Option<CommunityId>,
    page: Option<i64>,
    limit: Option<i64>,
  ) -> Result<Vec<Self>, Error> {
    let mut query = mod_hide_community::table
      .inner_join(person::table)
      .inner_join(community::table.on(mod_hide_community::community_id.eq(community::id)))
      .select((
        mod_hide_community::all_columns,
        Person::safe_columns_tuple(),
        Community::safe_columns_tuple(),
      ))
      .into_boxed();

    if let Some(community_id) = community_id {
      query = query.filter(mod_hide_community::community_id.eq(community_id));
    };

    let (limit, offset) = limit_and_offset(page, limit);

    let res = query
      .limit(limit)
      .offset(offset)
      .order_by(mod_hide_community::when_.desc())
      .load::<ModHideCommunityViewTuple>(conn)?;

    Ok(Self::from_tuple_to_vec(res))
  }
}

impl ViewToVec for ModHideCommunityView {
  type DbTuple = ModHideCommunityViewTuple;
  fn from_tuple_to_vec(items: Vec<Self::DbTuple>) -> Vec<Self> {
    items
      .iter()
      .map(|a| Self {
        mod_hide_community: a.0.to_owned(),
        admin: a.1.to_owned(),
        community: a.2.to_owned(),
      })
      .collect::<Vec<Self>>()
  }
}