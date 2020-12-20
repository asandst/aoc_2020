extern crate image;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use image::{ImageBuffer, Rgb, Pixel};
use image::GenericImage;

fn main() -> io::Result<()> {
    //let now = Instant::now();
    let input = File::open("input_day20")?;
    let input = BufReader::new(input).lines().map(|line| line.unwrap().to_string()).collect::<Vec<String>>();

    let mut current_id : usize = 0;
    let mut current_tile : Vec<Vec<u8>> = Vec::new();
    let mut tiles : HashMap<usize, Vec<Vec<u8>>> = HashMap::new();


    for (i, line) in input.iter().enumerate(){
        let i = i % 12;

        if i == 0 {
            current_id= line.replace("Tile ","").replace(":","").parse().unwrap();
        } else if i == 11{
            tiles.insert(current_id, current_tile);
            current_tile = Vec::new();
        } else {
            let mut row : Vec<u8>= Vec::new();
            for c in line.chars(){
                let num = match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!()
                };
                row.push(num);
            }
            current_tile.push(row);
        }
    }

    
    let dim = (tiles.len() as f64).sqrt() as usize;
    println!("dim {:?} {}", dim, tiles.len());
    let solver = Solver {tiles: &tiles, dim};
    let solution = solver.solve(&vec!{});

    println!("{:?}", solution);

    if solution.is_some() {
        let solution = solution.unwrap();
        println!("Part1 {:?}", solution[0].0 * solution[dim-1].0 * solution[solution.len()-dim].0 * solution[solution.len()-1].0);
        assert_eq!(solution[0].0 * solution[dim-1].0 * solution[solution.len()-dim].0 * solution[solution.len()-1].0, 107399567124539);


        let mut image : image::ImageBuffer<Rgb<u8>, std::vec::Vec<u8>>= ImageBuffer::new(8*dim as u32, 8*dim as u32);
        for (i,(id, rot, flipped)) in solution.iter().enumerate(){
            let tile = &tiles[&id];

            let mut tile_image : image::ImageBuffer<Rgb<u8>, std::vec::Vec<u8>>= ImageBuffer::new(8, 8);
            for x in 1..9 {
                for y in 1..9 {
                    let pixel = tile[y][x];
                    let rgb = image::Rgb{data: [100*pixel, 100*pixel, 100*pixel]};
                    tile_image.put_pixel((x-1) as u32, (y-1) as u32, rgb);
                }
            }

             tile_image = match rot{
                 Rotation::_0 => tile_image,
                 Rotation::_90 => image::imageops::rotate90(&tile_image),
                 Rotation::_180 => image::imageops::rotate180(&tile_image),
                 Rotation::_270 => image::imageops::rotate270(&tile_image)
             };
            if *flipped {
                tile_image = image::imageops::flip_horizontal(&tile_image);
            }

            image.copy_from(&tile_image, ((i%dim)*8) as u32, ((i/dim)*8) as u32);
        }

        image.save("test.png")?;

        for rot in [Rotation::_0, Rotation::_90, Rotation::_180, Rotation::_270].iter() {
            let mut image = image.clone();
            for &flipped in [false, true].iter() {
                image = match rot{
                    Rotation::_0 => image,
                    Rotation::_90 => image::imageops::rotate90(&image),
                    Rotation::_180 => image::imageops::rotate180(&image),
                    Rotation::_270 => image::imageops::rotate270(&image)
                };

                if flipped {
                    image = image::imageops::flip_horizontal(&image);
                }

                let mut monsters = false;

                for x in 0..(dim as u32*8-20){
                    for y in 0..(dim as u32*8-3){
                        let mut subimage = image::SubImage::new(&mut image, x, y, 20, 3);
                        let mut res = true;
                        res &= subimage.get_pixel(18,0)[0] == 100;
                        res &= subimage.get_pixel(0,1)[0] == 100;
                        res &= subimage.get_pixel(5,1)[0] == 100;
                        res &= subimage.get_pixel(6,1)[0] == 100;
                        res &= subimage.get_pixel(11,1)[0] == 100;
                        res &= subimage.get_pixel(12,1)[0] == 100;
                        res &= subimage.get_pixel(17,1)[0] == 100;
                        res &= subimage.get_pixel(18,1)[0] == 100;
                        res &= subimage.get_pixel(19,1)[0] == 100;
                        res &= subimage.get_pixel(1,2)[0] == 100;
                        res &= subimage.get_pixel(4,2)[0] == 100;
                        res &= subimage.get_pixel(7,2)[0] == 100;
                        res &= subimage.get_pixel(10,2)[0] == 100;
                        res &= subimage.get_pixel(13,2)[0] == 100;
                        res &= subimage.get_pixel(16,2)[0] == 100;
        
                        if res {
                            let rgb = image::Rgb{data: [200, 100, 100]};
                            subimage.put_pixel(18,0, rgb);
                            subimage.put_pixel(0,1, rgb);
                            subimage.put_pixel(5,1, rgb);
                            subimage.put_pixel(6,1, rgb);
                            subimage.put_pixel(11,1, rgb);
                            subimage.put_pixel(12,1, rgb);
                            subimage.put_pixel(17,1, rgb);
                            subimage.put_pixel(18,1, rgb);
                            subimage.put_pixel(19,1, rgb);
                            subimage.put_pixel(1,2, rgb);
                            subimage.put_pixel(4,2, rgb);
                            subimage.put_pixel(7,2, rgb);
                            subimage.put_pixel(10,2, rgb);
                            subimage.put_pixel(13,2, rgb);
                            subimage.put_pixel(16,2, rgb);
                            //println!("MONSTER");
                            monsters = true;
                        }
                    }
                }
                if monsters {
                    let mut count = 0;
                    for x in 0..image.width() {
                        for y in 0..image.height(){
                            if image.get_pixel(x,y)[0] == 100{
                                count += 1;
                            }
                        }
                    }
                    println!("Part2 {}", count);
                    image.save("test2.png")?;
                    assert_eq!(count, 1555);
                }
            }
        }

        
        
    }
    Ok(())
}

