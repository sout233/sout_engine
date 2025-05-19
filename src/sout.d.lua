-- sout.d.lua

---@class Sout
local sout = {}

---@return Scene
function sout.get_scene() end

---@return Player
function sout.new_player() end

---@param name "player"|"rect"
---@return Entity
function sout.new_entity(name) end

---@param key string
---@return boolean
function sout.key_pressed(key) end

---@param key string
---@return boolean
function sout.key_down(key) end

---@param ... string
function sout.print(...) end

---@param ... string
function sout.println(...) end

---@return number width, number height
function sout.viewport() end

---@alias Entity Player | Rectangle

--=== Scene ===--

---@class Scene
local scene = {}

---@param entity Entity
function scene:add_entity(entity) end

---@return integer
function scene:id() end

--=== Player ===--

---@class Player
local player = {}

---@return number
function player:x() end

---@param x number
function player:set_x(x) end

---@return number
function player:y() end

---@param y number
function player:set_y(y) end

---@return number
function player:speed() end

--=== Rectangle ===--

---@class Rectangle
local rect = {}

---@return number
function rect:x() end

---@param x number
function rect:set_x(x) end

---@return number
function rect:y() end

---@param y number
function rect:set_y(y) end

---@return number
function rect:width() end

---@param w number
function rect:set_width(w) end

---@return number
function rect:height() end

---@param h number
function rect:set_height(h) end

---@return string
function rect:shape() end

---@param shape string
function rect:set_shape(shape) end

---@param r number
---@param g number
---@param b number
function rect:set_color(r, g, b) end

return sout
