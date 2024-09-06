
fn problem(seed: u64) -> (
    Vec<f32>,
    Vec<f32>, Vec<usize>, Vec<f32>,
    Vec<f32>, Vec<i32>,
    Vec<(usize, usize)>)
{
    use rand::SeedableRng;
    let mut reng = rand_chacha::ChaCha8Rng::seed_from_u64(7);
    let num_room = 6;
    let room2color: Vec<i32> = {
        let mut room2color: Vec<i32> = vec!();
        for _i_room in 0..num_room {
            let c = floorplan::random_room_color(&mut reng);
            dbg!(c,format!("fill=\"#{:06X}\"",c));
            room2color.push(c);
        }
        room2color
    };
    let mut reng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    // let mut reng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    //
    let vtxl2xy = {
        let vtxl2xy = vec!(
            48.5, 59.4,
            88.5, 59.4,
            88.5, 88.0,
            126.6, 88.0,
            126.6, 59.4,
            178.4, 59.4,
            178.4, 113.4,
            197.7, 113.4,
            197.7, 172.5,
            48.5, 172.5,
            48.5, 59.4);
        let vtxl2xy = del_msh_core::polyloop::resample::<f32, 2>(&vtxl2xy, 75);
        let vtxl2xy = del_msh_core::vtx2xdim::to_array_of_nalgebra_vector(&vtxl2xy);
        let vtxl2xy = del_msh_core::vtx2vec::normalize2(&vtxl2xy, &nalgebra::Vector2::<f32>::new(0.5, 0.5), 1.0);
        // dbg!(&vtxl2xy);
        dbg!(vtxl2xy.len());
        del_msh_core::io_obj::save_vtx2vecn_as_polyloop("target/loop.obj", &vtxl2xy);
        del_msh_core::vtx2xdim::from_array_of_nalgebra(&vtxl2xy)
    };
    dbg!(&vtxl2xy);
    let area_ratio = [0.2, 0.4, 0.2, 0.2, 0.2, 0.03];
    assert_eq!(area_ratio.len(), num_room);
    let room2area_trg: Vec<f32> = {
        let total_area = del_msh_core::polyloop2::area_(&vtxl2xy);
        dbg!(total_area);
        let sum_ratio: f32 = area_ratio.iter().sum();
        area_ratio.iter().map(|v| v / sum_ratio * total_area).collect()
    };
    dbg!(&room2area_trg);
    //
    let (site2xy, site2xy2flag, site2room) = {
        let mut site2xy = del_msh_core::sampling::poisson_disk_sampling_from_polyloop2(
            &vtxl2xy, 0.03, 50, &mut reng);
        let mut site2xy2flag = vec!(0f32; site2xy.len());
        let mut site2room = floorplan::site2room(site2xy.len() / 2, &room2area_trg[0..room2area_trg.len() - 1]);
        site2xy.extend([0.1, 0.15]);
        site2xy2flag.extend([1., 1.]);
        site2room.push(room2area_trg.len() - 1);
        site2xy.extend([0.14, 0.15]);
        site2xy2flag.extend([1., 1.]);
        site2room.push(room2area_trg.len() - 1);
        (site2xy, site2xy2flag, site2room)
    };
    assert_eq!(site2xy.len(), site2xy2flag.len());
    let room_connections: Vec<(usize, usize)> = vec!((0, 1), (1, 2), (1, 3), (0, 4), (0, 5));
    // let room_connections: Vec<(usize, usize)> = vec!();
    (vtxl2xy,
     site2xy, site2room, site2xy2flag,
     room2area_trg, room2color,
     room_connections)
}


fn main() -> anyhow::Result<()> {
    let (vtxl2xy,
        site2xy, site2room, site2xy2flag,
        room2area_trg, room2color, room_connections) = problem(7);
    let mut canvas_gif = {
        let num_room = room2area_trg.len();
        let mut palette = vec![0xffffff, 0x000000];
        for i_room in 0..num_room {
            palette.push(room2color[i_room]);
        }
        del_canvas_core::canvas_gif::Canvas::new("target/2_shapeC.gif", (300, 300), &palette)
    };
    floorplan::optimize(
        &mut canvas_gif, vtxl2xy,
        site2xy, site2room, site2xy2flag,
        room2area_trg, room2color, room_connections)?;

    Ok(())
}
