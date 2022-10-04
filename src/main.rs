use aws_config::meta::region::RegionProviderChain;
use aws_sdk_imagebuilder::model::Filter;
use aws_sdk_imagebuilder::{Client, Error};
use semver::Version;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let resp = client
        .list_image_recipes()
        .filters(
            Filter::builder()
                .name("name")
                .values("whslabs-cardano-node")
                .build(),
        )
        //.max_results(1)
        .send()
        .await?;
    //println!("{:?}", resp);

    let mut summaries = resp.image_recipe_summary_list.unwrap().clone();
    summaries.sort_by(|b, a| a.clone().arn.unwrap().cmp(&b.clone().arn.unwrap()));
    println!("{:?}", summaries);
    //println!("{:?}", Version::parse(&summaries.first().unwrap().clone().arn.unwrap()).unwrap());
    println!("{:?}", Version::parse(summaries.first().unwrap().clone().arn.unwrap().split("/").last().unwrap()));

    summaries.sort_by_key(|k| Version::parse(k.clone().arn.unwrap().split("/").last().unwrap()).unwrap());
    println!("{:?}", summaries);

    //let req = client.get_image_recipe().image_recipe_arn(
    //    "arn:aws:imagebuilder:us-east-1:102933037533:image-recipe/whslabs-cardano-node/1.0.0",
    //);
    //let resp = req.send().await?;
    //println!("{:?}", resp);
    //let r = resp.image_recipe.unwrap();
    //println!("{:?}", r);

    //let req = client.get_image_recipe().image_recipe_arn(
    //    "arn:aws:imagebuilder:us-east-1:102933037533:image-recipe/whslabs-cardano-node/1.0.0",
    //);
    //let resp = req.send().await?;
    //println!("{:?}", resp);
    //let r = resp.image_recipe.unwrap();
    //println!("{:?}", r);

    //let req = client
    //    .create_image_recipe()
    //    .name("whslabs-cardano-node")
    //    .semantic_version("1.0.11")
    //    .components(r.components.unwrap().first().unwrap().clone())
    //    .parent_image(r.parent_image.unwrap())
    //    .block_device_mappings(r.block_device_mappings.unwrap().first().unwrap().clone());
    //
    //let resp = req.send().await?;
    //println!("{:?}", resp);
    Ok(())
}
