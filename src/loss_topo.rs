use del_candle::voronoi2::VoronoiInfo;

fn topology(
    voronoi_info: &VoronoiInfo,
    num_room: usize,
    site2room: &[usize])
    -> (usize, Vec<usize>, Vec<Vec<usize>>)
{
    let (num_group, site2group) = {
        // return j_site if it is a same room
        let siteface2adjsitesameroom = |i_site, i_face| {
            let i_room = site2room[i_site];
            if i_room == usize::MAX {
                return usize::MAX;
            }
            let j_site = voronoi_info.idx2site[voronoi_info.site2idx[i_site] + i_face];
            if j_site == usize::MAX {
                return usize::MAX;
            }
            let j_room = site2room[j_site];
            assert_ne!(j_room, usize::MAX);
            if i_room != j_room {
                return usize::MAX;
            }
            return j_site;
        };
        del_msh_core::elem2group::from_polygon_mesh(&voronoi_info.site2idx, siteface2adjsitesameroom)
    };
    assert_eq!(site2group.len(), site2room.len());
    //
    let room2group = {
        let mut room2group = vec![std::collections::BTreeSet::<usize>::new(); num_room];
        for i_site in 0..site2room.len() {
            let i_room = site2room[i_site];
            if i_room == usize::MAX {
                continue;
            }
            let i_group = site2group[i_site];
            room2group[i_room].insert(i_group);
        }
        room2group
    };
    let room2group: Vec<Vec<usize>> = room2group.iter().map(|v| v.iter().cloned().collect() ).collect();
    (num_group, site2group, room2group)
}

pub fn inverse_map(
    num_group: usize,
    site2group: &[usize]) -> Vec<Vec<usize>>{
    let mut group2site = vec![std::collections::BTreeSet::<usize>::new(); num_group];
    for i_site in 0..site2group.len() {
        let i_group = site2group[i_site];
        if i_group == usize::MAX {
            continue;
        }
        group2site[i_group].insert(i_site);
    }
    group2site.iter().map(|v| v.iter().cloned().collect() ).collect()
}

/*
pub fn room2site(
    num_room: usize,
    site2room: &[usize]) -> Vec<std::collections::BTreeSet<usize>>
{
    let mut room2site = vec![std::collections::BTreeSet::<usize>::new(); num_room];
    for i_site in 0..site2room.len() {
        let i_room = site2room[i_site];
        if i_room == usize::MAX {
            continue;
        }
        room2site[i_room].insert(i_site);
    }
    room2site
}
 */

fn is_two_room_connected(
    i0_room: usize,
    i1_room: usize,
    site2room: &[usize],
    room2site: &Vec<Vec<usize>>,
    voronoi_info: &VoronoiInfo,) -> bool
{
    let mut is_connected = false;
    for &i_site in room2site[i0_room].iter() {
        for &j_site in &voronoi_info.idx2site[voronoi_info.site2idx[i_site]..voronoi_info.site2idx[i_site+1]] {
            if j_site == usize::MAX { continue; }
            if site2room[j_site] != i1_room { continue; }
            is_connected = true;
            break;
        }
        if is_connected { break; }
    }
    is_connected
}

fn find_nearest_site(
    i0_room: usize,
    i1_room: usize,
    room2site: &Vec<Vec<usize>>,
    site2xy: &[f32]) -> (usize, usize)
{
    let mut pair = (0usize, 0usize);
    let mut min_dist = f32::INFINITY;
    for &i_site in room2site[i0_room].iter() {
        let pi = del_msh_core::vtx2xy::to_navec2(site2xy, i_site);
        for &j_site in room2site[i1_room].iter() {
            let pj = del_msh_core::vtx2xy::to_navec2(site2xy, j_site);
            let dist = (pi-pj).norm();
            if dist < min_dist {
                min_dist = dist;
                pair = (i_site, j_site);
            }
        }
    }
    pair
}

pub fn unidirectional(
    site2xy: &candle_core::Tensor,
    site2room: &[usize],
    num_room: usize,
    voronoi_info: &VoronoiInfo,
    room_connections: &Vec<(usize, usize)>)
