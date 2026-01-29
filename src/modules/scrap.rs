use chrono::Local;
use regex::Regex;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
// Helper pour l'échappement XML
fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ScrapeResult {
    pub id: i64,
    pub title: String,
    pub original_title: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub release_date: String,
    pub vote_average: f64,
    pub vote_count: u64,
    pub genres: Vec<String>,
    pub studios: Vec<String>,
    pub actors: Vec<Actor>,
    pub director: Option<String>,
    pub runtime: u32,
    pub tagline: String,
    pub imdb_id: Option<String>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Actor {
    pub name: String,
    pub role: String,
    pub thumb: Option<String>,
    pub id: i64,
}
pub fn download_image_bytes(poster_path: &str) -> Option<Vec<u8>> {
    let url = format!("https://image.tmdb.org/t/p/w92{}", poster_path);
    let client = Client::new();
    if let Ok(res) = client.get(url).send() {
        if let Ok(bytes) = res.bytes() {
            return Some(bytes.to_vec());
        }
    }
    None
}
pub fn save_metadata(input_path: PathBuf, data: ScrapeResult, is_series: bool) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let year = data
        .release_date
        .split('-')
        .next()
        .unwrap_or("")
        .to_string();
    let tag = if is_series { "tvshow" } else { "movie" };
    let filename = input_path.file_name().unwrap_or_default().to_string_lossy();
    let base_name = input_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let parent_dir = input_path.parent().unwrap_or(&input_path);

    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n");
    xml.push_str(&format!("{}\n", now));
    xml.push_str(&format!("<{}>\n", tag));
    xml.push_str(&format!("  <title>{}</title>\n", escape_xml(&data.title)));
    xml.push_str(&format!(
        "  <originaltitle>{}</originaltitle>\n",
        escape_xml(&data.original_title)
    ));
    xml.push_str(&format!("  <year>{}</year>\n", year));

    xml.push_str("  <ratings>\n    <rating default=\"false\" max=\"10\" name=\"themoviedb\">\n");
    xml.push_str(&format!("      <value>{}</value>\n", data.vote_average));
    xml.push_str(&format!("      <votes>{}</votes>\n", data.vote_count));
    xml.push_str("    </rating>\n  </ratings>\n");

    xml.push_str("  <userrating>0</userrating>\n  <top250>0</top250>\n");
    xml.push_str(&format!("  <plot>{}</plot>\n", escape_xml(&data.overview)));
    xml.push_str(&format!(
        "  <outline>{}</outline>\n",
        escape_xml(&data.overview)
    ));
    xml.push_str(&format!(
        "  <tagline>{}</tagline>\n",
        escape_xml(&data.tagline)
    ));
    xml.push_str(&format!("  <runtime>{}</runtime>\n", data.runtime));

    if let Some(poster) = &data.poster_path {
        xml.push_str(&format!(
            "  <thumb aspect=\"poster\">https://image.tmdb.org/t/p/original{}</thumb>\n",
            poster
        ));
    }
    xml.push_str(&format!("  <tmdbid>{}</tmdbid>\n", data.id));
    xml.push_str(&format!(
        "  <uniqueid default=\"true\" type=\"tmdb\">{}</uniqueid>\n",
        data.id
    ));
    if let Some(imdb) = &data.imdb_id {
        xml.push_str(&format!(
            "  <uniqueid default=\"false\" type=\"imdb\">{}</uniqueid>\n",
            imdb
        ));
    }
    xml.push_str(&format!("  <premiered>{}</premiered>\n", data.release_date));
    xml.push_str("  <watched>false</watched>\n  <playcount>0</playcount>\n");
    for genre in &data.genres {
        xml.push_str(&format!("  <genre>{}</genre>\n", escape_xml(genre)));
    }
    for studio in &data.studios {
        xml.push_str(&format!("  <studio>{}</studio>\n", escape_xml(studio)));
    }
    if let Some(d) = &data.director {
        xml.push_str(&format!("  <director>{}</director>\n", escape_xml(d)));
    }
    for actor in &data.actors {
        xml.push_str("  <actor>\n");
        xml.push_str(&format!("    <name>{}</name>\n", escape_xml(&actor.name)));
        xml.push_str(&format!("    <role>{}</role>\n", escape_xml(&actor.role)));
        if let Some(t) = &actor.thumb {
            xml.push_str(&format!(
                "    <thumb>https://image.tmdb.org/t/p/h632{}</thumb>\n",
                t
            ));
        }
        xml.push_str(&format!("    <tmdbid>{}</tmdbid>\n", actor.id));
        xml.push_str("  </actor>\n");
    }
    xml.push_str("  <fileinfo>\n    <streamdetails>\n    </streamdetails>\n  </fileinfo>\n");
    xml.push_str(&format!(
        "  <original_filename>{}</original_filename>\n",
        escape_xml(&filename)
    ));
    xml.push_str(&format!("</{}>\n", tag));
    let _ = fs::write(input_path.with_extension("nfo"), xml);

    // Téléchargement des images
    let client = Client::new();

    // Poster
    if let Some(ref path) = data.poster_path {
        let poster_url = format!("https://image.tmdb.org/t/p/original{}", path);
        let out_path = parent_dir.join(format!("{}-poster.jpg", base_name));
        let client_clone = client.clone();
        std::thread::spawn(move || {
            if let Ok(res) = client_clone.get(poster_url).send() {
                if let Ok(bytes) = res.bytes() {
                    let _ = fs::write(out_path, bytes);
                }
            }
        });
    }

    // Fanart (backdrop)
    if let Some(ref path) = data.backdrop_path {
        let fanart_url = format!("https://image.tmdb.org/t/p/original{}", path);
        let out_path = parent_dir.join(format!("{}-fanart.jpg", base_name));
        let client_clone = client.clone();
        std::thread::spawn(move || {
            if let Ok(res) = client_clone.get(fanart_url).send() {
                if let Ok(bytes) = res.bytes() {
                    let _ = fs::write(out_path, bytes);
                }
            }
        });
    }

    // ClearLogo depuis Fanart.tv
    let tmdb_id = data.id;
    let fanart_api_key = std::env::var("FANART_KEY").expect("FANART_KEY missing");
    let logo_url = if is_series {
        format!(
            "https://webservice.fanart.tv/v3/tv/{}?api_key={}",
            tmdb_id, fanart_api_key
        )
    } else {
        format!(
            "https://webservice.fanart.tv/v3/movies/{}?api_key={}",
            tmdb_id, fanart_api_key
        )
    };

    let out_logo_path = parent_dir.join(format!("{}-clearlogo.png", base_name));
    std::thread::spawn(move || {
        if let Ok(res) = client.get(&logo_url).send() {
            if let Ok(json) = res.json::<Value>() {
                let logo_path = if is_series {
                    json["hdtvlogo"]
                        .as_array()
                        .or_else(|| json["clearlogo"].as_array())
                        .and_then(|arr| arr.first())
                        .and_then(|v| v["url"].as_str())
                } else {
                    json["hdmovielogo"]
                        .as_array()
                        .or_else(|| json["movielogo"].as_array())
                        .and_then(|arr| arr.first())
                        .and_then(|v| v["url"].as_str())
                };

                if let Some(url) = logo_path {
                    if let Ok(img_res) = Client::new().get(url).send() {
                        if let Ok(bytes) = img_res.bytes() {
                            let _ = fs::write(out_logo_path, bytes);
                        }
                    }
                }
            }
        }
    });
}
pub fn search_tmdb(query: &str, is_series: bool) -> Result<Vec<ScrapeResult>, String> {
    let api_key = std::env::var("TMDB_KEY").expect("TMDB_KEY missing");
    let client = Client::builder()
        .user_agent("OXYON/2.1")
        .build()
        .map_err(|e| e.to_string())?;

    // Nettoyage simplifié pour la recherche
    let re_year = Regex::new(r"\b(19|20)\d{2}\b").unwrap();
    let clean_query = re_year
        .replace_all(query, "")
        .to_string()
        .replace(".", " ")
        .replace("_", " ")
        .replace("-", " ");
    let url = if is_series {
        "https://api.themoviedb.org/3/search/tv"
    } else {
        "https://api.themoviedb.org/3/search/movie"
    };
    let res = client
        .get(url)
        .query(&[
            ("api_key", api_key.as_str()),
            ("language", "fr-FR"),
            ("query", clean_query.trim()),
        ])
        .send()
        .map_err(|e| e.to_string())?
        .json::<Value>()
        .map_err(|e| e.to_string())?;
    let mut list = Vec::new();
    if let Some(results) = res["results"].as_array() {
        for r in results {
            let id = r["id"].as_i64().unwrap_or(0);
            let detail_url = format!(
                "https://api.themoviedb.org/3/{}?api_key={}&language=fr-FR&append_to_response=credits,external_ids",
                if is_series {
                    format!("tv/{}", id)
                } else {
                    format!("movie/{}", id)
                },
                api_key
            );

            if let Ok(d) = client
                .get(detail_url)
                .send()
                .and_then(|resp| resp.json::<Value>())
            {
                // Mapping des acteurs, réalisateurs, runtime
                let imdb_id = d["external_ids"]["imdb_id"].as_str().map(|s| s.to_string());

                let mut actors = Vec::new();
                if let Some(cast) = d["credits"]["cast"].as_array() {
                    for a in cast.iter().take(15) {
                        actors.push(Actor {
                            name: a["name"].as_str().unwrap_or("").to_string(),
                            role: a["character"].as_str().unwrap_or("").to_string(),
                            thumb: a["profile_path"].as_str().map(|s| s.to_string()),
                            id: a["id"].as_i64().unwrap_or(0),
                        });
                    }
                }
                let director = if !is_series {
                    d["credits"]["crew"]
                        .as_array()
                        .and_then(|crew| {
                            crew.iter()
                                .find(|m| m["job"] == "Director")
                                .and_then(|m| m["name"].as_str())
                        })
                        .map(|s| s.to_string())
                } else {
                    d["created_by"]
                        .as_array()
                        .and_then(|c| c.first())
                        .and_then(|m| m["name"].as_str())
                        .map(|s| s.to_string())
                };
                let runtime = if is_series {
                    d["episode_run_time"]
                        .as_array()
                        .and_then(|a| a.first())
                        .and_then(|v| v.as_u64())
                        .unwrap_or(45)
                } else {
                    d["runtime"].as_u64().unwrap_or(0)
                };
                list.push(ScrapeResult {
                    id,
                    title: d[if is_series { "name" } else { "title" }]
                        .as_str()
                        .unwrap_or("Inconnu")
                        .to_string(),
                    original_title: d[if is_series {
                        "original_name"
                    } else {
                        "original_title"
                    }]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                    overview: d["overview"].as_str().unwrap_or("").to_string(),
                    poster_path: d["poster_path"].as_str().map(|s| s.to_string()),
                    backdrop_path: d["backdrop_path"].as_str().map(|s| s.to_string()),
                    release_date: d[if is_series {
                        "first_air_date"
                    } else {
                        "release_date"
                    }]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                    vote_average: d["vote_average"].as_f64().unwrap_or(0.0),
                    vote_count: d["vote_count"].as_u64().unwrap_or(0),
                    runtime: runtime as u32,
                    tagline: d["tagline"].as_str().unwrap_or("").to_string(),
                    genres: d["genres"]
                        .as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|g| g["name"].as_str().unwrap_or("").to_string())
                        .collect(),
                    studios: d["production_companies"]
                        .as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|s| s["name"].as_str().unwrap_or("").to_string())
                        .collect(),
                    actors,
                    director,
                    imdb_id,
                });
            }
        }
    }
    Ok(list)
}
