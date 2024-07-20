/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::render::{Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::WindowContext;
use std::fs;
use std::path::Path;
use rand::{self,thread_rng,seq::SliceRandom};

use image::*;

const SCREEN_SIZE:u32 = 800;
const TILE_SIZE:u32 = 40;// ! update TILE_SIZE and TILE_PER_AXES together
const TILE_PER_AXE:u32 = 20;//TILE_PER_AXE*TILESIZE == SCREEN_SIZE
const TILE_IMG_SIZE:u32 = 14;
const TILE_NONE:usize = usize::DEFAULT_MAX_VALUE;


pub struct Vec2{
    x:u32,
    y:u32,
}
#[derive(Clone, Copy)]
pub struct Tileconnectors{
    pub top:usize,
    pub bottom:usize,
    pub right:usize,
    pub left:usize,
}
impl Tileconnectors{
    fn new() -> Tileconnectors{
        Tileconnectors{
            top:0,
            bottom:0,
            right:0,
            left:0,
        }
    }
}
pub struct Wfc<'a>{
    pub tiles_data:Vec<image::DynamicImage>,
    pub tiles_draw:Vec<Texture<'a>>,
    pub connectors_tiles:Vec<Tileconnectors>,
    pub connector_table:Vec<Vec<u32>>,
    pub tiles_grid:[[usize;TILE_PER_AXE as usize];TILE_PER_AXE as usize],
    pub connection_count:[[usize;TILE_PER_AXE as usize];TILE_PER_AXE as usize],
    pub min_connection:usize,
    pub min_connections_cache:Vec<Vec2>,
}


impl<'b> Wfc<'b>{
    fn new<'a>()->Wfc<'a>{
        Wfc{
            tiles_data:vec![],
            tiles_draw:vec![],
            connectors_tiles:vec![],
            connector_table:vec![],
            tiles_grid:[[TILE_NONE;TILE_PER_AXE as usize];TILE_PER_AXE as usize],
            connection_count:[[TILE_NONE;TILE_PER_AXE as usize];TILE_PER_AXE as usize],
            min_connection:0,
            min_connections_cache:Vec::new(),
        }
    }

    fn get_connector_table_pointer(&self,connector:&Vec<u32>)->usize{
        //if connector is in connector table:
        let mut connector_table_pointer:usize = 0;
        for existing_connector in self.connector_table.iter(){
            if *existing_connector == *connector{    
                return connector_table_pointer;
            }
            connector_table_pointer+=1;
        }
        //else:
        return connector_table_pointer;
    }
    fn calculate_conectors(&mut self){
        
        let get_color_code = |color:[u8;4]| {
            color[0] as u32+
            ((color[1] as u32)<<8)+
            ((color[2] as u32)<<16)+
            ((color[2] as u32)<<24)
        };
        for tile in self.tiles_data.iter(){
            let mut tileconectors = Tileconnectors::new();
            //top
            let mut connector_top:Vec<u32> = vec![];
            for j in 0..TILE_IMG_SIZE{
                connector_top.push(get_color_code(tile.get_pixel(j,0).0));
            }
            tileconectors.top = self.get_connector_table_pointer(&connector_top); 
            if tileconectors.top >= self.connector_table.len(){
                self.connector_table.push(connector_top);
            }

            //right
            let mut connector_right:Vec<u32> = vec![];
            for j in 0..TILE_IMG_SIZE{
                connector_right.push(get_color_code(tile.get_pixel(TILE_IMG_SIZE-1,j).0));
            }
            tileconectors.right = self.get_connector_table_pointer(&connector_right);
            if tileconectors.right >= self.connector_table.len(){
                self.connector_table.push(connector_right);
            }
            //bottom
            let mut connector_bottom:Vec<u32> = vec![];
            for j in 0..TILE_IMG_SIZE{
                connector_bottom.push(get_color_code(tile.get_pixel(j,TILE_IMG_SIZE-1).0));
            }
            tileconectors.bottom = self.get_connector_table_pointer(&connector_bottom);
            if tileconectors.bottom >= self.connector_table.len(){
                self.connector_table.push(connector_bottom);
            }
            //left
            let mut connector_left:Vec<u32> = vec![];
            for j in 0..TILE_IMG_SIZE{
                connector_left.push(get_color_code(tile.get_pixel(0,j).0));
            }
            tileconectors.left = self.get_connector_table_pointer(&connector_left);
            if tileconectors.left >= self.connector_table.len(){
                self.connector_table.push(connector_left);
            }
            self.connectors_tiles.push(tileconectors);
        }


    }

