mod article;
mod comment;
mod context;
mod error;
mod mutation;
mod profile;
mod query;
mod tag;
mod user;

use context::Context;
use juniper::{EmptySubscription, GraphQLObject, RootNode};
use juniper_warp::{make_graphql_filter, playground_filter};
use mutation::Mutation;
use query::Query;
use server::{warp, ServerState};
use warp::{Filter, Rejection, Reply};

pub struct Gql;

impl Gql {
    pub fn routes(
        state: ServerState,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let graphql = warp::path!("graphql")
            .and(warp::post())
            .and(make_graphql_filter(
                schema(),
                Context::extract(state).boxed(),
            ));

        let playground = warp::path!("graphql")
            .and(warp::get())
            .and(playground_filter("/graphql", None));

        graphql.or(playground)
    }
}

type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<Context>::new())
}

#[derive(GraphQLObject)]
pub struct OperationResult {
    success: bool,
}
impl From<()> for OperationResult {
    fn from(_: ()) -> Self {
        OperationResult { success: true }
    }
}
