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