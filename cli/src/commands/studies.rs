use clap::{Subcommand, ValueEnum};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::VariantKey;
use lichess_api::model::studies::*;
use reqwest;

use crate::output;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Clone, ValueEnum)]
pub enum Variant {
    Standard,
    Chess960,
    Crazyhouse,
    Antichess,
    Atomic,
    Horde,
    KingOfTheHill,
    RacingKings,
    ThreeCheck,
    FromPosition,
}

impl From<Variant> for VariantKey {
    fn from(variant: Variant) -> Self {
        match variant {
            Variant::Standard => VariantKey::Standard,
            Variant::Chess960 => VariantKey::Chess960,
            Variant::Crazyhouse => VariantKey::Crazyhouse,
            Variant::Antichess => VariantKey::Antichess,
            Variant::Atomic => VariantKey::Atomic,
            Variant::Horde => VariantKey::Horde,
            Variant::KingOfTheHill => VariantKey::KingOfTheHill,
            Variant::RacingKings => VariantKey::RacingKings,
            Variant::ThreeCheck => VariantKey::ThreeCheck,
            Variant::FromPosition => VariantKey::FromPosition,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum StudyVisibility {
    Public,
    Unlisted,
    Private,
}

impl From<StudyVisibility> for create::Visibility {
    fn from(visibility: StudyVisibility) -> Self {
        match visibility {
            StudyVisibility::Public => create::Visibility::Public,
            StudyVisibility::Unlisted => create::Visibility::Unlisted,
            StudyVisibility::Private => create::Visibility::Private,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum UserSelection {
    Nobody,
    Owner,
    Contributor,
    Member,
    Everyone,
}

impl From<UserSelection> for create::StudyUserSelection {
    fn from(selection: UserSelection) -> Self {
        match selection {
            UserSelection::Nobody => create::StudyUserSelection::Nobody,
            UserSelection::Owner => create::StudyUserSelection::Owner,
            UserSelection::Contributor => create::StudyUserSelection::Contributor,
            UserSelection::Member => create::StudyUserSelection::Member,
            UserSelection::Everyone => create::StudyUserSelection::Everyone,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum StudiesCommand {
    /// Create a study
    Create {
        /// Study name
        name: String,
        /// Visibility
        #[arg(long, value_enum, default_value = "public")]
        visibility: StudyVisibility,
        /// Flair emoji code
        #[arg(long)]
        flair: Option<String>,
        /// Who can use the computer analysis
        #[arg(long, value_enum, default_value = "everyone")]
        computer: UserSelection,
        /// Who can use the opening explorer
        #[arg(long, value_enum, default_value = "everyone")]
        explorer: UserSelection,
        /// Who can clone the study
        #[arg(long, value_enum, default_value = "everyone")]
        cloneable: UserSelection,
        /// Who can view/share the study
        #[arg(long, value_enum, default_value = "everyone")]
        shareable: UserSelection,
        /// Who can use the study chat
        #[arg(long, value_enum, default_value = "everyone")]
        chat: UserSelection,
        /// Direct new contributions to the last chapter
        #[arg(long)]
        sticky: Option<bool>,
    },
    /// Import a PGN into a study, as a new chapter
    ImportPgn {
        /// Study ID
        study_id: String,
        /// Chapter name
        name: String,
        /// PGN text
        pgn: String,
        /// Chess variant
        #[arg(long, value_enum)]
        variant: Option<Variant>,
        /// Board orientation, "white" or "black"
        #[arg(long)]
        orientation: Option<String>,
    },
    /// Export one chapter of a study as PGN
    ExportChapterPgn {
        /// Study ID
        study_id: String,
        /// Chapter ID
        chapter_id: String,
        /// Include clock comments
        #[arg(long)]
        clocks: Option<bool>,
        /// Include move comments
        #[arg(long)]
        comments: Option<bool>,
        /// Include variations
        #[arg(long)]
        variations: Option<bool>,
        /// Include the board orientation as a PGN tag
        #[arg(long)]
        orientation: Option<bool>,
    },
    /// Export a whole study as PGN
    ExportStudyPgn {
        /// Study ID
        study_id: String,
        /// Include clock comments
        #[arg(long)]
        clocks: Option<bool>,
        /// Include move comments
        #[arg(long)]
        comments: Option<bool>,
        /// Include variations
        #[arg(long)]
        variations: Option<bool>,
        /// Include the board orientation as a PGN tag
        #[arg(long)]
        orientation: Option<bool>,
    },
    /// Check whether a study exists and you have access to it
    Metadata {
        /// Study ID
        study_id: String,
    },
    /// Update the PGN tags of a study chapter
    UpdateChapterTags {
        /// Study ID
        study_id: String,
        /// Chapter ID
        chapter_id: String,
        /// PGN containing the new tags
        pgn: String,
    },
    /// Update the move tree of a study chapter
    UpdateChapterMoves {
        /// Study ID
        study_id: String,
        /// Chapter ID
        chapter_id: String,
        /// PGN containing the new moves
        pgn: String,
    },
    /// Export all studies of a user as PGN
    ExportUserStudiesPgn {
        /// Username
        username: String,
        /// Include clock comments
        #[arg(long)]
        clocks: Option<bool>,
        /// Include move comments
        #[arg(long)]
        comments: Option<bool>,
        /// Include variations
        #[arg(long)]
        variations: Option<bool>,
        /// Include the board orientation as a PGN tag
        #[arg(long)]
        orientation: Option<bool>,
    },
    /// List the metadata of all studies of a user
    ListUserStudies {
        /// Username
        username: String,
    },
    /// Delete a study chapter
    DeleteChapter {
        /// Study ID
        study_id: String,
        /// Chapter ID
        chapter_id: String,
    },
}

impl StudiesCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            StudiesCommand::Create {
                name,
                visibility,
                flair,
                computer,
                explorer,
                cloneable,
                shareable,
                chat,
                sticky,
            } => {
                let form = create::CreateStudyForm {
                    name,
                    visibility: visibility.into(),
                    flair,
                    computer: computer.into(),
                    explorer: explorer.into(),
                    cloneable: cloneable.into(),
                    shareable: shareable.into(),
                    chat: chat.into(),
                    sticky,
                    description: None,
                };
                let study = lichess
                    .create_study(form)
                    .await
                    .wrap_err("failed to create study")?;
                output::print(&study, json);
                Ok(())
            }
            StudiesCommand::ImportPgn {
                study_id,
                name,
                pgn,
                variant,
                orientation,
            } => {
                let body = import_pgn_into_study::ImportPgnBody {
                    name,
                    pgn,
                    variant: variant.map(Into::into),
                    orientation,
                };
                let request = import_pgn_into_study::PostRequest::new(study_id.clone(), body);
                let chapters = lichess
                    .import_pgn_into_study(request)
                    .await
                    .wrap_err_with(|| format!("failed to import pgn into study '{study_id}'"))?;
                output::print(&chapters, json);
                Ok(())
            }
            StudiesCommand::ExportChapterPgn {
                study_id,
                chapter_id,
                clocks,
                comments,
                variations,
                orientation,
            } => {
                let query = export_chapter::GetQuery {
                    options: PgnExportQuery {
                        clocks,
                        comments,
                        variations,
                        orientation,
                    },
                };
                let mut stream = lichess
                    .export_study_chapter_pgn(&study_id, &chapter_id, query)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to export chapter '{chapter_id}' of study '{study_id}'")
                    })?;
                while let Some(chunk) = stream.next().await {
                    let chunk = chunk.wrap_err("failed to read pgn stream")?;
                    println!("{chunk}");
                }
                Ok(())
            }
            StudiesCommand::ExportStudyPgn {
                study_id,
                clocks,
                comments,
                variations,
                orientation,
            } => {
                let query = export_study::GetQuery {
                    options: PgnExportQuery {
                        clocks,
                        comments,
                        variations,
                        orientation,
                    },
                };
                let mut stream = lichess
                    .export_study_pgn(&study_id, query)
                    .await
                    .wrap_err_with(|| format!("failed to export study '{study_id}'"))?;
                while let Some(chunk) = stream.next().await {
                    let chunk = chunk.wrap_err("failed to read pgn stream")?;
                    println!("{chunk}");
                }
                Ok(())
            }
            StudiesCommand::Metadata { study_id } => {
                lichess
                    .get_study_metadata(study_id.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to fetch metadata for study '{study_id}'"))?;
                println!("Study '{study_id}' exists and is accessible");
                Ok(())
            }
            StudiesCommand::UpdateChapterTags {
                study_id,
                chapter_id,
                pgn,
            } => {
                let form = update_chapter_tags::UpdateChapterTagsForm { pgn };
                lichess
                    .update_study_chapter_tags(&study_id, &chapter_id, form)
                    .await
                    .wrap_err_with(|| {
                        format!(
                            "failed to update tags of chapter '{chapter_id}' in study '{study_id}'"
                        )
                    })?;
                println!("Tags updated for chapter '{chapter_id}' in study '{study_id}'");
                Ok(())
            }
            StudiesCommand::UpdateChapterMoves {
                study_id,
                chapter_id,
                pgn,
            } => {
                let form = update_chapter_moves::UpdateChapterMovesForm { pgn };
                lichess
                    .update_study_chapter_moves(&study_id, &chapter_id, form)
                    .await
                    .wrap_err_with(|| {
                        format!(
                            "failed to update moves of chapter '{chapter_id}' in study '{study_id}'"
                        )
                    })?;
                println!("Moves updated for chapter '{chapter_id}' in study '{study_id}'");
                Ok(())
            }
            StudiesCommand::ExportUserStudiesPgn {
                username,
                clocks,
                comments,
                variations,
                orientation,
            } => {
                let query = export_user_studies::GetQuery {
                    options: PgnExportQuery {
                        clocks,
                        comments,
                        variations,
                        orientation,
                    },
                };
                let mut stream = lichess
                    .export_user_studies_pgn(&username, query)
                    .await
                    .wrap_err_with(|| format!("failed to export studies of user '{username}'"))?;
                while let Some(chunk) = stream.next().await {
                    let chunk = chunk.wrap_err("failed to read pgn stream")?;
                    println!("{chunk}");
                }
                Ok(())
            }
            StudiesCommand::ListUserStudies { username } => {
                let mut stream = lichess
                    .list_user_studies(username.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to list studies of user '{username}'"))?;
                while let Some(study) = stream.next().await {
                    match study {
                        Ok(study) => output::print(&study, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            StudiesCommand::DeleteChapter {
                study_id,
                chapter_id,
            } => {
                lichess
                    .delete_study_chapter(&study_id, &chapter_id)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to delete chapter '{chapter_id}' from study '{study_id}'")
                    })?;
                println!("Chapter '{chapter_id}' deleted from study '{study_id}'");
                Ok(())
            }
        }
    }
}
