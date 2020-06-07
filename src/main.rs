mod walking;
use dirs;

fn main() {
    let get_root = dirs::home_dir();

    match get_root {
        Some(val) => match val.to_str() {
            Some(root) => {
                println!("{}", root);

                let res = walking::walking(root).unwrap();
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
                let finall: Vec<(String, String)> = res
                    .into_iter()
                    .filter(|i| {
                        let xx = dub_signatures.clone().into_iter().find(|x| x == &i.1);
                        match xx {
                            Some(_) => true,
                            None => false,
                        }
                    })
                    .collect();
                println!("dub_signatures {:?}", dub_signatures);
                println!("finall {:?}", finall);
            }
            None => (),
        },
        None => (),
    }
}
