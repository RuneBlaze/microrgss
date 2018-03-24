extern crate image;
extern crate ggez;
#[macro_use]
extern crate mrusty;

use std::cmp::Ordering;
use std::cell::RefCell;
use mrusty::{Mruby, MrubyImpl};
use std::rc::Rc;
use std::collections::BinaryHeap;
use ggez::*;
use ggez::event::Mod;
use std::io::Read;
use ggez::graphics::{DrawMode, Point2, Image};
use ggez::graphics::Drawable;
use std::path::Path;
use ggez::event::Keycode;
use std::env;
use std::path;
use image::load_from_memory;
use std::convert::AsMut;
use std::cell::Ref;
use std::collections::HashMap;


struct MainState {
    pos_x: f32,
    // sprite: Sprite,
    input: Input,
    // graphics: Graphics,
    mruby: Rc<RefCell<Mruby>>,
}

struct Input {
    trigger: HashMap<Keycode, bool>,
    press: HashMap<Keycode, bool>,
}

impl Input {
    fn new() -> Input {
        Input{trigger: HashMap::new(), press: HashMap::new()}
    }

    fn trigger(&self, kc : Keycode) -> bool {
        *(self.trigger.get(&kc).unwrap_or(&false))
    }

    fn press(&self, kc : Keycode) -> bool {
        *(self.press.get(&kc).unwrap_or(&false))
    }
}



struct Graphics {
    sprites : Vec<Rc<RefCell<Sprite>>>
}

impl Graphics {
    fn new () -> Graphics {
        Graphics{sprites: Vec::new()}
    }

    fn reg (&mut self, sprite : Rc<RefCell<Sprite>>) {
        self.sprites.push(sprite);
    }

    fn update(&mut self, ctx : &mut Context) {
        for x in self.sprites.iter_mut() {
            x.borrow_mut().predraw(ctx)
        }
    }

    fn draw(&self, ctx: &mut Context) {
        for x in self.sprites.iter() {
            x.borrow_mut().draw(ctx)
        }
    }
}


struct Bitmap {
    raw: Vec<u8>,
    width: u32,
    height: u32,
    dirty: bool,
}

#[derive(Copy, Clone)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

fn packRgba8(r : u8, g : u8, b : u8, a : u8) -> Pixel {
    return Pixel{r: r, g : g, b : b , a : a};
}

impl Bitmap {
    fn injection(&self, x : u32, y : u32) -> u32 {
        return (x + y * self.width) * 4;
    }

    fn get_pixel(&self, x : u32, y : u32) -> Pixel {
        let i = self.injection(x, y) as usize;
        return packRgba8(self.raw[i],self.raw[i+1],self.raw[i+2],self.raw[i+3]);
    }

    fn set_pixel(&mut self, x : u32, y : u32, c : Pixel) {
        let i = self.injection(x, y) as usize;
        self.raw[i] = c.r;
        self.raw[i+1] = c.g;
        self.raw[i+2] = c.b;
        self.raw[i + 3] = c.a;
        self.dirty = true;
    }
}

impl Bitmap {
    fn new(width: u32, height: u32) -> Bitmap {
        Bitmap {raw: vec![0xff; (width * height * 4) as usize], width: width, height: height, dirty: true}
    }

    fn from_image(context : &mut Context, image : Image) -> Bitmap {
        Bitmap{ raw: image.to_rgba8(context).unwrap(), width: image.width(), height: image.height(), dirty: true}
    }

    fn from_path<P: AsRef<Path>>(path: P) -> Bitmap {
        // let mut buf = vec![];
        // let mut reader = context.filesystem.open(path).unwrap();
        // reader.read_to_end(&mut buf);
        // let mem = image::load_from_memory(&buf).unwrap();
        let np = Path::new("resources").join(path);
        let mem = image::open(np).unwrap();
        let img = mem.to_rgba();
        let (width, height) = img.dimensions();
        println!("width: {}, height: {}, len: {} <-> {}", width, height, mem.raw_pixels().len(), width * height * 4);
        Bitmap {  raw: img.into_raw(), width: width, height: height, dirty: true}
    }
}


struct Sprite {
    bitmap: Option<Rc<Bitmap>>,
    image: Option<Image>,
    pos : Point2,
    z : i32,
}