    fn init<'a>(&mut self,texture_creator:&'b TextureCreator<WindowContext>)->Result<(),String>{
        for file in fs::read_dir("rsc/circuit2").unwrap() {
            let path = file.unwrap().path();
            
            let surface = Surface::load_bmp(path.clone()).map_err(|e| e.to_string())?;
            self.tiles_draw.push(Texture::from_surface(&surface, texture_creator).map_err(|e| e.to_string())?);
            self.tiles_data.push(image::open(&Path::new(&path)).unwrap());
        }
        //connector table Work in Progress
        self.calculate_conectors();
        Ok(())
    }
    fn tile_matches(&mut self,new_tile:usize,x:u32,y:u32)->bool{
    //if the top connector is equal to the butoom connector on top of the tile
        
        if y != 0{
            let index_top = self.tiles_grid[x as usize][(y-1) as usize];
            if index_top !=TILE_NONE{//if tile_right != None: compare_pages() 
                if self.connectors_tiles[new_tile].top != self.connectors_tiles[index_top].bottom{
                    return false;
                }
            }
        }
     
       
        //if the left connector is equal to the right connector left to the current tile
        if x != 0{
            let index_left = self.tiles_grid[(x-1) as usize][y as usize];
            if index_left != TILE_NONE{
                if self.connectors_tiles[new_tile].left != self.connectors_tiles[index_left].right{
                    return false;
                }
            }                
        }

        
        //if the buttom connector is equal to the top connector below the current tile
        if y < TILE_PER_AXE-1{
            let index_bottom = self.tiles_grid[x as usize][(y+1) as usize];
            if index_bottom !=TILE_NONE{//if tile_right != None: compare_pages()
                if self.connectors_tiles[new_tile].bottom!=self.connectors_tiles[index_bottom].top{
                    return false;
                }
            }
        }
        
        //if the right connector is equal to the left connector right to the current tile
        if x < TILE_PER_AXE-1{
            let index_right = self.tiles_grid[(x+1) as usize][y as usize];
            if index_right !=TILE_NONE{
                if self.connectors_tiles[new_tile].right!=self.connectors_tiles[index_right].left{
                    return false;
                }
            }
        }
        true

    }
    fn get_connection_count(&mut self){
        self.min_connection = TILE_NONE;
        for x in 0..(TILE_PER_AXE as usize){
            for y in 0..(TILE_PER_AXE as usize){
                if self.tiles_grid[x][y] != TILE_NONE{
                    self.connection_count[x][y] = TILE_NONE;
                }else{
                    self.connection_count[x][y] = 0;
                    for tile in 0..self.connectors_tiles.len(){
                        if self.tile_matches(tile,x as u32,y as u32){
                            self.connection_count[x][y] += 1
                        }
                    }
                    if self.connection_count[x][y]<self.min_connection&&self.connection_count[x][y]!=0{
                        self.min_connection=self.connection_count[x][y];
                        self.min_connections_cache.clear();
                    }
                    if self.connection_count[x][y]==self.min_connection{
                        self.min_connections_cache.push(Vec2{x:x as u32,y:y as u32});
                    }
                }
            }
        }
    }
    fn draw_sceene(&self,canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
        canvas.clear();
        for x in 0..TILE_PER_AXE{
            for y in 0..TILE_PER_AXE{
                    let tile_id = self.tiles_grid[x as usize][y as usize];
                    if tile_id != TILE_NONE{
                        canvas.copy(
                            &self.tiles_draw[tile_id],
                            None,
                            Rect::new((x*TILE_SIZE) as i32,(y*TILE_SIZE) as i32,TILE_SIZE,TILE_SIZE),
                        ).unwrap();
                    }
            }
        }
        canvas.present();
    }
    fn add_tile(&mut self){
            '_add_tile: loop{
                self.min_connections_cache.shuffle(&mut thread_rng());
                let x = self.min_connections_cache.last().unwrap().x;
                let y = self.min_connections_cache.last().unwrap().y;
                if self.connection_count[x as usize][y as usize]!=self.min_connection{
                    return;
                }
                let mut range:Vec<usize> = (0..(self.connectors_tiles.len())).collect();
                range.shuffle(&mut thread_rng());
                for i in range.iter(){
                    if self.tile_matches(*i, x, y){
                        self.tiles_grid[x as usize][y as usize] = *i; 
                        return;
                    }
                }
                self.min_connections_cache.pop();
            }
       
    }
    fn main(&mut self,canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
        while self.min_connection!=TILE_NONE{
            self.get_connection_count();
            self.add_tile();
            self.draw_sceene(canvas);
        }
    }
}

fn main() -> Result<(), String> /*Error Handling*/{
    //inititlizing SDL
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rusty WaveFunctionCollaps", SCREEN_SIZE, SCREEN_SIZE)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 255, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?; 
    let texture_creator = canvas.texture_creator();
    //loads texture by name into the hashmap
    //use it by borrowing &img["texture name"]
    let mut wfc:Wfc = Wfc::new();
    wfc.init(&texture_creator)?;
    //init random
    
    //draw stuff
    canvas.set_draw_color(Color::RGB(0,255,0));
    canvas.clear();
    wfc.main(&mut canvas); 
    println!("done!");
    //debuging:   
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
               _ => {}
            }

        }
 
    }
    
    Ok(())
}
