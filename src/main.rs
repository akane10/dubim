mod walking;
use dirs;

fn main() {
    let get_root = dirs::home_dir();

    match get_root {
        Some(val) => match val.to_str() {
            Some(root) => {
                println!("{}", root);

                let res = walking::walking("/home/yapie/Desktop").unwrap();
                let x: Vec<(String, String)> = res.iter().fold(Vec::new(), |mut acc, val| {
                    let y: Vec<(String, String)> = res
                        .clone()
                        .into_iter()
                        .filter(|x| x.1 == val.1)
                        .collect::<Vec<(String, String)>>();
                    if y.len() > 1 {
                        for i in y {
                            let xx = acc.clone().into_iter().find(|x| x.1 == i.1);
                            match xx {
                                Some(_) => (),
                                None => acc.push(i),
                            }
                        }
                    }
                    acc
                });
                let dub_signatures: Vec<String> = x.into_iter().map(|x| x.1).collect();
                let mut finall: Vec<(String, String)> = res
                    .into_iter()
                    .filter(|i| {
                        let xx = dub_signatures.clone().into_iter().find(|x| x == &i.1);
                        match xx {
                            Some(_) => true,
                            None => false,
                        }
                    })
                    .collect();

                finall.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                let all_paths: Vec<String> = finall.into_iter().map(|i| i.0).collect();

                println!("dub_signatures {:?}", dub_signatures);
                println!("finall {:?}", all_paths);
            }
            None => (),
        },
        None => (),
    }
}
