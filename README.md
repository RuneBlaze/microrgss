microrgss
===================

RGSS is the game engine for RPG Maker.

This is not a reimplementation of that engine,
but only something that resembles (a very limited) part of it.

It is mostly just for trying out Rust.

It runs the following snippet just fine:

```ruby
# microrgss
def load
    $s = Sprite.new('lbq_sound.png')
    b = Bitmap.new(200, 100);
    b.fill_rect(10, 10, 30, 30, Color.new(255,0, 0,255))
    $s.bitmap = b
end

def update
    if RGSS::Input.press? :RIGHT
        $s.x += 1
    elsif RGSS::Input.press? :LEFT
        $s.x -= 1
    end
end
```


## Running

```bash
cargo build
cargo run
```