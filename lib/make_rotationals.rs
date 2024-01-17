use std::{fs::File, io::BufReader};

use clap::Parser;
use dmi::{
    dirs::Dirs,
    icon::{dir_to_dmi_index, Icon, IconState},
};
use image::DynamicImage;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    filename: std::path::PathBuf,
    #[arg(long)]
    statename: String,
}

fn main() {
    let args = Cli::parse();
    let filename = args.filename;
    let statename = args.statename;
    let file = File::open(&filename).unwrap();

    let reader = BufReader::new(file);
    let dmi = Icon::load(reader).unwrap();

    for state in &dmi.states {
        if state.name == statename {
            if state.dirs != 1 {
                panic!("state already has more than 1 direction");
            }

            let mut south_image_data: Vec<DynamicImage> = vec![];
            for frame in 1..state.frames + 1 {
                south_image_data.push(state.get_image(&Dirs::SOUTH, frame).unwrap().clone());
            }

            let north_image_data: &Vec<DynamicImage> = &south_image_data
                .iter()
                .map(|data| data.rotate180())
                .collect();

            let east_image_data: &Vec<DynamicImage> = &south_image_data
                .iter()
                .map(|data| data.rotate270())
                .collect();

            let west_image_data: &Vec<DynamicImage> = &south_image_data
                .iter()
                .map(|data| data.rotate90())
                .collect();

            let mut image_data: Vec<DynamicImage> = vec!();

            for i in 0..south_image_data.len() {
                image_data.push(south_image_data[i].clone());
                image_data.push(north_image_data[i].clone());
                image_data.push(east_image_data[i].clone());
                image_data.push(west_image_data[i].clone());
            }

            let new_states2: Vec<IconState> = dmi
                .states
                .iter()
                .map(|state| IconState {
                    name: state.name.clone(),
                    dirs: if state.name == statename {
                        4
                    } else {
                        state.dirs
                    },
                    frames: state.frames,
                    delay: state.delay.clone(),
                    images: if state.name == statename {
                        image_data.clone()
                    } else {
                        state.images.clone()
                    },
                    loop_flag: state.loop_flag,
                    rewind: state.rewind,
                    movement: state.movement,
                    hotspot: state.hotspot,
                    unknown_settings: state.unknown_settings.clone(),
                })
                .collect();

            let new_icon: Icon = Icon {
                version: dmi.version,
                width: dmi.width,
                height: dmi.height,
                states: new_states2,
            };

            let mut write_file =
                File::create(filename.as_path()).expect("Failed to create dmi file");
            new_icon
                .save(&mut write_file)
                .expect("Failed to save DMI file");

            return;
        }
    }
}
