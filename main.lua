-- @types:begin

-- Vector2 = {
--     x: number,
--     y: number
-- }

-- @types:end

-- vector2 = stack:add_component("Vector2")
require("pebble_core")

Components:add(Material)
Components:add(Transform)

Assets:add_file("pebble.png")

Log:info("blablabla")

---@type Material
local material = Components:filter( 
    ---@param t Component
    function (t) return t.c_type == "Material"; end
)[0].data

---@type Transform
local transform = Components:filter(
    ---@param t Component
    function (t)
        return t.c_type == "Transform";
    end
)[0].data

material.albedo = Color:new(255,0,0,255)
material.texture = "pebble.png"

---@type number
local accumulator = 0

---@type number
local chrono = 0

---comment
---@param start_value number
---@param end_value number
---@param t number
---@return number
function Lerp(start_value, end_value, t)
    return start_value + (end_value  - start_value) * t
end


transform.position = Vector:new(400,400)

local ancient_pos = transform.position

local random_x = 0
local random_y = 0

local function randomize_pos()
    random_x = math.random() * 300 + 200
    random_y = math.random() * 300 + 200
end

local function change_pos()
    randomize_pos()
    ancient_pos = transform.position
    chrono = 0
end

change_pos()

function Update(dt)
    accumulator = accumulator + dt * 0.5
    chrono = chrono + dt

    local x = math.abs(math.cos(accumulator))
    local y = math.abs(math.sin(accumulator))
    local z = math.abs(math.cos(accumulator) * math.sin(accumulator)) * 2

    local r = x * 255
    local g = y * 255
    local b = z * 255

    material.albedo = Color:new(r, b, g, 255);

    transform.scale.x = (1 * z * 10) + 1
    transform.scale.y = (1 * y * 10) + 1
    transform.rotation = transform.rotation + 0.5 * dt

    transform.position.x = Lerp(ancient_pos.x, random_x, chrono)
    transform.position.y = Lerp(ancient_pos.y, random_y, chrono)

    if chrono >= 1 then
        change_pos()
    end
end

-- Material.albedo = Color:new(255, 128, 0, 255);
-- Material.texture = "pebble.png"

-- Transform.position = Vector:new(400, 400);
-- Transform.scale = Vector:new(10,10);