enum Result{
    Partial,
    Full,
    Fail
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Rotation{
    _0,
    _90,
    _180,
    _270,
}

struct Solver <'a>{
    tiles: &'a HashMap<usize, Vec<Vec<u8>>>,
    dim: usize
}

impl<'a> Solver<'a>{
    fn solve(&self, solution: &Vec<(usize, Rotation, bool)>) -> Option<Vec<(usize, Rotation, bool)>> {
        let all = self.tiles.keys().map(|&k| k).collect::<HashSet<usize>>();
        let used = solution.iter().map(|(k, _rot, _flipped)| *k).collect::<HashSet<usize>>();
        let left = all.difference(&used).map(|&k| k).map(|t| 
            vec!{
                (t, Rotation::_0, false), (t, Rotation::_90, false), (t, Rotation::_180, false), (t, Rotation::_270, false),
                (t, Rotation::_0, true), (t, Rotation::_90, true), (t, Rotation::_180, true), (t, Rotation::_270, true)
            })
            .flatten().collect::<HashSet<(usize, Rotation, bool)>>();

        left.iter().find_map(|l| {
            let mut new_solution = solution.clone();
            new_solution.push(l.clone());
            
            match self.check(&new_solution){
                Result::Full => Some(new_solution),
                Result::Partial => self.solve(&new_solution),
                Result::Fail => None
            }
        })
    }

    fn check(&self, solution: &Vec<(usize, Rotation, bool)>) -> Result {
        if solution.len() == 1{
            return Result::Partial;
        }
        let x = (solution.len()-1) % self.dim;
        let y = (solution.len()-1) / self.dim;
        if x != 0{
            let new_solution = &solution[solution.len()-1];
            let new_tile_edges = get_edges(&self.tiles[&new_solution.0], &new_solution.1);
            let old_solution = &solution[solution.len()-2];
            let old_tile_edges = get_edges(&self.tiles[&old_solution.0], &old_solution.1);
            
            let mut new_edge = new_tile_edges[&Rotation::_270].clone();
            let mut old_edge = old_tile_edges[&Rotation::_90].clone();
            let old_flipped = old_solution.2;
            if old_flipped {
                old_edge = old_tile_edges[&Rotation::_270].clone();
                old_edge.reverse();
            }

            let new_flipped = new_solution.2;
            if new_flipped {
                new_edge = new_tile_edges[&Rotation::_90].clone();
                new_edge.reverse();
            }
            new_edge.reverse();

            if old_edge == new_edge{
                if solution.len() == self.dim*self.dim{
                    return Result::Full;
                }else {
                    return Result::Partial;
                }
            } else {
                return Result::Fail;
            }
        }

        if y != 0{
            let new_solution = &solution[solution.len()-1];
            let new_tile_edges = get_edges(&self.tiles[&new_solution.0], &new_solution.1);
            let old_solution = &solution[solution.len()-(1+self.dim)];
            let old_tile_edges = get_edges(&self.tiles[&old_solution.0], &old_solution.1);

            let mut new_edge = new_tile_edges[&Rotation::_0].clone();
            let mut old_edge = old_tile_edges[&Rotation::_180].clone();
            let old_flipped = old_solution.2;
            if old_flipped {
                old_edge.reverse();
            }

            let new_flipped = new_solution.2;
            if new_flipped {
                new_edge.reverse();
            }
            new_edge.reverse();

            if old_edge == new_edge{
                if solution.len() == self.dim*self.dim{
                    return Result::Full;
                }else {
                    return Result::Partial;
                }
            } else {
                return Result::Fail;
            }
        }

        assert!(false);
        return Result::Fail;
    }
}

fn get_edges(tile: &Vec<Vec<u8>>, rotation: &Rotation) -> HashMap<Rotation, Vec<u8>>{
    let mut edges = HashMap::new();

    let top = tile[0].clone();
    let mut bottom = tile[9].clone();
    bottom.reverse();
    let mut col0 = Vec::new();
    let mut col9 = Vec::new();
    for i in 0..10{
        col0.push(tile[i][0]);
        col9.push(tile[i][9]);
    }
    col0.reverse();

    if *rotation == Rotation::_0{
        edges.insert(Rotation::_0, top);
        edges.insert(Rotation::_180, bottom);
        edges.insert(Rotation::_270, col0);
        edges.insert(Rotation::_90, col9);
    } else if *rotation == Rotation::_180{
        edges.insert(Rotation::_180, top);
        edges.insert(Rotation::_0, bottom);
        edges.insert(Rotation::_90, col0);
        edges.insert(Rotation::_270, col9);
    } else if *rotation == Rotation::_90 {
        edges.insert(Rotation::_90, top);
        edges.insert(Rotation::_270, bottom);
        edges.insert(Rotation::_0, col0);
        edges.insert(Rotation::_180, col9);
    } else if *rotation == Rotation::_270 {
        edges.insert(Rotation::_270, top);
        edges.insert(Rotation::_90, bottom);
        edges.insert(Rotation::_180, col0);
        edges.insert(Rotation::_0, col9);
    }
    edges
}