-> candle_core::Result<candle_core::Tensor>
{
    // dbg!("{}", &room_connections);
    let num_site = site2xy.dims2()?.0;
    let (num_group, site2group, room2group)
        = topology(voronoi_info, num_room, site2room);
    let room2site = inverse_map(num_room, site2room);
    let group2site = inverse_map(num_group, &site2group);
    let site2xy0 = site2xy.flatten_all()?.to_vec1::<f32>()?;
    assert_eq!(site2xy0.len(), num_site*2);
    let mut site2xytrg = site2xy0.clone();
    for i_room in 0..num_room {
        assert!( !room2group[i_room].is_empty() );
        if room2group[i_room].len() == 1 { // the room is in one piece
            let rooms_to_connect: Vec<usize> = {
                let mut rooms_to_connect = vec!();
                for &(i0_room, i1_room) in room_connections.iter() {
                    if i0_room == i_room { rooms_to_connect.push(i1_room); }
                    else if i1_room == i_room { rooms_to_connect.push(i0_room); }
                }
                rooms_to_connect
            };
            for &j_room in rooms_to_connect.iter() {
                let is_connected = is_two_room_connected(
                    i_room, j_room, site2room, &room2site, voronoi_info);
                if is_connected { continue; }
                // println!("{} {}", i_room, j_room);
                let (i_site, j_site)
                    = find_nearest_site(i_room, j_room, &room2site, &site2xy0);
                site2xytrg[i_site*2+0] = site2xy0[j_site*2+0];
                site2xytrg[i_site*2+1] = site2xy0[j_site*2+1];
            }
        }
        else {
            // the room is split
            let i_group = { // group to attract other groups
                let mut i_group = usize::MAX;
                for &j_group in room2group[i_room].iter() {
                    for &j_site in &group2site[j_group] { // this site has cell
                        if voronoi_info.site2idx[j_site+1] > voronoi_info.site2idx[j_site] {
                            i_group = j_group;
                            break;
                        }
                    }
                    if i_group != usize::MAX { break; }
                }
                i_group
            };
            if i_group == usize::MAX { // no cell for this room
                for ij_group in 0..room2group[i_room].len() {
                    let j_group = room2group[i_room][ij_group];
                    for &j_site in &group2site[j_group] {
                        site2xytrg[j_site * 2 + 0] = 0.5;
                        site2xytrg[j_site * 2 + 1] = 0.5;
                    }
                }
                continue;
            }
            // assert!(i_group!=usize::MAX);
            for ij_group in 0..room2group[i_room].len() {
                let j_group = room2group[i_room][ij_group];
                if i_group == j_group { continue; }
                for &j_site in &group2site[j_group] {
                    let pj = del_msh_core::vtx2xy::to_navec2(&site2xy0, j_site);
                    let mut dist_min = f32::INFINITY;
                    let mut pi_min = nalgebra::Vector2::<f32>::new(0., 0.);
                    for &i_site in &group2site[i_group] {
                        assert_ne!(i_site, j_site);
                        let pi = del_msh_core::vtx2xy::to_navec2(&site2xy0, i_site);
                        let dist = (pj - pi).norm();
                        if dist < dist_min {
                            pi_min = pi;
                            dist_min = dist;
                        }
                    }
                    site2xytrg[j_site * 2 + 0] = pi_min[0];
                    site2xytrg[j_site * 2 + 1] = pi_min[1];
                }
            }
        }
    }
    let site2xytrg = candle_core::Tensor::from_vec(
        site2xytrg,
        candle_core::Shape::from((num_site, 2)),
        &candle_core::Device::Cpu,
    )?;
    (site2xy-site2xytrg).unwrap().sqr()?.sum_all()
}

pub fn kmean_style(
    site2xy: &candle_core::Tensor,
    site2room: &[usize],
    num_room: usize,
    voronoi_info: &VoronoiInfo,
    rooom_connections: &Vec<(usize, usize)>
) -> candle_core::Result<candle_core::Tensor> {
    let (num_group, site2group, room2group)
        = topology(voronoi_info, num_room, site2room);
    let room2site = inverse_map(num_room, site2room);
    let group2site = inverse_map(num_group, &site2group);
    let edge2roomgroup = {
        let mut edge2roomgroup = vec![(0usize, false); 0];
        // edge for divided room
        for i_room in 0..num_room {
            if room2group[i_room].len() <= 1 {
                continue;
            }
            for i_group in &room2group[i_room] {
                edge2roomgroup.push((i_room, true));
                edge2roomgroup.push((*i_group, false));
            }
        }
        // edge for missing room connection
        for &(i0_room, i1_room) in rooom_connections {
            let is_connected = is_two_room_connected(
                i0_room, i1_room, site2room, &room2site, voronoi_info);
            if is_connected { continue; }
            edge2roomgroup.push((i0_room, true));
            edge2roomgroup.push((i1_room, true));
        }
        edge2roomgroup
    };
    let num_site = site2room.len();
    let num_edge = edge2roomgroup.len() / 2;
    let mut edge2site = vec![0f32; num_edge * num_site];
    for i_edge in 0..num_edge {
        for i_node in 0..2 {
            let sign: f32 = if i_node == 0 { 1f32 } else { -1f32 };
            let (irg, is_room) = edge2roomgroup[i_edge * 2 + i_node];
            if is_room {
                let i_room = irg;
                let w0 = 1f32 / room2site[i_room].len() as f32;
                for i_site in &room2site[i_room] {
                    edge2site[i_edge * num_site + i_site] += w0 * sign;
                }
            } else {
                let i_group = irg;
                let w0 = 1f32 / group2site[i_group].len() as f32;
                for i_site in &group2site[i_group] {
                    edge2site[i_edge * num_site + i_site] += w0 * sign;
                }
            }
        }
    }
    let edge2site = candle_core::Tensor::from_vec(
        edge2site,
        candle_core::Shape::from((num_edge, num_site)),
        &candle_core::Device::Cpu,
    )?;
    let edge2xy = edge2site.matmul(&site2xy)?;
    println!("    {:?}", edge2xy.shape().dims2()?);
    for i_room in 0..num_room {
        println!(
            "    room {} -> {:?}, {:?}",
            i_room, room2group[i_room], room2site[i_room]
        );
    }
    edge2xy.sqr()?.sum_all()
}
