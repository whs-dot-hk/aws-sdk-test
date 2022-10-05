use aws_config::meta::region::RegionProviderChain;
use aws_sdk_imagebuilder::model::{Filter, ImageRecipeSummary};
use aws_sdk_imagebuilder::{Client, Error};
use semver::Version;

fn get_version(s: &ImageRecipeSummary) -> Version {
    Version::parse(s.arn.as_ref().unwrap().split("/").last().unwrap()).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    //let mut next_token: Option<String> = None;
    //let mut summary: Option<ImageRecipeSummary> = None;

    //loop {
    //    let resp = client
    //        .list_image_recipes()
    //        .filters(
    //            Filter::builder()
    //                .name("name")
    //                .values("whslabs-cardano-node")
    //                .build(),
    //        )
    //        .set_next_token(next_token.take())
    //        .send()
    //        .await?;

    //    let mut summary_list = resp.image_recipe_summary_list.unwrap();
    //    summary_list.sort_by_key(|k| get_version(k));

    //    let new = summary_list.last().unwrap().clone();

    //    match summary.clone() {
    //        Some(old) => {
    //            if get_version(&new) > get_version(&old) {
    //                summary = Some(new);
    //            }
    //        }
    //        None => {
    //            summary = Some(new);
    //            continue;
    //        }
    //    }

    //    next_token = resp.next_token;
    //    if next_token == None {
    //        break;
    //    }
    //}

    //let req = client
    //    .get_image_recipe()
    //    .set_image_recipe_arn(summary.unwrap().arn);
    //let resp = req.send().await?;
    //println!("{:?}", resp);
    //let r = resp.image_recipe.unwrap();
    //println!("{:?}", r);

    //let mut new_version = Version::parse(&r.version.unwrap()).unwrap();
    //new_version.patch += 1;

    //let req = client
    //    .create_image_recipe()
    //    .set_name(r.name)
    //    .semantic_version(new_version.to_string())
    //    .components(r.components.unwrap().first().unwrap().clone())
    //    .parent_image(r.parent_image.unwrap())
    //    .block_device_mappings(r.block_device_mappings.unwrap().first().unwrap().clone());

    //let resp = req.send().await?;
    //println!("{:?}", resp);

    let request = client
        .list_image_pipelines()
        .filters(
            Filter::builder()
                .name("name")
                .values("whslabs-cardano-node")
                .build(),
        )
        .send()
        .await?;

    println!("{:?}", request);

    let test = request.image_pipeline_list.unwrap();

    let test2 = test.first().unwrap();

    println!("{:?}", test2);

    let request = client
        .update_image_pipeline()
        .image_pipeline_arn(
            test2.arn.as_ref().unwrap()
        )
        .image_recipe_arn(
            "arn:aws:imagebuilder:us-east-1:102933037533:image-recipe/whslabs-cardano-node/1.0.13",
        )
        .infrastructure_configuration_arn(test2.infrastructure_configuration_arn.as_ref().unwrap())
        .send()
        .await?;
    println!("{:?}", request);

    Ok(())
}
