use bytes::Buf as _;

const BASE_URL: &str = "https://dl.google.com/android/repository";
const REPOSITORY_DIR: &str = "repository";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tokio::fs::remove_dir_all(REPOSITORY_DIR).await?;
    let client = reqwest::Client::new();
    tokio::try_join!(update_addons(&client), update_repository(&client))?;
    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct SiteList {
    #[serde(rename = "site")]
    sites: Vec<Site>,
}

#[derive(Debug, serde::Deserialize)]
struct Site {
    url: String,
}

async fn update_addons(client: &reqwest::Client) -> anyhow::Result<()> {
    // addons_list versions are listed here as XML schema definition.
    // https://android.googlesource.com/platform/tools/base/+/refs/heads/mirror-goog-studio-main/sdklib/src/main/resources/xsd/sources/
    const VERSIONS: &[u8] = &[3, 4, 5];

    let mut set = tokio::task::JoinSet::new();

    for version in VERSIONS {
        let client = client.clone();
        set.spawn(async move {
            let filename = format!("addons_list-{}.xml", version);
            let body = download(&client, &filename).await?;
            let site_list: SiteList = quick_xml::de::from_reader(body.reader())?;
            for site in site_list.sites {
                download(&client, &site.url).await?;
            }
            Ok::<_, anyhow::Error>(())
        });
    }

    while set.join_next().await.transpose()?.transpose()?.is_some() {}
    Ok(())
}

async fn update_repository(client: &reqwest::Client) -> anyhow::Result<()> {
    // repository versions are listed here as XML schema definition.
    // https://android.googlesource.com/platform/tools/base/+/refs/heads/mirror-goog-studio-main/sdklib/src/main/resources/xsd/
    const VERSIONS: &[u8] = &[1, 2, 3];

    let mut set = tokio::task::JoinSet::new();

    for version in VERSIONS {
        let client = client.clone();
        set.spawn(async move {
            let filename = format!("repository2-{}.xml", version);
            download(&client, &filename).await?;
            Ok::<_, anyhow::Error>(())
        });
    }

    while set.join_next().await.transpose()?.transpose()?.is_some() {}
    Ok(())
}

async fn download(client: &reqwest::Client, filename: &str) -> anyhow::Result<bytes::Bytes> {
    let url = format!("{}/{}", BASE_URL, filename);
    println!("Downloading {}", url);
    let body = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    let path = format!("{}/{}", REPOSITORY_DIR, filename);
    tokio::fs::create_dir_all(std::path::Path::new(&path).parent().unwrap()).await?;
    tokio::fs::write(&path, &body).await?;
    Ok(body)
}
