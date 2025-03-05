use std::{fs::File, sync::Arc};

use anyhow::Context;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use qdrant_client::{
    Payload, Qdrant,
    qdrant::{PointStruct, UpsertPointsBuilder},
};
use serde_json::json;
use ti_core::{
    embedding::TextEmbedder,
    models::data_file::{Chapter, DataFile, DataFiles, Verse},
};
use tokio::task::JoinSet;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let em = TextEmbedder::new()?;

    let client = Arc::new(
        Qdrant::from_url("http://localhost:6334")
            .keep_alive_while_idle()
            .timeout(std::time::Duration::from_secs(60))
            .build()?,
    );

    let DataFiles(entries) = DataFiles::new_from_dir(".data/text/lxx")?;
    let mut js = JoinSet::new();

    let m = Arc::new(MultiProgress::new());

    for DataFile { book, chapters } in entries {
        let book = book.clone();
        let em = em.clone();

        let client = Arc::clone(&client);
        let m = Arc::clone(&m);
        let pb_book = m.add(ProgressBar::new(chapters.len() as u64));
        pb_book.set_style(
            ProgressStyle::default_bar()
                .template(
                    "[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} Chapters ({percent}%)",
                )
                .unwrap()
                .progress_chars("=> "),
        );

        js.spawn(async move {
            for Chapter { chapter, verses } in chapters {

                let pb_chapter = m.add(ProgressBar::new(verses.len() as u64));
                pb_chapter.set_style(
                ProgressStyle::default_bar()
                    .template("  -> Chapter {pos}: {bar:40.green/yellow} {pos}/{len} Verses ({percent}%)")
                    .unwrap()
                    .progress_chars("=> "),
                );

                for Verse { verse, text } in verses {
                    let points = em.embed(&text)?.into_iter().map(|vector| PointStruct::new(
                        Uuid::new_v4().to_string(),
                        vector,
                        Payload::try_from(json!({
                            "book": book.clone(),
                            "chapter": (chapter as i64),
                            "verse": (verse as i64),
                            "text": text,
                        })).unwrap()
                    )).collect::<Vec<_>>();
                    let _ = client
                        .upsert_points(
                            UpsertPointsBuilder::new(
                                "lxx",
                                points,
                            )
                            .wait(true),
                        )
                        .await?;
                    pb_chapter.inc(1);
                }
                pb_chapter.finish_and_clear();
                pb_book.inc(1);
            }
            pb_book.finish_with_message(format!("Book {book} complete"));


            Ok::<_, anyhow::Error>(())
        });
    }

    while let Some(r) = js.join_next().await {
        r??
    }

    Ok(())
}
