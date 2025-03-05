use std::sync::Arc;

use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use qdrant_client::{
    Qdrant,
    qdrant::{
        Condition, DiscoverPointsBuilder, Filter, QueryPointsBuilder, RecommendInputBuilder,
        RecommendStrategy, SearchParamsBuilder, SearchPointsBuilder, VectorExample,
        target_vector::Target, vector_example::Example,
    },
};
use ti_core::embedding::TextEmbedder;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DataVerse {
    pub book: String,
    pub chapter: i64,
    pub verse: i64,
    pub text: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <argument>", args[0]);
        std::process::exit(1);
    }

    let term = &args[1];

    let em = TextEmbedder::new()?;
    let client = Arc::new(Qdrant::from_url("http://localhost:6334").build()?);

    let embed = &em.embed(term.as_str())?[0];
    // let found = client
    //     .query(
    //         QueryPointsBuilder::new("lxx")
    //             .query(
    //                 RecommendInputBuilder::default()
    //                     .add_positive(embed.clone())
    //                     .strategy(RecommendStrategy::AverageVector)
    //                     .build(),
    //             )
    //             .limit(3)
    //             .filter(Filter::must([Condition::matches(
    //                 "city",
    //                 "London".to_string(),
    //             )])),
    //     )
    //     .await?;

    // let found = client
    //     .discover(
    //         DiscoverPointsBuilder::new(
    //             "lxx", // Collection name
    //             vec![],
    //             10,
    //         )
    //         .target(Target::Single(VectorExample {
    //             example: Some(Example::Vector(embed.clone().into())),
    //         })),
    //     )
    //     .await?;
    // let found = client
    //     .query(
    //         QueryPointsBuilder::new("lxx")
    //             .query(embed.clone())
    //             .limit(10)
    //             .params(SearchParamsBuilder::default().hnsw_ef(1024).exact(true))
    //             .with_payload(true),
    //     )
    //     .await?;
    let found = client
        .search_points(
            SearchPointsBuilder::new("lxx", embed.clone(), 10)
                .with_payload(true)
                .params(SearchParamsBuilder::default().hnsw_ef(1024).exact(true)),
        )
        .await?;

    for result in found.result {
        println!("Score: {}", result.score);
        let val = serde_json::to_value(&result.payload)?;
        let payload: DataVerse = serde_json::from_value(val)?;
        println!("Payload: {:#?}", payload);
    }

    Ok(())
}
