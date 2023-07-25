require("pebble_log")

Assets = { textures = {} }

---@class Array
---@field count number The number of element in the array 
---@field data any The data of the array
Array = {}

function Array:new()
    local o = { count = 0, data = {} }
    setmetatable(o, self)
    self.__index = self
    return o
end

---@generic T
---@param data `T`
function Array:push(data)
    self.data[self.count] = data
    self.count = self.count + 1
end

function Array:pop()
    self.data[self.count] = nil
    self.count = self.count - 1
end

---@generic T
---@param callback fun(t: `T`) : boolean
function Array:filter(callback)
    local copy = Array:new()
    for i = 0, self.count - 1 do
        if callback(self.data[i]) then
            copy:push(self.data[i])
        end
    end
    return copy.data
end

---@class Component
---@field c_type string
---@field data any
Component = {}

---@param type string
---@param data any
function Component:new(type, data)
    local o = { c_type = type, data = data }
    setmetatable(o, self)
    self.__index = self
    return o
end

---@return string
function Component:type()
    return ""
end

---@return any
function Component:default()
    error("Component need to be defined")
end

---@class Components : Array<Component>
Components = Array:new()

---@param data Component
function Components:add(data)
    Components:push(Component:new(data:type(), data:default()))
end

function Assets:add_file(filename)
    if filename:match("[.jpg][.png]") then
        self:add_image(filename)
    end
end

function Assets:add_image(filename)
    self["textures"][filename] = {
        file_type = "image",
        filename = filename
    }
end

---@class Vector
---@field x number
---@field y number
Vector = {}

---@param x number
---@param y number
---@return Vector
function Vector:new(x, y)
    local o = {x = x, y = y}
    setmetatable(o, self)
    self.__index = self
    return o
end

---@class Transform : Component
---@field position Vector
---@field scale Vector
---@field rotation number
Transform = {}

---@return Transform
function Transform:new()
    local o = { position = Vector:new(0.0, 0.0), rotation = 0.0, scale = Vector:new(1.0, 1.0) }
    setmetatable(o, self)
    self.__index = self
    return o
end

---comment
---@return Transform
function Transform:default()
    return Transform:new()
end

---comment
---@return string
function Transform:type()
    return "Transform"
end

---@class Color
---@field r number
---@field g number
---@field b number
---@field a number
Color = {r = 0.0, g = 0.0, b = 0.0, a = 0.0}

---@param r number
---@param g number
---@param b number
---@param a number
---@return Color
function Color:new(r, g, b, a)
    local o = {r = r, g = g, b = b, a = a}
    setmetatable(o, self)
    self.__index = self
    return o
end

---@class Material : Component
---@field albedo Color
---@field texture string|nil
Material = {}

function Material:new()
    local o = { albedo = Color:new(255, 255, 255, 255), texture = nil }
    setmetatable(o, self)
    self.__index = self
    return o
end

---@return Material
function Material:default()
    return Material:new()
end

---@return string
function Material:type()
    return "Material"
end