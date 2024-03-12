use image::DynamicImage;

// bubblesort implemented for this version TODO: faster algo here
pub fn lg_to_sm(images: &mut Vec<DynamicImage>) {
    let mut change: bool = true;
    let mut i: u32;

    while change == true {
        i = 0;
        change = false;

        for _ in images.clone().iter() {

            if i + 1 < images.len().try_into().unwrap() {

                let i_area_1: u32 = images[i as usize].height() * images[i as usize].width();
                let i_area_2: u32 = images[(i+1) as usize].height() * images[(i+1) as usize].width();

                // check & swap
                if i_area_1 < i_area_2 {
                    let tmp = images[i as usize].clone();
                    images[i as usize] = images[(i+1) as usize].clone();
                    images[(i+1) as usize] = tmp;
                    change = true;
                    }
                i += 1;
            }
        }
    }

}
