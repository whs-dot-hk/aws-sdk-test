use aws_config::meta::region::RegionProviderChain;
use aws_sdk_imagebuilder::model::{Filter, ImageRecipeSummary};
use aws_sdk_imagebuilder::{Client, Error};
use semver::Version;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let mut next_token: Option<String> = None;
    let mut summary: Option<ImageRecipeSummary> = None;

    loop {
        let resp = client
            .list_image_recipes()
            .filters(
                Filter::builder()
                    .name("name")
                    .values("whslabs-cardano-node")
                    .build(),
            )
            .max_results(3)
            .set_next_token(next_token)
            .send()
            .await?;
        //println!("{:?}", resp);

        let mut summaries = resp.image_recipe_summary_list.unwrap().clone();
        summaries.sort_by_key(|k| {
            Version::parse(k.clone().arn.unwrap().split("/").last().unwrap()).unwrap()
        });

        let l = summaries.last().unwrap().clone();
        if summary == None {
            summary = Some(l);
        }
        else if Version::parse(l.clone().arn.unwrap().split("/").last().unwrap()).unwrap() > Version::parse(summary.clone().unwrap().arn.unwrap().split("/").last().unwrap()).unwrap() {
            summary = Some(l);
        }

        next_token = resp.next_token;
        if next_token == None {
            break;
        }
    }

    println!("{:?}", summary);
    //let mut summaries = resp.image_recipe_summary_list.unwrap().clone();
    //summaries.sort_by(|b, a| a.clone().arn.unwrap().cmp(&b.clone().arn.unwrap()));
    //println!("{:?}", summaries);
    //println!("{:?}", Version::parse(&summaries.first().unwrap().clone().arn.unwrap()).unwrap());
    //println!("{:?}", Version::parse(summaries.first().unwrap().clone().arn.unwrap().split("/").last().unwrap()));

    //summaries.sort_by_key(|k| Version::parse(k.clone().arn.unwrap().split("/").last().unwrap()).unwrap());
    //println!("{:?}", summaries);

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
