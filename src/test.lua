---@type Sout
sout = sout

local velocity = 0
local jump_velocity = -444
local gravity = 888

local scene = sout.get_scene()
local player = sout.new_player()

local pipes = {}
local pipe_spawn_timer = 0
local pipe_interval = 2.0 -- 1pipe 每 2s

function setup()
    sout.println("Lua loaded")

    scene:add_entity(player)

    local w, h = sout.viewport()
    player:set_x(w / 4)
    player:set_y(h / 2)
end

function spawn_pipe()
    local w, h = sout.viewport()
    local pipe_width = 50
    local gap_height = 150
    local gap_y = math.random(100, h - 100 - gap_height)

    local top_pipe = sout.new_entity("rect")
    top_pipe:set_x(w)
    top_pipe:set_y(0)
    top_pipe:set_size(pipe_width, gap_y)
    top_pipe:set_color(0, 255, 0)
    scene:add_entity(top_pipe)

    local bottom_pipe = sout.new_entity("rect")
    bottom_pipe:set_x(w)
    bottom_pipe:set_y(gap_y + gap_height)
    bottom_pipe:set_size(pipe_width, h - gap_y - gap_height)
    bottom_pipe:set_color(0, 255, 0)
    scene:add_entity(bottom_pipe)

    table.insert(pipes, top_pipe)
    table.insert(pipes, bottom_pipe)
end

function process(delta)
    local w, h = sout.viewport()

    velocity = velocity + gravity * delta
    player:set_y(player:y() + velocity * delta)

    if sout.key_pressed("space") then
        velocity = jump_velocity
    end

    if player:y() > h or player:y() < 0 then
        sout.println("Game Over")
        velocity = 0
        player:set_y(h / 2)
    end

    for i, pipe in ipairs(pipes) do
        pipe:set_x(pipe:x() - 200 * delta)
    end

    pipe_spawn_timer = pipe_spawn_timer + delta
    if pipe_spawn_timer >= pipe_interval then
        spawn_pipe()
        pipe_spawn_timer = 0
    end
end
