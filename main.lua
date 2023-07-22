-- @types:begin

-- Vector2 = {
--     x: number,
--     y: number
-- }

-- @types:end

-- vector2 = stack:add_component("Vector2")
require("pebble_core")

Material.albedo = Color:new(255, 128, 0, 255);
Material.texture = "pebble.png"

Transform.position = Vector:new(400, 400);
Transform.scale = Vector:new(10,10);

Assets:add_file("pebble.png")

local accumulator = 0

function Update(delta)
    accumulator = accumulator + delta * 0.5
    local x = math.abs(math.cos(accumulator))
    local y = math.abs(math.sin(accumulator))
    local z = math.abs(math.cos(accumulator) * math.sin(accumulator)) * 2

    local r = x * 255
    local g = y * 255
    local b = z * 255

    Material.albedo = Color:new(r, b, g, 255);

    Transform.scale.x = (1 * z * 10) + 1
    Transform.scale.y = (1 * y * 10) + 1
    Transform.rotation = Transform.rotation + 0.01 * delta
end