Assets = { files = {} }

function Assets:add_file(filename)
    if filename:match("[.jpg][.png]") then
        self:add_image(filename)
    end
end

function Assets:add_image(filename)
    self["files"][filename] = {
        file_type = "image",
        filename = filename
    }
end

Vector = {x  = 0, y = 0}

function Vector:new(x, y, o)
    o = o or {x = x, y = y}
    setmetatable(o, self)
    self.__index = self
    return o
end

Transform = { position = Vector:new(0.0, 0.0), rotation = 0.0, scale = Vector:new(1.0, 1.0) }

function Transform:new(o)
    o = o or {position = Vector:new(0.0, 0.0), rotation = 0.0, scale = Vector:new(1.0, 1.0) }
    setmetatable(o, self)
    self.__index = self
    return o
end

Color = {r = 0.0, g = 0.0, b = 0.0, a = 0.0}

function Color:new(r, g, b, a, o)
    o = o or {r = r, g = g, b = b, a = a}
    setmetatable(o, self)
    self.__index = self
    return o
end

Material = { albedo = Color:new(200, 200, 200, 255), texture = nil }

function Material:new(o)
    o = o or { albedo = Color:new() }
    setmetatable(o, self)
    self.__index = self
    return o
end