impl PartialOrd for Sprite {
    fn partial_cmp(&self, other: &Sprite) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Sprite {

}

impl PartialEq for Sprite {
    fn eq(&self, other: &Sprite) -> bool {
        self.pos == other.pos
    }
}

impl Ord for Sprite {
    fn cmp(&self, other: &Sprite) -> Ordering {
        self.z.cmp(&other.z)
    }
}




mrusty_class!(Pixel, "Color", {
    def!("initialize", |r: i32, g: i32, b: i32, a: i32| {
        Pixel{r:r as u8, g: g as u8, b: b as u8, a: a as u8}
    });

    def!("r", |mruby, slf: (&Pixel)| {
        mruby.fixnum(slf.r as i32)
    });

    def!("g", |mruby, slf: (&Pixel)| {
        mruby.fixnum(slf.g as i32)
    });

    def!("b", |mruby, slf: (&Pixel)| {
        mruby.fixnum(slf.b as i32)
    });

    def!("a", |mruby, slf: (&Pixel)| {
        mruby.fixnum(slf.a as i32)
    });
});

mrusty_class!(Bitmap, "Bitmap", {
    def!("initialize", |w: i32, h: i32| {
        Bitmap::new(w as u32, h as u32)
    });
});



mrusty_class!(Sprite, "Sprite", {
    def!("initialize", || {
        Sprite::new()
    });

    def!("x", |mruby, slf: (&Sprite)| {
        mruby.fixnum(slf.pos[0] as i32)
    });

    def!("y", |mruby, slf: (&Sprite)| {
        mruby.fixnum(slf.pos[1] as i32)
    });

    def!("x=", |mruby, slf: (&mut Sprite), x : i32| {
        slf.pos[0] = x as f32;
        mruby.fixnum(x)
    });

    def!("y=", |mruby, slf: (&mut Sprite), y : i32| {
        slf.pos[0] = y as f32;
        mruby.fixnum(y)
    });

    def!("z=", |mruby, slf: (&mut Sprite), z : i32| {
        slf.z = z;
        mruby.fixnum(z)
    });
});


mrusty_class!(Graphics, "Graphics", {
    def!("create", |mruby, slf: (&mut Graphics)| {
        let s = Rc::new(RefCell::new(Sprite::from_bitmap(Bitmap::from_path("lbq_sound.png"))));
        let v = &s;
        slf.reg(s);
        &s
    });
});




impl Sprite {
    fn new() -> Sprite {
        Sprite {bitmap: None, image : None, pos: Point2::new(0.0, 0.0), z: 0}
    }

    fn from_bitmap(bitmap : Bitmap) -> Sprite {
        let mut sp = Sprite::new();
        sp.bitmap = Some(Rc::new(bitmap));
        return sp;
    }

    fn predraw(&mut self, context: &mut Context) {
        if let Some(ref mut bitmap) = self.bitmap {
            if bitmap.dirty {
                let image = Image::from_rgba8(context, bitmap.width as u16, bitmap.height as u16, &bitmap.raw);
                self.image = Some(image.unwrap());
                Rc::get_mut(bitmap).unwrap().dirty = false;
            }
        }
    }

    fn set_x(&mut self, x : f32) {
        self.pos[0] = x
    }

    fn set_y(&mut self, y : f32) {
        self.pos[1] = y
    }

    fn width(&self) -> u32 {
        if let Some(ref bitmap) = self.bitmap {
            return bitmap.width;
        } else {
            panic!("")
        }
    }

    fn height(&self) -> u32 {
        if let Some(ref bitmap) = self.bitmap {
            return bitmap.height;
        } else {
            panic!("")
        }
    }

    fn draw(&self, ctx : &mut Context) {
        if let Some(ref image) = self.image {
            image.draw(ctx, self.pos, 0.0).unwrap();
        } else {
            panic!("")
        }
    }
}

impl MainState  {
    fn new(_ctx: &mut Context, mruby: Rc<RefCell<Mruby>>) -> GameResult<MainState> {
        let rgss = mruby.get_module("RGSS").unwrap();

        // mruby.def_class_for::<Graphics>("Graphics");
        assert!(mruby.is_defined("Graphics"));
        let mut graphics = Graphics::new();
        rgss.def_const("Graphics", mruby.obj(graphics));
        mruby.run("load()").unwrap();
        // let s = Sprite::from_bitmap(Bitmap::from_path("lbq_sound.png"));
        // graphics.reg(s);
        let s = MainState { pos_x: 0.0, input : Input::new(), mruby: mruby};
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.mruby.run("update()").unwrap();
        self.pos_x = self.pos_x % 800.0 + 1.0;
        // if self.input.trigger(Keycode::Right) {
        //     self.sprite.pos[0] += 50.0;
        // } else {
        // }

        let graphics = self.mruby.run("RGSS::Graphics").unwrap().to_obj::<Graphics>().unwrap();

        graphics.borrow_mut().update(_ctx);


        self.input.trigger = HashMap::new();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let graphics = self.mruby.run("RGSS::Graphics").unwrap().to_obj::<Graphics>().unwrap();

        graphics.borrow_mut().draw(ctx);
        // self.sprite.predraw(ctx);
        // self.graphics.draw(ctx);
        // graphics::circle(ctx,
        //                  DrawMode::Fill,
        //                  Point2::new(self.pos_x, 380.0),
        //                  100.0,
        //                  2.0)?;
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _: Mod, _: bool) {
        self.input.press.insert(keycode, true);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _: Mod, _: bool) {
        self.input.trigger.insert(keycode, true);
        self.input.press.insert(keycode, false);
    }
}




pub fn main() {
    let mruby = Mruby::new();
    mruby.def_file::<Pixel>("color");
    mruby.def_file::<Sprite>("sprite");
    mruby.def_file::<Graphics>("graphics");

    mruby.def_method_for::<Graphics, _>("create", mrfn!(|mruby, slf: Value, ))
    mruby.def_module("RGSS");
    let rgss = mruby.get_module("RGSS").unwrap();


    let result = mruby.run("
        require 'color'
        require 'sprite'
        require 'graphics'

        Color.new(255,255,255,140).a

        def load
            s = Sprite.new
            RGSS::Graphics.reg(s)
        end

        def update
        end
    ").unwrap();

    // println!("mrb: {}", result.to_i32().unwrap());

    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }
    let state = &mut MainState::new(ctx, mruby).unwrap();
    event::run(ctx, state).unwrap();
}

