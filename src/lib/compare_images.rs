use diffimg;
use image;
use std::io;
use tokio::task;

pub async fn compare_images(
    image_paths_1: Vec<String>,
    image_paths_2: Vec<String>,
) -> Result<(), io::Error> {
    let mut handles = Vec::new();

    let path_pairs: Vec<(String, String)> = image_paths_1
        .into_iter()
        .map(|image_1| {
            let image_2 = image_paths_2
                .iter()
                .find(|&r| &r.split('/').last() == &image_1.split('/').last());
            match image_2 {
                Some(image_2) => (image_1.to_string(), image_2.to_string()),
                None => (image_1.to_string(), "not found".to_string()),
            }
        })
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect();

    for chunk in path_pairs.chunks(10) {
        let chunk = chunk.to_vec(); // clone chunk to move into async block
        handles.push(task::spawn(async move {
            for (new, old) in chunk {
                if old == "not found" || new == "not found" {
                    println!("Old image not found for: {}", new);
                    continue;
                }

                let new_image = image::open(&new).unwrap();
                let old_image = image::open(&old).unwrap();

                let file_name = format!("assets/diff/{}", new.split('/').last().unwrap());
                diffimg::get_diff_from_images(new_image, old_image, file_name.as_str()).unwrap();
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}
